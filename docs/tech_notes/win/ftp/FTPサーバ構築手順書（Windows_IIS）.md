
※　PSVersion 5.1.26100.7705で実行する。
## 1. 目的

ローカル環境に FTP サーバを構築し、以下の条件で接続可能とする。

| 項目     | 設定値         |
| ------ | ----------- |
| FTPサーバ | 127.0.0.1   |
| 公開フォルダ | C:\FTP\A    |
| ユーザー   | ftpuserA    |
| パスワード  | ftppawwsord |

---

# 2. IISおよびFTP機能の有効化

PowerShell を **管理者権限**で起動し、以下を実行する。

```powershell
dism /online /enable-feature /featurename:IIS-WebServerRole /all
dism /online /enable-feature /featurename:IIS-WebServer /all
dism /online /enable-feature /featurename:IIS-FTPServer /all
dism /online /enable-feature /featurename:IIS-FTPSvc /all
dism /online /enable-feature /featurename:IIS-FTPExtensibility /all
```

IISを再起動する。

```powershell
iisreset
```

---

# 3. FTP公開フォルダ作成

FTP公開用ディレクトリを作成する。

```powershell
mkdir C:\FTP\A
```

---

# 4. FTP用ユーザー作成

FTP接続用のWindowsユーザーを作成する。

```powershell
net user ftpuserA ftppawwsord /add
```

---

# 5. フォルダ権限設定

FTPユーザーがアクセスできるように権限を付与する。

```powershell
icacls C:\FTP\A /grant "ftpuserA:(OI)(CI)F"
```

---

# 6. FTPサイト作成

PowerShellでFTPサイトを作成する。

```powershell
Import-Module WebAdministration

New-WebFtpSite -Name "LocalFTP" `
-Port 21 `
-PhysicalPath "C:\FTP\A" `
-IPAddress "127.0.0.1"
```

---

# 7. FTP認証設定

Basic認証を有効化する。

```powershell
Set-ItemProperty IIS:\Sites\LocalFTP `
-Name ftpServer.security.authentication.basicAuthentication.enabled `
-Value $true
```

匿名ログインを無効化する。

```powershell
Set-ItemProperty IIS:\Sites\LocalFTP `
-Name ftpServer.security.authentication.anonymousAuthentication.enabled `
-Value $false
```

---

# 8. FTPアクセス許可設定

FTPユーザーにアクセス権を付与する。

```powershell
Add-WebConfiguration `
-Filter "/system.ftpServer/security/authorization" `
-PSPath IIS:\ `
-Location "LocalFTP" `
-Value @{accessType="Allow";users="ftpuserA";permissions="Read,Write"}
```

---

# 9. FTP接続確認

コマンドプロンプトまたはPowerShellで以下を実行する。

```bash
ftp 127.0.0.1
```

ログイン情報

```
user ftpuserA
password ftppawwsord
```

ログイン後、以下のコマンドで確認する。

```
dir
```

---

# 10. 接続イメージ

```
FTP Client
    │
    ▼
127.0.0.1:21
    │
    ▼
IIS FTP Server
    │
    ▼
C:\FTP\A
```

---

# 11. トラブルシューティング

### FTP接続できない場合

IISが起動しているか確認

```powershell
iisreset
```

ポート21が開いているか確認

```powershell
netstat -an | findstr 21
```

---

