function Get-Sha256([string]$LiteralPath) {
    $stream=[System.IO.File]::OpenRead($ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($LiteralPath))
    $algorithm=[System.Security.Cryptography.SHA256]::Create()
    try { return ([System.BitConverter]::ToString($algorithm.ComputeHash($stream))).Replace('-','').ToLowerInvariant() }
    finally { $algorithm.Dispose(); $stream.Dispose() }
}
function Get-ReleaseSignatureStatus([string]$LiteralPath) {
    # A pwsh parent may supply incompatible module search paths to Windows
    # PowerShell. Load the security module belonging to this host explicitly.
    $securityModule=Join-Path $PSHOME 'Modules/Microsoft.PowerShell.Security/Microsoft.PowerShell.Security.psd1'
    Import-Module -Name $securityModule -Force -ErrorAction Stop
    return (Microsoft.PowerShell.Security\Get-AuthenticodeSignature -LiteralPath $LiteralPath -ErrorAction Stop).Status.ToString()
}
