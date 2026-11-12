param([switch]$SkipBuild)
$ErrorActionPreference='Stop'
function Get-Sha256([string]$LiteralPath) {
    $stream=[System.IO.File]::OpenRead($ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($LiteralPath))
    $algorithm=[System.Security.Cryptography.SHA256]::Create()
    try { return ([System.BitConverter]::ToString($algorithm.ComputeHash($stream))).Replace('-','').ToLowerInvariant() }
    finally { $algorithm.Dispose(); $stream.Dispose() }
}
$repoRoot=[System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
Push-Location $repoRoot
try {
    $package=Get-Content -LiteralPath 'package.json' -Raw | ConvertFrom-Json
    $config=Get-Content -LiteralPath 'src-tauri/tauri.conf.json' -Raw | ConvertFrom-Json
    if($package.version -ne $config.version){throw 'Frontend and Tauri versions differ.'}
    if(-not $SkipBuild){ & npm.cmd run tauri build -- --bundles nsis -- --locked; if($LASTEXITCODE -ne 0){throw 'Windows packaging failed.'} }
    $filename='Restnaa_'+$config.version+'_x64-setup.exe'
    $source=Join-Path $repoRoot ('src-tauri/target/release/bundle/nsis/'+$filename)
    if(-not (Test-Path -LiteralPath $source)){throw 'Expected NSIS artifact missing.'}
    $destination=Join-Path $repoRoot ('release/restnaa-'+$config.version)
    New-Item -ItemType Directory -Path (Join-Path $destination 'docs') -Force | Out-Null
    $installer=Join-Path $destination $filename
    Copy-Item -LiteralPath $source -Destination $installer -Force
    $documents=@('RELEASE_NOTES.md','PRIVACY.md','docs/INSTALLATION.md','docs/KNOWN_LIMITATIONS.md','docs/RELEASE_AUDIT.md','LICENSE','THIRD_PARTY_NOTICES.md')
    foreach($document in $documents){Copy-Item -LiteralPath (Join-Path $repoRoot $document) -Destination (Join-Path $destination $document) -Force}
    $encoding=[System.Text.UTF8Encoding]::new($false)
    $hash=(Get-Sha256 $installer)
    [System.IO.File]::WriteAllText(($installer+'.sha256'),($hash+'  '+$filename+"`n"),$encoding)
    $readme="# Restnaa release candidate`n`nStart with [installation](docs/INSTALLATION.md), [release notes](RELEASE_NOTES.md), [privacy](PRIVACY.md), [audit evidence](docs/RELEASE_AUDIT.md) and [limitations](docs/KNOWN_LIMITATIONS.md).`n`nVerify the installer against its SHA-256 file. This candidate is unsigned and still has manual release gates. Build instructions refer to the source repository checkout and its README.`n"
    [System.IO.File]::WriteAllText((Join-Path $destination 'README.md'),$readme,$encoding)
    $sourceCommit=(& git rev-parse HEAD).Trim()
    if($LASTEXITCODE -ne 0){throw 'Could not determine the release source revision.'}
    $sourceDirty=@(& git status --porcelain).Count -gt 0
    if($LASTEXITCODE -ne 0){throw 'Could not determine source worktree status.'}
    $manifest=[ordered]@{
        sourceCommit=$sourceCommit;sourceDirty=$sourceDirty
        product='Restnaa';version=$config.version;releaseCandidate=$true;platform='windows-x86_64';identifier=$config.identifier
        generatedAtUtc=[DateTime]::UtcNow.ToString('o');installer=$filename;bytes=(Get-Item -LiteralPath $installer).Length;sha256=$hash
        authenticodeStatus=(Get-AuthenticodeSignature -LiteralPath $installer).Status.ToString()
        webview2='Evergreen required; download bootstrapper may need internet if runtime is absent'
        lockfiles=@{npm=(Get-Sha256 'package-lock.json');cargo=(Get-Sha256 'src-tauri/Cargo.lock')}
        audit='docs/RELEASE_AUDIT.md';limitations='docs/KNOWN_LIMITATIONS.md'
    }
    [System.IO.File]::WriteAllText((Join-Path $destination 'manifest.json'),($manifest | ConvertTo-Json -Depth 4),$encoding)
    $manifest | ConvertTo-Json -Depth 4
} finally {Pop-Location}
