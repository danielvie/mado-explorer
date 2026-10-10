param([ValidateSet('SetupOnly', 'Roundtrip')][string]$Mode = 'SetupOnly')
$ErrorActionPreference = 'Stop'
$root = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$target = Join-Path $root 'target/p22'
$manifest = Join-Path $PSScriptRoot 'Cargo.toml'
$files = @('Cargo.toml', 'Cargo.lock', 'src/main.rs', 'src/native.rs', 'run.ps1', 'Taskfile.yml')
function FileHash([string]$path) {
    $hasher = [Security.Cryptography.SHA256]::Create()
    $input = [IO.File]::OpenRead($path)
    try { return [BitConverter]::ToString($hasher.ComputeHash($input)).Replace('-', '') }
    finally { $input.Dispose(); $hasher.Dispose() }
}
function SourceHashes {
    $lines = foreach ($relative in $files) {
        "$relative=$(FileHash (Join-Path $PSScriptRoot $relative))"
    }
    return ($lines -join "`n")
}
$before = SourceHashes
& cargo build --locked --offline --manifest-path $manifest --target-dir $target
if ($LASTEXITCODE -ne 0) { throw 'Build failed; probe not launched' }
if ($before -cne (SourceHashes)) { throw 'Source changed during build; probe not launched' }
$exe = Join-Path $target 'debug/mado-private-clipboard-probe.exe'
$binaryHash = FileHash $exe
$sourceHasher = [Security.Cryptography.SHA256]::Create()
try { $sourceHash = [BitConverter]::ToString($sourceHasher.ComputeHash([Text.Encoding]::UTF8.GetBytes($before))).Replace('-', '') }
finally { $sourceHasher.Dispose() }
$argument = if ($Mode -eq 'SetupOnly') { '--setup-only' } else { '--roundtrip' }
$head = (& git -C $root rev-parse HEAD).Trim()
$backend = (& git -C $root rev-parse 'HEAD:components/file-clipboard').Trim()
$evidence = Join-Path $PSScriptRoot 'evidence'
[IO.Directory]::CreateDirectory($evidence) | Out-Null
$log = Join-Path $evidence ("{0}-{1}.txt" -f $Mode.ToLowerInvariant(), [DateTime]::UtcNow.ToString('yyyyMMddTHHmmssfffffffZ'))
# Create-new only. Logs contain own identity/resource names, no clipboard contents.
$stream = [IO.File]::Open($log, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read)
$writer = [IO.StreamWriter]::new($stream, [Text.UTF8Encoding]::new($false))
try {
    $header = @("utc=$([DateTime]::UtcNow.ToString('o'))", "host=$env:COMPUTERNAME os=$([Environment]::OSVersion.VersionString)",
        "checkout_head=$head backend_tree=$backend", "parameters=$argument sourceSHA256=$sourceHash binarySHA256=$binaryHash", $before)
    foreach ($line in $header) { Write-Output $line; $writer.WriteLine($line) }
    $writer.Flush()
    & $exe $argument 2>&1 | ForEach-Object { Write-Output $_; $writer.WriteLine($_) }
    $code = $LASTEXITCODE
    $writer.WriteLine("process_exit=$code")
    Write-Output "process_exit=$code"
} finally { $writer.Dispose() }
Write-Output "evidence=$log"
exit $code
