
C# / Rust / Python / TypeScript（Node.js）について、コマンドライン引数・環境変数・設定ファイルを**層（レイヤ）** として統合する方法を横断比較する。最小例→推奨ライブラリ→レイヤ統合→よくある落とし穴の順で整理。

---

## 0. 原則（優先順位と 12-factor）

- 推奨の優先順位（上が強い）: **CLI > 環境変数 > 設定ファイル > デフォルト**。
    
- 機密値は環境変数やシークレットストアを使い、リポジトリに平文で置かない。
    
- 設定は**不変**前提で起動時に読み込み、再読み込みが必要なら機能として明示実装する。
    

代表キー例：`port: number`, `host: string`, `debug: bool`, `db.url: string`。環境変数は `APP_PORT`, `APP_HOST`, `APP_DEBUG`, `APP_DB__URL`（ネストは `__` 区切り）を例示する。

---

## 1. CLI 引数（最小例）

C#（最小）

```csharp
// Program.cs（top-level statements）
// dotnet run -- --port 8081
var argsDic = new Dictionary<string, string>();
for (int i = 0; i < args.Length - 1; i++)
    if (args[i].StartsWith("--")) argsDic[args[i][2..]] = args[i+1];
Console.WriteLine(argsDic.GetValueOrDefault("port", "8080"));
```

Rust（最小）

```rust
// cargo run -- --port 8081
let mut port = 8080u16;
let mut it = std::env::args().skip(1);
while let Some(k) = it.next() {
    if k == "--port" { if let Some(v) = it.next() { port = v.parse().unwrap(); } }
}
println!("{}", port);
```

Python（最小）

```python
# python app.py --port 8081
import sys
port = 8080
argv = sys.argv[1:]
if "--port" in argv:
    i = argv.index("--port")
    port = int(argv[i+1])
print(port)
```

TypeScript（最小, Node 18+）

```ts
// node --loader ts-node/esm app.ts --port 8081
import { parseArgs } from 'node:util';
const { values } = parseArgs({ options: { port: { type: 'string' } } });
const port = Number(values.port ?? 8080);
console.log(port);
```

---

## 2. CLI 引数（推奨ライブラリ）

C#（System.CommandLine）

```csharp
// <PackageReference Include="System.CommandLine" Version="2.*" />
using System.CommandLine;
var port = new Option<int>("--port", () => 8080, "Port");
var root = new RootCommand { port };
root.SetHandler((int p) => Console.WriteLine(p), port);
return await root.InvokeAsync(args);
```

Rust（clap）

```rust
// Cargo.toml: clap = { version = "4", features = ["derive"] }
use clap::Parser;
#[derive(Parser)]
struct Opt { #[arg(long, default_value_t = 8080)] port: u16 }
fn main(){ let opt = Opt::parse(); println!("{}", opt.port); }
```

Python（argparse / Typer）

```python
# argparse（標準）
import argparse
p = argparse.ArgumentParser()
p.add_argument("--port", type=int, default=8080)
args = p.parse_args()
print(args.port)
```

```python
# Typer（Click ベース, 開発体験向上）
import typer
app = typer.Typer()
@app.command()
def main(port: int = 8080):
    print(port)
if __name__ == "__main__":
    app()
```

TypeScript（yargs / commander）

```ts
// npm i yargs yargs-parser @types/yargs --save
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
const argv = yargs(hideBin(process.argv)).option('port', { type: 'number', default: 8080 }).parseSync();
console.log(argv.port);
```

---

## 3. 環境変数（最小）

C#

```csharp
var envPort = Environment.GetEnvironmentVariable("APP_PORT");
int port = int.TryParse(envPort, out var p) ? p : 8080;
```

Rust

```rust
let port = std::env::var("APP_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);
```

Python

```python
import os
port = int(os.getenv("APP_PORT", "8080"))
```

TypeScript（Node）

```ts
const port = Number(process.env.APP_PORT ?? 8080);
```

---

## 4. .env ファイル（任意）

C\#

- 標準では `.env` ローダーがないため、`DotNetEnv` 等のパッケージを使用。
    

```csharp
// DotNetEnv
DotNetEnv.Env.Load(); // .env を現在のプロセス環境に流し込む
```

Rust

```rust
// dotenvy = "0.15"
dotenvy::dotenv().ok(); // .env を読み込んで std::env に反映
```

Python

```python
# pip install python-dotenv
from dotenv import load_dotenv
load_dotenv()  # .env を読み込み
```

TypeScript（Node）

```ts
// npm i dotenv
import 'dotenv/config'; // または: import dotenv from 'dotenv'; dotenv.config();
```

---

## 5. 設定ファイル（JSON / YAML / TOML / INI）

C#（Microsoft.Extensions.Configuration）

```csharp
// Microsoft.Extensions.Configuration.* を利用
using Microsoft.Extensions.Configuration;
var cfg = new ConfigurationBuilder()
    .AddJsonFile("appsettings.json", optional: true)
    .AddIniFile("appsettings.ini", optional: true)
    .AddEnvironmentVariables(prefix: "APP_")
    .AddCommandLine(args)
    .Build();
int port = cfg.GetValue("port", 8080);
string? dbUrl = cfg["db:url"]; // 区切りはコロン
```

備考: 標準は JSON/INI/環境変数/コマンドラインをサポート（YAML は外部パッケージ）。

Rust（`config` クレート + `serde`）

```rust
// Cargo.toml: config = "0.14", serde = { version = "1", features = ["derive"] }
#[derive(serde::Deserialize)]
struct Settings { port: Option<u16>, #[serde(default)] db: Db }
#[derive(serde::Deserialize, Default)]
struct Db { url: Option<String> }

let settings = config::Config::builder()
    .add_source(config::File::with_name("config").required(false)) // config.{toml,yaml,json}
    .add_source(config::Environment::with_prefix("APP").separator("__"))
    .build()?;
let s: Settings = settings.try_deserialize()?;
let port = s.port.unwrap_or(8080);
```

Python（標準 + 外部）

```python
import json, os
# JSON
cfg = {}
if os.path.exists("config.json"):
    with open("config.json", "r", encoding="utf-8") as f:
        cfg = json.load(f)
# TOML (3.11+ 標準)
try:
    import tomllib
    if os.path.exists("config.toml"):
        with open("config.toml", "rb") as f:
            cfg |= tomllib.load(f)
except ModuleNotFoundError:
    pass
# YAML
try:
    import yaml
    if os.path.exists("config.yaml"):
        with open("config.yaml", "r", encoding="utf-8") as f:
            cfg |= yaml.safe_load(f) or {}
except ModuleNotFoundError:
    pass
```

TypeScript（Node）

```ts
// JSON
import fs from 'node:fs';
let cfg: any = {};
if (fs.existsSync('config.json')) cfg = JSON.parse(fs.readFileSync('config.json', 'utf8'));
// YAML
// npm i js-yaml
import yaml from 'js-yaml';
if (fs.existsSync('config.yaml')) Object.assign(cfg, yaml.load(fs.readFileSync('config.yaml', 'utf8')) as object);
// TOML
// npm i toml
import toml from 'toml';
if (fs.existsSync('config.toml')) Object.assign(cfg, toml.parse(fs.readFileSync('config.toml', 'utf8')));
```

---

## 6. レイヤ統合（CLI > ENV > FILE > DEFAULT）

C#（一括ビルダー推奨）

```csharp
using Microsoft.Extensions.Configuration;
var cfg = new ConfigurationBuilder()
  .AddJsonFile("appsettings.json", optional: true)
  .AddEnvironmentVariables(prefix: "APP_")
  .AddCommandLine(args)
  .Build();
var port = cfg.GetValue("port", 8080);
var host = cfg.GetValue("host", "127.0.0.1");
var debug = cfg.GetValue("debug", false);
var dbUrl = cfg["db:url"];
```

Rust（`clap` + `config`）

```rust
use clap::Parser;
#[derive(Parser)] struct Opt { #[arg(long)] port: Option<u16> }
let opt = Opt::parse();
dotenvy::dotenv().ok();
let settings = config::Config::builder()
    .add_source(config::File::with_name("config").required(false))
    .add_source(config::Environment::with_prefix("APP").separator("__"))
    .build()?;
let port = opt.port
    .or_else(|| std::env::var("APP_PORT").ok()?.parse().ok())
    .or_else(|| settings.get::<u16>("port").ok())
    .unwrap_or(8080);
```

Python（`argparse` + `dotenv` + 各ファイル）

```python
import argparse, os, json
from dotenv import load_dotenv
load_dotenv()
# DEFAULT
cfg = {"port": 8080, "host": "127.0.0.1", "debug": False}
# FILE
if os.path.exists("config.json"):
    cfg |= json.load(open("config.json", "r", encoding="utf-8"))
# ENV
if (v := os.getenv("APP_PORT")): cfg["port"] = int(v)
if (v := os.getenv("APP_HOST")): cfg["host"] = v
if (v := os.getenv("APP_DEBUG")): cfg["debug"] = v.lower() in ("1","true","yes")
# CLI
p = argparse.ArgumentParser(); p.add_argument("--port", type=int); p.add_argument("--host"); p.add_argument("--debug", action="store_true")
args = p.parse_args()
if args.port: cfg["port"] = args.port
if args.host: cfg["host"] = args.host
if args.debug: cfg["debug"] = True
print(cfg)
```

TypeScript（`yargs` + `dotenv` + ファイル）

```ts
import 'dotenv/config';
import fs from 'node:fs';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
let cfg: any = { port: 8080, host: '127.0.0.1', debug: false };
if (fs.existsSync('config.json')) Object.assign(cfg, JSON.parse(fs.readFileSync('config.json','utf8')));
if (process.env.APP_PORT) cfg.port = Number(process.env.APP_PORT);
if (process.env.APP_HOST) cfg.host = process.env.APP_HOST;
if (process.env.APP_DEBUG) cfg.debug = /^(1|true|yes)$/i.test(process.env.APP_DEBUG);
const argv = yargs(hideBin(process.argv)).option('port', { type: 'number' }).option('host', { type: 'string' }).option('debug', { type: 'boolean' }).parseSync();
if (argv.port) cfg.port = argv.port;
if (argv.host) cfg.host = argv.host;
if (argv.debug !== undefined) cfg.debug = argv.debug;
console.log(cfg);
```

---

## 7. よくある落とし穴と対策

- **型変換**: 環境変数は文字列。`bool` 変換の規則を明示（`1/true/yes` など）。
    
- **ネストキーの表現**: C# は `:` 区切り、Rust `config` は `__` 環境区切り、TS/Python は自前規約を決める。
    
- **path/URL**: 相対パスは実行ディレクトリ依存。`--config` でパスを受け取り `Path.GetFullPath`/`resolve()` 等で絶対化する。
    
- **優先順位の逆転**: ビルダーに複数ソースを渡す順序が**後勝ち**か**先勝ち**かを確認（C# のビルダーは後勝ち）。
    
- **多環境**: `APP_ENV=production` を導入し、`config.{env}.json` といった差分ファイルで切り替える設計も有効。
    
- **ホットリロード**: 設定変更を反映するならファイル監視や SIGHUP 等の仕組みを用意する。
    

---

## 8. 付録：最小設定ファイル例

`config.json`

```json
{ "port": 8080, "host": "127.0.0.1", "debug": false, "db": { "url": "postgres://user:pass@localhost:5432/app" } }
```

`config.toml`

```toml
port = 8080
host = "127.0.0.1"
debug = false
[db]
url = "postgres://user:pass@localhost:5432/app"
```

`config.yaml`

```yaml
port: 8080
host: 127.0.0.1
debug: false
db:
  url: postgres://user:pass@localhost:5432/app
```

---

### 参照

- モジュール・依存・テスト: [[07-４言語-モジュール-名前空間-依存管理-テスト最小例]]
    
- 文字列・正規表現・パス: [[06-４言語-文字列フォーマット-正規表現-パス操作]]
    
- 標準操作: [[03-４言語-標準操作チートシート]]