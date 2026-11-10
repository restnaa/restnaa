param(
    [Parameter(Mandatory=$true)][int]$AppProcessId,
    [ValidateRange(10,300)][int]$Seconds=60,
    [Parameter(Mandatory=$true)][string]$OutputPath
)
$ErrorActionPreference='Stop'
if((Get-Process -Id $AppProcessId).ProcessName -ne 'pausepal'){throw 'Select the installed Restnaa process only.'}
function Read-AppTree {
    $all=Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId
    $ids=[System.Collections.Generic.HashSet[int]]::new()
    $null=$ids.Add($AppProcessId)
    do {
        $added=$false
        foreach($entry in $all){if($ids.Contains([int]$entry.ParentProcessId) -and $ids.Add([int]$entry.ProcessId)){$added=$true}}
    }while($added)
    @($ids | ForEach-Object {Get-Process -Id $_ -ErrorAction SilentlyContinue})
}
$previous=@{}
foreach($process in (Read-AppTree)){$previous[$process.Id]=$process.TotalProcessorTime.TotalSeconds}
$timer=[System.Diagnostics.Stopwatch]::StartNew()
$cpu=0.0; $appCpu=0.0; $peakWorking=0L; $peakPrivate=0L; $samples=0; $peakCount=0
do {
    Start-Sleep -Milliseconds ([Math]::Min(5000,[Math]::Max(1,($Seconds-$timer.Elapsed.TotalSeconds)*1000)))
    if(-not (Get-Process -Id $AppProcessId -ErrorAction SilentlyContinue)){throw 'Restnaa exited during the sample.'}
    $tree=Read-AppTree
    $working=0L; $private=0L
    foreach($process in $tree){
        $now=$process.TotalProcessorTime.TotalSeconds
        $delta=if($previous.ContainsKey($process.Id)){[Math]::Max(0,$now-$previous[$process.Id])}else{$now}
        $cpu+=$delta
        if($process.Id -eq $AppProcessId){$appCpu+=$delta}
        $previous[$process.Id]=$now
        $working+=$process.WorkingSet64; $private+=$process.PrivateMemorySize64
    }
    $peakWorking=[Math]::Max($peakWorking,$working);$peakPrivate=[Math]::Max($peakPrivate,$private)
    $peakCount=[Math]::Max($peakCount,$tree.Count);$samples++
}while($timer.Elapsed.TotalSeconds -lt $Seconds)
$elapsed=$timer.Elapsed.TotalSeconds
$result=[ordered]@{
    sampledAtUtc=[DateTime]::UtcNow.ToString('o');elapsedSeconds=[Math]::Round($elapsed,2);samples=$samples;peakProcessCount=$peakCount
    appCpuSeconds=[Math]::Round($appCpu,4);treeCpuSeconds=[Math]::Round($cpu,4)
    treePercentOfOneCore=[Math]::Round(100*$cpu/$elapsed,3)
    treePercentOfMachine=[Math]::Round(100*$cpu/$elapsed/[Environment]::ProcessorCount,3)
    peakAggregateWorkingSetMiB=[Math]::Round($peakWorking/1MB,2);peakPrivateBytesMiB=[Math]::Round($peakPrivate/1MB,2)
    note='Installed app plus WebView2 descendants; summed working sets may count shared pages more than once. Exited child CPU between samples may be missed. Short host sample, not battery or long-term validation.'
}
$json=$result | ConvertTo-Json
[System.IO.File]::WriteAllText([System.IO.Path]::GetFullPath($OutputPath),$json,[System.Text.UTF8Encoding]::new($false))
$json
