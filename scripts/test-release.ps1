$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'release-functions.ps1')
$testRoot=Join-Path ([System.IO.Path]::GetTempPath()) ('restnaa-release-'+[Guid]::NewGuid().ToString('N'))
$fakeRoot=Join-Path $testRoot 'modules'
$fakeModule=Join-Path $fakeRoot 'Microsoft.PowerShell.Security'
$manifest=Join-Path $fakeModule 'Microsoft.PowerShell.Security.psd1'
$moduleScript=Join-Path $fakeModule 'FakeSecurity.psm1'
$sample=Join-Path $testRoot 'unsigned.ps1'
$originalModulePath=$env:PSModulePath
try {
    New-Item -ItemType Directory -Path $fakeModule -Force | Out-Null
    [System.IO.File]::WriteAllText($sample,'abc',[System.Text.Encoding]::ASCII)
    if((Get-Sha256 $sample) -ne 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'){throw 'SHA-256 known-vector check failed.'}
    [System.IO.File]::WriteAllText($sample,"# Unsigned release test fixture`r`nWrite-Output 1`r`n",[System.Text.Encoding]::ASCII)
    [System.IO.File]::WriteAllText($manifest,"@{RootModule='FakeSecurity.psm1';ModuleVersion='99.0.0';FunctionsToExport=@('Get-AuthenticodeSignature')}")
    [System.IO.File]::WriteAllText($moduleScript,"throw 'Simulated incompatible inherited security module'")
    Remove-Module Microsoft.PowerShell.Security -Force -ErrorAction SilentlyContinue
    $env:PSModulePath=$fakeRoot
    $reproduced=$false
    try { Get-AuthenticodeSignature -LiteralPath $sample -ErrorAction Stop | Out-Null }
    catch { $reproduced=$true }
    if(-not $reproduced){throw 'Expected the shadowed security module to fail autoload.'}
    if((Get-ReleaseSignatureStatus $sample) -ne 'NotSigned'){throw 'Explicit host security module did not inspect the unsigned file.'}
    Assert-UnsignedBetaSignature -LiteralPath $sample
    foreach($status in @('Valid','HashMismatch','NotTrusted','UnknownError')) {
        $rejected=$false
        $inspect = {param($path) [pscustomobject]@{Status=$status}}.GetNewClosure()
        try { Assert-UnsignedBetaSignature 'fixture' $inspect } catch { $rejected=$true }
        if(-not $rejected){throw "Unsigned beta gate accepted unexpected $status."}
    }
    Write-Output 'PASS: unsigned beta gate accepts a real unsigned file and rejects unexpected signature states.'
    $missingRejected=$false
    try { Get-ReleaseSignatureStatus (Join-Path $testRoot 'missing.exe') | Out-Null } catch { $missingRejected=$true }
    if(-not $missingRejected){throw 'Missing signature input was silently accepted.'}
    $unsignedRejected=$false
    try { Assert-ReleaseSignature -LiteralPath $sample } catch { $unsignedRejected=$true }
    if(-not $unsignedRejected){throw 'Release gate accepted a real unsigned file.'}
    foreach($status in @('NotSigned','HashMismatch','NotTrusted','UnknownError')) {
        $rejected=$false
        $inspect = {param($path) [pscustomobject]@{Status=$status;SignerCertificate=@{};TimeStamperCertificate=@{}}}.GetNewClosure()
        try { Assert-ReleaseSignature 'fixture' $inspect } catch { $rejected=$true }
        if(-not $rejected){throw "Release gate accepted $status."}
    }
    $untimestampedRejected=$false
    try { Assert-ReleaseSignature 'fixture' {param($path) [pscustomobject]@{Status='Valid';SignerCertificate=@{};TimeStamperCertificate=$null}} } catch { $untimestampedRejected=$true }
    if(-not $untimestampedRejected){throw 'Release gate accepted a missing timestamp.'}
    Assert-ReleaseSignature 'fixture' {param($path) [pscustomobject]@{Status='Valid';SignerCertificate=@{};TimeStamperCertificate=@{}}}
    Write-Output 'PASS: release gate rejects unsigned, invalid/untrusted and untimestamped signatures; accepts valid timestamped inspection metadata (synthetic positive fixture only).'
    Write-Output 'PASS: missing input rejected; SHA-256 vector; incompatible security-module autoload reproduced; explicit host module returned NotSigned.'
} finally {
    $env:PSModulePath=$originalModulePath
    foreach($file in @($sample,$manifest,$moduleScript)){if(Test-Path -LiteralPath $file){Remove-Item -LiteralPath $file}}
    foreach($folder in @($fakeModule,$fakeRoot,$testRoot)){if(Test-Path -LiteralPath $folder){[System.IO.Directory]::Delete($folder)}}
}
