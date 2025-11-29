
## 概要

Windows（PowerShell）と Linux（bash）で、サービス配下（親PIDとその子孫）に属する**各プロセス単位**のリソース指標を一定間隔でCSVに記録する。`available_mb` のみは**システム全体**のスナップショットであり、他は各行の `procId` に対する値（集約なし）。

---

## 収集対象

- **対象範囲**: 監視起点プロセス（Windows: 指定サービスの実体PID / Linux: `dotnet APP_BIN`のPID）と、その**全子孫プロセス**（再帰）。
    
- **行の意味**: 1行=1プロセス（その瞬間のスナップショット）。
    
- **重複**: PIDごとに1回出力（同一時刻で重複しない）。
    

---

## 各指標の定義と変動の仕方

以下、**単位**は 1024 基準（KiB/MiB）を前提。

### 1) CPU 時間

- **列**: `user_time_ms`, `kernel_time_ms`
    
- **意味**: そのプロセスが [[CPU]] を実行した累積時間（ミリ秒）。
    
- **変動**: 時間とともに**単調増加**（プロセス終了で打ち止め）。
    
- **他プロセス影響**: 直接加算はされないが、スケジューラの競合で**増え方**（傾き）は変化する。
    
- **差分の読み方**: `Δuser_time_ms / IntervalSec` や `Δ(kernel+user)/IntervalSec` で**概ねのCPU占有率**目安になる。
    

### 2) I/O バイト

- **列（Windows）**: `read_bytes`, `write_bytes` = `ReadTransferCount`, `WriteTransferCount`（プロセスが要求した転送量）
    
- **列（Linux）**: 仕様では `rchar`/`wchar` を `read_bytes`/`write_bytes` として採用。**理由**: Windowsの TransferCount に概念が近く、ページキャッシュ命中時でも増えるため、クロスOS比較が容易。
    
- **変動**: 基本は**累積**で単調増加。ページキャッシュに全乗りの場合でも Linux の `rchar/wchar` は増える（`read_bytes/write_bytes` は0のままになることが多い）。
    
- **差分の読み方**: `Δread_bytes/IntervalSec` = 読み込みスループット（B/s）。
    

### 3) メモリ

- **`working_set_mb`**: 実メモリ常駐量（Win: WorkingSet ≒ Lin: RSS）。**非単調**（増減する）。
    
- **`private_mb`**: プライベートな占有量（Win: PrivatePageCount、Lin: `RSS−Shared` の近似）。**非単調**。
    
- **`swap_kb`**: スワップ使用量（Win: PageFileUsage[KB]、Lin: VmSwap[kB]）。通常**増加傾向**だが回収で減ることもある（**非単調**）。
    
- **`virtual_mb`**: 仮想アドレス空間サイズ（Win: VirtualSize、Lin: VmSize/statm.size）。**おおむね単調増加**だが解放で減る可能性あり。
    

### 4) システム全体の空きメモリ

- **列**: `available_mb`
    
- **意味**: マシン全体の空き物理メモリ見積もり（Windows: `FreePhysicalMemory`、Linux: `/proc/meminfo:MemAvailable`）。
    
- **変動**: **非単調**で大きく揺れる。同一時刻に出力された行では同じ値になる（ほぼ同時取得のため僅差あり）。
    
- **注意**: プロセス固有値ではない（`procId` 関連なし）。
    

---

## Windows と Linux の列対応

|概念|Windows (WMI/CIM)|Linux (procfs)|CSV 列名|単調性|
|---|---|---|---|---|
|PID/PPID/Name|`ProcessId` / `ParentProcessId` / `Name`|`/proc/[pid]/status:PPid`, `/proc/[pid]/comm`|`procId`,`ppid`,`process_name`|-|
|CPU User|`UserModeTime/10000`|`/proc/[pid]/stat: utime` → ms|`user_time_ms`|増加|
|CPU Kernel|`KernelModeTime/10000`|`/proc/[pid]/stat: stime` → ms|`kernel_time_ms`|増加|
|読み込みI/O|`ReadTransferCount`|**`io:rchar`** を採用|`read_bytes`|増加|
|書き込みI/O|`WriteTransferCount`|**`io:wchar`** を採用|`write_bytes`|増加|
|Working Set|`WorkingSetSize/1MB`|`status:VmRSS`（≒`statm.resident*PAGE_SIZE`）|`working_set_mb`|増減|
|Private|`PrivatePageCount/1MB`|`RSS−Shared`（近似）|`private_mb`|増減|
|Swap|`PageFileUsage` (KB)|`status:VmSwap` (kB)|`swap_kb`|増減|
|Virtual|`VirtualSize/1MB`|`status:VmSize` or `statm.size*PAGE_SIZE`|`virtual_mb`|増加/減少|
|System Free|`Win32_OperatingSystem.FreePhysicalMemory/1024`|`/proc/meminfo:MemAvailable/1024`|`available_mb`|増減|

> 備考: Linuxの `read_bytes/write_bytes` は「実ディスク到達バイト」であり、ページキャッシュ命中時は0のままになることが多い。クロスOS比較の整合性のため、本仕様では `rchar/wchar` を `read_bytes/write_bytes` として採用。

---

## 差分・スループットの算出

- **I/O スループット**: `read_rate_Bps = (read_bytes[t] - read_bytes[t-1]) / IntervalSec`
    
- **CPU 占有目安**: `cpu_ms = (Δuser_time_ms + Δkernel_time_ms)`、コア数Nなら `min(100, 100 * cpu_ms / (IntervalSec*1000*N))` 程度で概算。
    
- **メモリ増減**: `Δworking_set_mb` や `Δswap_kb` を追う。急増はメモリリークやページアウト兆候の手掛かり。
    

---

## CSV ログ仕様

- **ヘッダ**（共通）:
    
    ```
    timestamp,procId,ppid,process_name,user_time_ms,kernel_time_ms,read_bytes,write_bytes,private_mb,swap_kb,available_mb,working_set_mb,virtual_mb
    ```
    
- **行**: 各プロセスごとに 1 行 / サンプル毎。`available_mb` は同一サンプル内で共通。
    
- **改行/カンマ**: `printf`（Linux）とCSV文字列（Windows）で**1行固定**の整形。プロセス名は改行除去。
    

---

## 実装（保存版）

### Windows PowerShell スクリプト（サービス配下監視）

```powershell
<#!
.SYNOPSIS
  サービス配下のプロセスを監視し、スワップ量(swap_kb)を含むリソース統計をCSV出力する。
#>

param(
    [Parameter(Mandatory = $true)]
    [string]$ServiceName,
    [int]$IntervalSec = 1,
    [string]$OutFile = "service_resource_windows.csv"
)

# === CSVヘッダ ===
"timestamp,procId,ppid,process_name,user_time_ms,kernel_time_ms,read_bytes,write_bytes,private_mb,swap_kb,available_mb,working_set_mb,virtual_mb" |
    Out-File $OutFile -Encoding UTF8

# === サービスPID取得 ===
$ServicePid = (Get-CimInstance Win32_Service -Filter "Name='$ServiceName'").ProcessId
if (-not $ServicePid) { Write-Error "サービス [$ServiceName] が見つかりません。"; exit }

# === 子孫プロセス再帰探索 ===
function Get-AllDescendantProcesses {
    param([int]$RootPid)
    $all = @($RootPid)
    $queue = @($RootPid)
    $allProc = Get-CimInstance Win32_Process
    while ($queue.Count -gt 0) {
        $current = $queue[0]
        $queue = if ($queue.Count -gt 1) { $queue[1..($queue.Count - 1)] } else { @() }
        $children = $allProc | Where-Object { $_.ParentProcessId -eq $current }
        foreach ($child in $children) {
            if ($all -notcontains $child.ProcessId) {
                $all += $child.ProcessId
                $queue += $child.ProcessId
            }
        }
    }
    return $all | Sort-Object -Unique
}

Write-Host "監視開始: サービス=$ServiceName (PID=$ServicePid)"
Write-Host "出力: $OutFile`n"

try {
    while ($true) {
        $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        $pids = Get-AllDescendantProcesses -RootPid $ServicePid

        # システム全体のAvailableメモリ
        $os = Get-CimInstance Win32_OperatingSystem
        $avail = [math]::Round($os.FreePhysicalMemory / 1024, 2)   # MB

        foreach ($procId in $pids) {
            try {
                $proc = Get-CimInstance Win32_Process -Filter "ProcessId=$procId"
                if (-not $proc) { continue }

                $user_time_ms   = $proc.UserModeTime / 10000
                $kernel_time_ms = $proc.KernelModeTime / 10000
                $read_bytes     = $proc.ReadTransferCount
                $write_bytes    = $proc.WriteTransferCount
                $private_mb     = [math]::Round($proc.PrivatePageCount / 1MB, 2)
                $swap_kb        = [math]::Round($proc.PageFileUsage, 2)  # KB
                $working_set_mb = [math]::Round($proc.WorkingSetSize / 1MB, 2)
                $virtual_mb     = [math]::Round($proc.VirtualSize / 1MB, 2)

                "$timestamp,$procId,$($proc.ParentProcessId),$($proc.Name),$user_time_ms,$kernel_time_ms,$read_bytes,$write_bytes,$private_mb,$swap_kb,$avail,$working_set_mb,$virtual_mb" |
                    Out-File $OutFile -Append -Encoding UTF8
            } catch {}
        }

        Start-Sleep -Seconds $IntervalSec
    }
}
catch {
    Write-Error $_
}
```

### Linux bash スクリプト（サービス配下監視）

```bash
#!/bin/bash
# ============================================================
# monitor_service_linux_swap_kb.sh
# Windows版 monitor_service_swap_kb.ps1 と同一列構成の Linux監視スクリプト
# ============================================================

set -euo pipefail

APP_BIN="/ndensan/bin/ndensan.framework.us.privatemodule.service.usbbatchstartservice.dll"
HOST_SUFFIX="${HOSTNAME:-unknown}"
LOG_FILE="/ndensan/mnt/todoroki/service_resource_linux_${HOST_SUFFIX}.csv"
INTERVAL_SEC=1

mkdir -p "$(dirname "$LOG_FILE")"

# 監視対象を起動（例: dotnet 実行）。外部からPIDを与える設計に変える場合はここを差し替え。
dotnet "$APP_BIN" &
APP_PID=$!

echo "Monitoring started for PID=$APP_PID → $LOG_FILE"

# === CSVヘッダ ===
echo "timestamp,procId,ppid,process_name,user_time_ms,kernel_time_ms,read_bytes,write_bytes,private_mb,swap_kb,available_mb,working_set_mb,virtual_mb" > "$LOG_FILE"

CLK_TCK=$(getconf CLK_TCK)
PAGE_SIZE=$(getconf PAGESIZE)

# --- 子孫PIDを再帰的に取得 ---
get_descendants() {
  local pid=$1
  local children
  children=$(pgrep -P "$pid" 2>/dev/null || true)
  for child in $children; do
    echo "$child"
    get_descendants "$child"
  done
}

# --- /proc/$pid/stat から utime/stime を安全に抜く（commの括弧対応）---
# 返り値: "utime stime"
get_utime_stime() {
  local pid=$1
  awk '
    {
      i = index($0,")");
      if (i==0) { print "0 0"; next }
      rest = substr($0, i+2);
      n = split(rest, a, " ");
      u = (n>=12 ? a[12] : 0);
      s = (n>=13 ? a[13] : 0);
      print u, s;
    }
  ' "/proc/$pid/stat" 2>/dev/null || echo "0 0"
}

# --- PPid は /proc/$pid/status の PPid: から安全取得 ---
get_ppid() {
  local pid=$1
  awk '/^PPid:/ {print $2; found=1} END{if(!found) print 0}' "/proc/$pid/status" 2>/dev/null || echo 0
}

# === メイン監視ループ ===
while kill -0 "$APP_PID" 2>/dev/null; do
  timestamp=$(date '+%Y-%m-%d %H:%M:%S')
  available_mb=$(awk '/MemAvailable:/ {print int($2/1024)}' /proc/meminfo)

  all_pids="$APP_PID $(get_descendants "$APP_PID" | tr '\n' ' ')"

  for pid in $all_pids; do
    [ -r "/proc/$pid/stat" ] || continue

    proc_name=$(tr -d '\n\r' < /proc/$pid/comm 2>/dev/null || echo "")
    ppid=$(get_ppid "$pid")

    # utime/stime（jiffies）→ ms
    read utime_j stime_j < <(get_utime_stime "$pid")
    utime_j=${utime_j:-0}; stime_j=${stime_j:-0}
    user_time_ms=$((utime_j * 1000 / CLK_TCK))
    kernel_time_ms=$((stime_j * 1000 / CLK_TCK))

    # === I/O統計（Windows TransferCountに寄せる: rchar/wchar を採用）===
    read_bytes=$(awk '/^rchar:/ {print $2}' /proc/$pid/io 2>/dev/null || echo 0)
    write_bytes=$(awk '/^wchar:/ {print $2}' /proc/$pid/io 2>/dev/null || echo 0)

    # === メモリ統計 ===
    read size resident shared _ _ _ _ < /proc/$pid/statm 2>/dev/null || true
    size=${size:-0}; resident=${resident:-0}; shared=${shared:-0}
    virtual_mb=$((size * PAGE_SIZE / 1024 / 1024))
    working_set_mb=$((resident * PAGE_SIZE / 1024 / 1024))
    shared_mb=$((shared * PAGE_SIZE / 1024 / 1024))
    private_mb=$((working_set_mb - shared_mb))
    [ "$private_mb" -lt 0 ] && private_mb=0

    # スワップ（KB）
    swap_kb=$(awk '/^VmSwap:/ {print $2}' /proc/$pid/status 2>/dev/null || echo 0)

    # 出力（1行固定）
    printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
      "$timestamp" "$pid" "$ppid" "$proc_name" \
      "${user_time_ms:-0}" "${kernel_time_ms:-0}" \
      "${read_bytes:-0}" "${write_bytes:-0}" \
      "${private_mb:-0}" "${swap_kb:-0}" \
      "${available_mb:-0}" "${working_set_mb:-0}" "${virtual_mb:-0}" >> "$LOG_FILE"
  done

  sleep "$INTERVAL_SEC"
done

echo "Monitoring stopped at $(date)" >> "$LOG_FILE"
```

---

## 運用・解析 Tips

- **WindowsとLinux比較**: I/Oは本仕様により概念整合（TransferCount≒rchar/wchar）。グラフは**差分/秒**で見ると傾向が掴みやすい。
    
- **しきい値例**: `swap_kb` の**継続増加**、`available_mb` の**急落**、`working_set_mb` の**右肩上がり＋GC不発**等は要注意。
    
- **子プロセス寄与**: 親/子別に出力されるため、**合算ビュー**が必要なら後処理で `groupby(timestamp, 親系統)` を実施。
    

---

## 今後の改善案

1. **差分CSVの併産生**（`*_delta.csv`）: 各カウンタのΔとB/s, ms/s を出す。
    
2. **`status:VmRSS/VmSize` に切替**: ヒューマンリーダブルな数値を優先。
    
3. **PID列のユニーク化**: `all_pids` を `sort -u` で安全化。
    
4. **外部PID指定モード**: 対象起動を外出しし、任意PID/サービスへの適用を容易化。