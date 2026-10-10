# Own-process listing capture only. Never launches File Pilot or reads desktop/clipboard.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class S016Window {
    [StructLayout(LayoutKind.Sequential)] public struct Rect { public int left, top, right, bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct Point { public int x, y; }
    [DllImport("user32")] public static extern IntPtr SetThreadDpiAwarenessContext(IntPtr value);
    [DllImport("user32")] public static extern bool GetWindowRect(IntPtr window, out Rect value);
    [DllImport("user32")] public static extern bool GetClientRect(IntPtr window, out Rect value);
    [DllImport("user32")] public static extern bool ClientToScreen(IntPtr window, ref Point value);
    [DllImport("user32")] public static extern uint GetWindowThreadProcessId(IntPtr window, out uint pid);
    [DllImport("user32")] public static extern uint GetDpiForWindow(IntPtr window);
    [DllImport("user32")] public static extern bool PrintWindow(IntPtr window, IntPtr dc, uint flags);
}
'@
$repo = Split-Path $PSScriptRoot -Parent
$referenceRoot = 'C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport'
$fixture = "$referenceRoot/target/visual/v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20"
$generator = "$referenceRoot/validation/file-pilot/fixture.mjs"
& node $generator validate
if ($LASTEXITCODE -ne 0) { throw 'Owned fixture validation failed; no launch' }
$exe = Join-Path $repo 'target/debug/mado-explorer.exe'
$source = (& git -C $repo rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Source identity unavailable' }
$hash = (Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash
$output = Join-Path $repo ('target/visual/s016-owned-' + [guid]::NewGuid().ToString('N'))
# New output is retained, never recursively deleted by this script.
New-Item -ItemType Directory -Path $output -ErrorAction Stop | Out-Null
$p = $null
$before = $env:MADO_SETTINGS_PATH
try {
    $env:MADO_SETTINGS_PATH = "$fixture/workspace.json"
    $p = Start-Process -FilePath $exe -ArgumentList ('"' + $fixture + '/listing"') -PassThru
    $deadline = [DateTime]::UtcNow.AddSeconds(30)
    do {
        Start-Sleep -Milliseconds 200
        $p.Refresh()
        if ($p.HasExited) { throw 'Own explorer exited before capture' }
    } while ($p.MainWindowTitle -ne 'Mado Explorer' -and [DateTime]::UtcNow -lt $deadline)
    if ($p.MainWindowTitle -ne 'Mado Explorer') { throw 'Own application window absent' }
    Start-Sleep -Seconds 2
    $window = $p.MainWindowHandle
    [uint32]$pidObserved = 0
    [S016Window]::GetWindowThreadProcessId($window, [ref]$pidObserved) | Out-Null
    if ($pidObserved -ne $p.Id) { throw 'Window does not belong to launched process' }
    [S016Window]::SetThreadDpiAwarenessContext([IntPtr](-4)) | Out-Null
    $outer = [S016Window+Rect]::new()
    $client = [S016Window+Rect]::new()
    $origin = [S016Window+Point]::new()
    if (-not [S016Window]::GetWindowRect($window, [ref]$outer) -or
        -not [S016Window]::GetClientRect($window, [ref]$client) -or
        -not [S016Window]::ClientToScreen($window, [ref]$origin)) { throw 'Own geometry unavailable' }
    $bitmap = [Drawing.Bitmap]::new($outer.right - $outer.left, $outer.bottom - $outer.top)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    try {
        $dc = $graphics.GetHdc()
        try { if (-not [S016Window]::PrintWindow($window, $dc, 2)) { throw 'Own PrintWindow failed' } }
        finally { $graphics.ReleaseHdc($dc) }
        $rectangle = [Drawing.Rectangle]::new($origin.x - $outer.left, $origin.y - $outer.top, $client.right, $client.bottom)
        $image = $bitmap.Clone($rectangle, [Drawing.Imaging.PixelFormat]::Format32bppArgb)
        try {
            $colors = [Collections.Generic.HashSet[int]]::new()
            for ($y = 0; $y -lt $image.Height; $y += 16) {
                for ($x = 0; $x -lt $image.Width; $x += 16) { $colors.Add($image.GetPixel($x, $y).ToArgb()) | Out-Null }
            }
            if ($colors.Count -lt 8) { throw 'Capture lacks useful rendered content; verification pending' }
            $image.Save((Join-Path $output 'listing.png'), [Drawing.Imaging.ImageFormat]::Png)
        } finally { $image.Dispose() }
    } finally { $graphics.Dispose(); $bitmap.Dispose() }
    $record = @{ source = $source; binarySha256 = $hash; pid = $p.Id; fixture = $fixture;
        clientPixels = @($client.right, $client.bottom); dpi = [S016Window]::GetDpiForWindow($window);
        capture = 'PrintWindow of launched own HWND, measured client crop, no desktop/clipboard';
        captureUtc = [DateTime]::UtcNow.ToString('o'); filePilotComparison = 'PENDING'; userVisualApproval = 'PENDING' }
    [IO.File]::WriteAllText((Join-Path $output 'record.json'), ($record | ConvertTo-Json), [Text.UTF8Encoding]::new($false))
    if (-not $p.CloseMainWindow() -or -not $p.WaitForExit(10000) -or $p.ExitCode -ne 0) { throw 'Own normal close failed' }
    & node $generator validate
    if ($LASTEXITCODE -ne 0) { throw 'Post-close fixture validation failed; retained evidence' }
    Write-Output ('OWNED_CAPTURE=' + (Join-Path $output 'listing.png'))
    Write-Output ('OWNED_RECORD=' + (Join-Path $output 'record.json'))
} finally {
    $env:MADO_SETTINGS_PATH = $before
    if ($p -and -not $p.HasExited) { $p.Kill(); $p.WaitForExit(10000) | Out-Null }
}
