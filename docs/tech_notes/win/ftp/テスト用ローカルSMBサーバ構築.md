
```ps1
<#
.SYNOPSIS
SMB share environment setup script for reams unittest

.DESCRIPTION
- Create local user
- Create folders (input/output)
- Configure NTFS permissions
- Create SMB shares
- Provide UNC paths for unittest usage

Prerequisites:
- Must be executed with administrator privileges
#>

# =========================
# 1. Parameter Definition
# =========================

# SMB access user
$UserName = "smbuser"

# Password (for testing purpose)
$PasswordPlain = "YourPassword123!"
$Password = ConvertTo-SecureString $PasswordPlain -AsPlainText -Force

# Share configuration for reams unittest
$Shares = @(
    @{ Name = "reams_input";  Path = "C:\SMBShare\input"  },
    @{ Name = "reams_output"; Path = "C:\SMBShare\output" }
)

# =========================
# 2. Create Local User
# =========================

Write-Host "=== Creating User ==="

if (-not (Get-LocalUser -Name $UserName -ErrorAction SilentlyContinue)) {
    New-LocalUser `
        -Name $UserName `
        -Password $Password `
        -FullName "reams smb user" `
        -Description "reams unittest SMB user"
    Write-Host "User created: $UserName"
}
else {
    Write-Host "User already exists: $UserName"
}

# =========================
# 3. Create Folders
# =========================

Write-Host "=== Creating Folders ==="

foreach ($share in $Shares) {
    New-Item -Path $share.Path -ItemType Directory -Force | Out-Null
    Write-Host "Created: $($share.Path)"
}

# =========================
# 4. Configure NTFS Permissions
# =========================

Write-Host "=== Setting NTFS Permissions ==="

foreach ($share in $Shares) {

    # Get current ACL
    $Acl = Get-Acl $share.Path

    # Grant Modify permission to smbuser
    $AccessRule = New-Object System.Security.AccessControl.FileSystemAccessRule(
        $UserName,
        "Modify",
        "ContainerInherit,ObjectInherit",
        "None",
        "Allow"
    )

    # Apply permission
    $Acl.SetAccessRule($AccessRule)
    Set-Acl -Path $share.Path -AclObject $Acl

    Write-Host "Permission applied: $($share.Path)"
}

# =========================
# 5. Create SMB Shares
# =========================

Write-Host "=== Creating SMB Shares ==="

foreach ($share in $Shares) {

    if (-not (Get-SmbShare -Name $share.Name -ErrorAction SilentlyContinue)) {

        New-SmbShare `
            -Name $share.Name `
            -Path $share.Path `
            -FullAccess $UserName | Out-Null

        Write-Host "Share created: $($share.Name)"
    }
    else {
        Write-Host "Share already exists: $($share.Name)"
    }
}

# =========================
# 6. SMB Security Settings
# =========================

Write-Host "=== SMB Configuration ==="

# Disable SMBv1 (security best practice)
Set-SmbServerConfiguration -EnableSMB1Protocol $false -Force | Out-Null

Write-Host "SMBv1 disabled"

# =========================
# 7. Connection Information
# =========================

Write-Host ""
Write-Host "=== Connection Info ==="
Write-Host "Accessible via the following UNC paths:"
foreach ($share in $Shares) {
    Write-Host "\\127.0.0.1\$($share.Name)"
}

Write-Host ""
Write-Host "=== Authentication (if required) ==="
Write-Host "net use \\127.0.0.1\reams_input /user:$UserName $PasswordPlain"

# =========================
# 8. Notes
# =========================

Write-Host ""
Write-Host "=== Notes ==="
Write-Host "- Password may be cached when connecting from the same machine"
Write-Host "- To reset authentication before testing, run:"
Write-Host "  net use * /delete /y"
Write-Host "  cmdkey /delete:127.0.0.1"

```
