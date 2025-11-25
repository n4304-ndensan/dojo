
_C# / ASP.NET Core / Blazor_

---

## 1. 目的

このドキュメントは、C# における DI の基礎から、ASP.NET Core と Blazor での実践までを、上流（設計方針）から下流（実装・テスト・運用）へ流れで整理します。ライフタイム（Transient / Scoped / Singleton）の使い分け、コンストラクタ注入の原則、Blazor 特有の注意点を含みます。

---

## 2. 用語と前提

- **コンテナ**: `Microsoft.Extensions.DependencyInjection` によるサービス登録・解決の仕組み
    
- **登録**: `IServiceCollection` に型やファクトリを紐づける（例: `AddScoped<IRepo, Repo>()`）
    
- **解決（Resolve）**: コンストラクタや属性などを通じて、必要な型のインスタンスを受け取る
    
- **ホスト**: `Generic Host`（`Host.CreateDefaultBuilder` / `WebApplication.CreateBuilder`）が DI と構成、ログを提供
    

---

## 3. ライフタイムの基本と使い分け

|種別|生成単位|典型用途|主な注意点|
|---|---|---|---|
|**Transient**|解決ごとに新規|軽量で状態を持たない処理（バリデーション、フォーマッタ）|多用すると割り当て増。複数注入で別インスタンスになる|
|**Scoped**|リクエスト単位（ASP.NET Core） / 接続単位（Blazor Server）|`DbContext`、ユーザーセッション、ユースケース単位のサービス|Singleton から保持しない。Dispose タイミングはスコープ終了時|
|**Singleton**|アプリ全体で 1 個|静的設定、スレッドセーフなキャッシュ、変化の少ない提供元|可変状態を持たない。スレッドセーフ前提。Scoped をキャプチャしない|

**Blazor の補足**

- **Blazor Server**: Scoped は **SignalR 接続単位**（ユーザーごと）。セッション的なサービスに適合
    
- **Blazor WebAssembly**: プロセスが単一のため、Scoped は実質 Singleton に近い。状態の扱いに注意
    

---

## 4. 登録と解決の基本パターン

### 4.1 Program.cs（登録）

```csharp
var builder = WebApplication.CreateBuilder(args);

// 例: ライフタイム登録
builder.Services.AddTransient<IEmailSender, SmtpEmailSender>();
builder.Services.AddScoped<IOrderService, OrderService>();
builder.Services.AddSingleton<IAppConfig>(sp => new AppConfig(builder.Configuration));

// Web（MVC/Minimal API/Blazor）
builder.Services.AddRazorPages();
builder.Services.AddRazorComponents(); // Blazor
// .AddInteractiveServerComponents(); // Server
// .AddInteractiveWebAssemblyComponents(); // WASM

var app = builder.Build();
// 省略
```

### 4.2 コンストラクタ注入（原則・推奨）

```csharp
public class OrderController : ControllerBase
{
    private readonly IOrderService _orders;
    private readonly ILogger<OrderController> _log;

    public OrderController(IOrderService orders, ILogger<OrderController> log)
    {
        _orders = orders;
        _log = log;
    }
}
```

**利点**: 不変性（`readonly`）、型安全、依存不足はコンパイルで検出、テスト容易

### 4.3 プロパティ注入（Blazor コンポーネント限定）

```razor
@page "/sample"
@code {
    [Inject] public IClock Clock { get; set; } = default!;
    protected override void OnInitialized() { var now = Clock.Now; }
}
```

> 通常のサービスクラスでは `[Inject]` は効かない。Blazor コンポーネント（`.razor` / `.razor.cs` の partial `ComponentBase`）に限定

### 4.4 Minimal API のパラメータ注入

```csharp
app.MapGet("/health", (ILoggerFactory lf) =>
{
    var logger = lf.CreateLogger("Health");
    logger.LogInformation("ok");
    return Results.Ok();
});
```

---

## 5. Blazor での DI

### 5.1 .razor（インライン）

```razor
@page "/time"
@inject IClock Clock
<h3>@Clock.Now</h3>
```

### 5.2 .razor.cs（コードビハインド）

```csharp
public partial class Time : ComponentBase
{
    [Inject] protected IClock Clock { get; set; } = default!;

    protected override void OnInitialized()
    {
        _now = Clock.Now; // ライフサイクル内で使用
    }

    private DateTime _now;
}
```

- `.razor` と同名の `partial class` + `ComponentBase` なら `[Inject]` が有効
    
- 初期化子やコンストラクタで `[Inject]` プロパティを参照しない（未設定の可能性がある）
    

### 5.3 セッション的サービスは Scoped

```csharp
builder.Services.AddScoped<ISessionService, SessionService>();
```

Singleton にすると全ユーザーで共有されるため不適切。

### 5.4 OwningComponentBase（コンポーネント専用スコープ）

```csharp
public class MyScoped : IDisposable { public void Dispose(){} }

builder.Services.AddScoped<MyScoped>();

public partial class PageA : OwningComponentBase
{
    protected override void OnInitialized()
    {
        var svc = ScopedServices.GetRequiredService<MyScoped>(); // 画面の寿命でDispose
    }
}
```

---

## 6. ASP.NET Core（MVC / API）での DI

### 6.1 Controllers / Endpoints

- コンストラクタ注入が基本
    
- `ILogger<T>`、`IOptions<T>`、`IHttpClientFactory` などはフレームワーク組み込み
    

### 6.2 BackgroundService

```csharp
public class Worker : BackgroundService
{
    private readonly ILogger<Worker> _log;
    private readonly IServiceScopeFactory _scopeFactory;

    public Worker(ILogger<Worker> log, IServiceScopeFactory scopeFactory)
    {
        _log = log;
        _scopeFactory = scopeFactory;
    }

    protected override async Task ExecuteAsync(CancellationToken ct)
    {
        while (!ct.IsCancellationRequested)
        {
            using var scope = _scopeFactory.CreateScope();
            var db = scope.ServiceProvider.GetRequiredService<AppDbContext>(); // Scoped を都度作成
            // 処理...
            await Task.Delay(TimeSpan.FromSeconds(10), ct);
        }
    }
}
```

**注意**: Singleton（Worker 本体）から Scoped を**キャプチャせず**、必要時にスコープを作る

### 6.3 Middleware

- コンストラクタは Singleton 的に注入される
    
- リクエストごとに必要な Scoped サービスは `InvokeAsync` の引数で受け取るか `IServiceProvider` から取得する
    

---

## 7. HttpClient と構成（Options）

### 7.1 IHttpClientFactory

```csharp
builder.Services.AddHttpClient("external", c => c.BaseAddress = new Uri("https://api.example.com"));
builder.Services.AddHttpClient<MyTypedClient>(c => c.BaseAddress = new Uri("https://api.example.com"));

public class MyTypedClient
{
    private readonly HttpClient _http;
    public MyTypedClient(HttpClient http) => _http = http;
}
```

**原則**: `new HttpClient()` を直接作らない。ソケット枯渇やハンドラ管理の問題を回避

### 7.2 Options パターン

```csharp
public class MyOptions { public string Endpoint { get; set; } = ""; }

builder.Services.Configure<MyOptions>(builder.Configuration.GetSection("My"));
// 変更追従が必要なら IOptionsMonitor<T> を使う
```

---

## 8. Keyed Services（.NET 8 以降）

```csharp
public interface IPayment {}
public class PayPay : IPayment {}
public class LinePay : IPayment {}

builder.Services.AddKeyedScoped<IPayment, PayPay>("paypay");
builder.Services.AddKeyedScoped<IPayment, LinePay>("linepay");

// 使う側（例: Minimal API）
app.MapPost("/pay", ([FromKeyedServices("paypay")] IPayment pay) => { /* ... */ });
```

用途が同じで実装が複数ある場合の切り替えに有効。

---

## 9. スコープと破棄（Dispose）

- Scoped / Transient が `IDisposable` なら、スコープ終了時に破棄される
    
- Singleton はアプリ終了時に破棄
    
- 非同期破棄は `IAsyncDisposable` を実装
    
- Singleton に Scoped を保持すると破棄漏れや不正アクセスが起きるため禁止
    

---

## 10. よくある落とし穴と対策

- **サービスロケータ乱用**（`IServiceProvider.GetService` の多用）  
    → 設計が不透明になる。**コンストラクタ注入を基本**にする
    
- **Singleton に状態を持たせる**  
    → できる限り不変・スレッドセーフ。可変状態は別レイヤで管理
    
- **Scoped を Singleton に注入する**  
    → 破棄や同時実行で破綻。`IServiceScopeFactory` で必要時に取得
    
- **DbContext のライフタイム誤り**  
    → 必ず Scoped。長期保持しない
    
- **Blazor で `[Inject]` を初期化子やコンストラクタで参照**  
    → 未設定の可能性。`OnInitialized{Async}` 以降で利用
    
- **Blazor WASM の Scoped をセッション的に使う**  
    → 実質 Singleton。ユーザー状態は外部ストアや状態管理で工夫
    

---

## 11. 設計の流れ（上流 → 下流）

1. **要件整理**
    
    - どの単位で状態を隔離すべきか（ユーザー／リクエスト／アプリ全体）
        
    - 外部 API、DB、認証、キャッシュの有無
        
2. **責務分割とライフタイム設計**
    
    - ドメインサービス、アプリケーションサービス、リポジトリ
        
    - `Transient`（純粋ロジック）、`Scoped`（ユースケース／トランザクション）、`Singleton`（設定・定数）
        
3. **登録ポリシーと命名**
    
    - `AddXxx` を `Program.cs` に集約
        
    - Named/Keyed、TypedHttpClient、Options を整理
        
4. **解決パターンの統一**
    
    - 原則コンストラクタ注入
        
    - Blazor コンポーネントのみ `[Inject]` を例外的に許容
        
5. **テスト方針**
    
    - DI コンテナで置換可能な設計（インターフェース）
        
    - Web（bUnit / WebApplicationFactory）、サービス（xUnit + TestHost）
        
6. **運用**
    
    - 破棄（Dispose）方針、可観測性（ログ/メトリクス）
        
    - 構成変更は Options + `IOptionsMonitor<T>`
        

---

## 12. 最小構成のサンプル

### 12.1 サービス

```csharp
public interface IClock { DateTime Now { get; } }
public class SystemClock : IClock { public DateTime Now => DateTime.Now; }

public interface ISessionService { string? UserName { get; set; } }
public class SessionService : ISessionService { public string? UserName { get; set; } }
```

### 12.2 Program.cs

```csharp
var builder = WebApplication.CreateBuilder(args);

builder.Services.AddRazorPages();
builder.Services.AddRazorComponents().AddInteractiveServerComponents(); // Blazor Server 例

builder.Services.AddScoped<IClock, SystemClock>();
builder.Services.AddScoped<ISessionService, SessionService>();

builder.Services.AddHttpClient("ext", c => c.BaseAddress = new Uri("https://api.example.com"));

var app = builder.Build();
app.MapRazorComponents<App>();
app.Run();
```

### 12.3 Blazor コンポーネント（.razor）

```razor
@page "/"
@inject IClock Clock
@inject ISessionService Session

<h3>@Clock.Now</h3>
<p>@Session.UserName</p>

@code {
    protected override void OnInitialized()
    {
        if (string.IsNullOrEmpty(Session.UserName))
            Session.UserName = "Guest";
    }
}
```

---

## 13. チェックリスト

- ライフタイムは責務と寿命で決めているか
    
- Singleton に可変状態を持たせていないか
    
- Scoped を Singleton に保持していないか
    
- Blazor で `[Inject]` の参照タイミングは適切か
    
- `HttpClient` は `IHttpClientFactory` を使っているか
    
- 構成は Options パターンで管理しているか
    
- テストで代替実装に置換できるか
    

---

## 14. まとめ

- **原則はコンストラクタ注入**
    
- **セッションやリクエスト依存の状態は Scoped**
    
- **グローバルな参照は Singleton（不変・スレッドセーフ）**
    
- Blazor はコンポーネントに限り **プロパティ注入（`[Inject]`）が可能**
    
- Singleton から Scoped を直接保持しない。必要時にスコープを切って取得する
    

必要であれば、上記をベースにあなたのプロジェクト構成に合わせたテンプレート（フォルダ構成、`Program.cs` の登録分割、テスト雛形）も提示します。