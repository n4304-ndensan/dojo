#理論分野 #OSカーネル #プロセス時間管理

# クロックティック（Clock Tick / Jiffy）

---

## 1. 概要

**クロックティック（Clock Tick）** とは、  
Linux カーネルが CPU 時間を測定・スケジューリングする際に使用する  
**内部時間単位（仮想的なタイムスライス）** である。

1 tick はカーネルの定数 `HZ` により定義され、  
「1 秒あたりにカーネルが時間割込みを発生させる回数」を意味する。

---

## 2. 定義

| 項目 | 内容 |
|------|------|
| 名称 | クロックティック（Clock Tick）または Jiffy |
| 定義元 | Linux カーネル変数 `HZ` |
| 意味 | 「1 秒を HZ 分割した最小単位の時間」 |
| 一般値 | 100, 250, または 1000（ディストリビューションやカーネル設定に依存） |
| 表現単位 | 秒換算で `1 / HZ` 秒 |

---

## 3. 設定値の確認方法

```bash
$ getconf CLK_TCK
````

出力例：

```
100
```

→ この場合、**1 tick = 1 / 100 秒 = 10ms** に相当する。

---

## 4. `/proc` における利用箇所

Linux の `/proc/[pid]/stat` 内では、  
プロセスの CPU 使用時間がクロックティック単位で表現されている。

| フィールド             | 意味              | 単位    | 換算式                   |
| ----------------- | --------------- | ----- | --------------------- |
| field14: `utime`  | ユーザーモードCPU時間    | ticks | `秒 = utime / CLK_TCK` |
| field15: `stime`  | カーネルモードCPU時間    | ticks | 同上                    |
| field16: `cutime` | 子プロセスのユーザーモード時間 | ticks | 同上                    |
| field17: `cstime` | 子プロセスのカーネルモード時間 | ticks | 同上                    |

---

## 5. 換算式と実装例

### 時間換算

$$
t_{\text{sec}} = \frac{\text{ticks}}{\text{CLK\_TCK}}
$$

$$
t_{\text{ms}} = \frac{\text{ticks} \times 1000}{\text{CLK\_TCK}}
$$


### Bash実装例

```bash
CLK_TCK=$(getconf CLK_TCK)
utime=$(awk '{print $14}' /proc/$pid/stat)
user_time_ms=$((utime * 1000 / CLK_TCK))
```

---

## 6. Windowsとの比較

|概念|Linux|Windows|
|---|---|---|
|内部時間単位|クロックティック (`HZ`)|100ナノ秒単位 (FILETIME)|
|1単位あたり時間|1 / HZ 秒|100ns = 10^-7 秒|
|ユーザーモード時間|`/proc/[pid]/stat` field14 (`utime`)|`Win32_Process.UserModeTime`|
|カーネルモード時間|`/proc/[pid]/stat` field15 (`stime`)|`Win32_Process.KernelModeTime`|
|単位換算式|ticks → 秒|100ns × 値|

→ LinuxのCPU時間をWindows互換形式で扱うには、  
`ms = ticks * 1000 / CLK_TCK` の換算が必要。

---

## 7. カーネル内部での扱い

### 概念図

```
┌──────────────┬──────────────┐
│   HZ = 100   │   1 tick = 10ms │
└──────────────┴──────────────┘
      │
      ├─> スケジューラが1 tickごとに実行残時間を減算
      ├─> /proc/[pid]/stat の utime, stime に累積
      └─> /proc/stat, /proc/uptime にも反映
```

### カーネル定義箇所（参考）

```c
// include/linux/jiffies.h
#define HZ CONFIG_HZ
extern unsigned long volatile jiffies;
```

`jiffies` はシステム起動からの総tick数を表すグローバル変数である。

---

## 8. 典型的な換算例

|HZ値|1 tick の長さ|100 tick の長さ|備考|
|---|---|---|---|
|100|10 ms|1.0 秒|多くのx86系環境の標準値|
|250|4 ms|0.4 秒|一部のリアルタイム系|
|1000|1 ms|0.1 秒|高分解能タイマ構成|

---

## 9. 注意事項

- tick数はCPUコア単位ではなく**スレッド単位で累積**される。
    
- `utime` + `stime` が実際のCPU使用時間であり、マルチスレッドでは合算値が実際のCPU時間より大きくなる。
    
- コンテナ環境ではホストカーネルのHZ値を継承するため、値の解釈に注意。
    
- 仮想化環境ではHZが1000の場合でも実時間より誤差が出ることがある（vCPUスケジューラの粒度差）。
    

---

## 10. まとめ

|項目|内容|
|---|---|
|定義|Linuxカーネル内部で時間を表す最小単位|
|システム定数|`HZ` または `CLK_TCK`|
|主な利用箇所|`/proc/[pid]/stat`, `/proc/stat`, `/proc/uptime`|
|換算式|`秒 = ticks / CLK_TCK`|
|実用換算例|`ms = ticks * 1000 / CLK_TCK`|
|Windows対応|100ns単位のFILETIME換算に相当|
|注意点|環境依存、スレッド単位、累積値、単位換算要|

---

## 11. 参考資料

- Linux Kernel Documentation: [`proc(5)`](https://man7.org/linux/man-pages/man5/proc.5.html)
    
- `getconf(1)` — POSIX system configuration utility
    
- `include/linux/jiffies.h`（Linux Kernel Source）
    
- Windows API Reference: [`Win32_Process`](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process)
    

---
