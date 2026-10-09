param([switch]$ExpectWrites, [switch]$MinimizedClose)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class MadoOwnedWindow {
    [StructLayout(LayoutKind.Sequential)] public struct Rect { public int left, top, right, bottom; }
    [DllImport("user32")] public static extern IntPtr SetThreadDpiAwarenessContext(IntPtr context);
    [DllImport("user32")] public static extern bool GetClientRect(IntPtr window, out Rect rect);
    [DllImport("user32")] public static extern uint GetDpiForWindow(IntPtr window);
    [DllImport("user32")] public static extern bool ShowWindow(IntPtr window, int command);
    [DllImport("user32")] public static extern bool IsIconic(IntPtr window);
    [DllImport("user32")] public static extern bool IsZoomed(IntPtr window);
}
'@
$fixture = Join-Path $env:TEMP ('mado-s011-native-' + [guid]::NewGuid().ToString('N'))
$p = $null
$originalOverride = $env:MADO_SETTINGS_PATH
function NativePath([string]$path) { @{ encoding = 'Windows'; units = @($path.ToCharArray() | ForEach-Object { [int]$_ }) } }
function FindInvokable($root, [string]$name) {
    $condition = [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::NameProperty, $name)
    foreach ($element in $root.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)) {
        $pattern = $null
        if ($element.TryGetCurrentPattern([Windows.Automation.InvokePattern]::Pattern, [ref]$pattern)) { return $pattern }
    }
    return $null
}
try {
    New-Item -ItemType Directory -Path $fixture | Out-Null
    $a = Join-Path $fixture 'a-é-日本語'
    $b = Join-Path $fixture 'b'
    $missing = Join-Path $fixture 'missing-folder'
    New-Item -ItemType Directory -Path $a, $b | Out-Null
    [IO.File]::WriteAllText((Join-Path $a 'fixture.txt'), 'owned fixture')
    $settings = Join-Path $fixture 'workspace.json'
    $seed = @{
        version = 1
        geometry = @{ position = @(90, 90); size = @(1100, 800); maximized = $false }
        panes = @(
            @{ tabs = @((NativePath $a), (NativePath $fixture)); active = 0 },
            @{ tabs = @((NativePath $b), (NativePath $missing)); active = 1 }
        )
        focused = 1
        layout = @{ kind = 'Split'; vertical = $false; ratio = 0.4; first = @{ kind = 'Pane'; pane = 0 }; second = @{ kind = 'Pane'; pane = 1 } }
        bookmarks = @((NativePath $a), (NativePath $missing))
        recents = @((NativePath $b), (NativePath $a))
    }
    [IO.File]::WriteAllText($settings, ($seed | ConvertTo-Json -Depth 12), [Text.UTF8Encoding]::new($false))
    $before = [IO.File]::ReadAllText($settings)
    $env:MADO_SETTINGS_PATH = $settings
    $exe = Join-Path (Get-Location) 'target\debug\mado-explorer.exe'
    $p = Start-Process -FilePath $exe -PassThru
    $deadline = [DateTime]::UtcNow.AddSeconds(30)
    do {
        Start-Sleep -Milliseconds 200
        $p.Refresh()
        if ($p.HasExited) { throw 'Explorer exited before own-window startup' }
    } while ($p.MainWindowTitle -ne 'Mado Explorer' -and [DateTime]::UtcNow -lt $deadline)
    if ($p.MainWindowTitle -ne 'Mado Explorer') { throw 'Own explorer application window not found' }
    Start-Sleep -Seconds 1
    $root = [Windows.Automation.AutomationElement]::FromHandle($p.MainWindowHandle)
    $names = @($root.FindAll([Windows.Automation.TreeScope]::Descendants, [Windows.Automation.Condition]::TrueCondition) | ForEach-Object { $_.Current.Name })
    if (-not ($names -contains 'a-é-日本語') -or -not ($names -contains 'missing-folder')) { throw 'Native restored tab names absent from own accessibility tree' }
    Write-Output 'Native own-window restore and Unicode/missing-tab accessible labels passed.'
    [MadoOwnedWindow]::SetThreadDpiAwarenessContext([IntPtr](-4)) | Out-Null
    $client = [MadoOwnedWindow+Rect]::new()
    if (-not [MadoOwnedWindow]::GetClientRect($p.MainWindowHandle, [ref]$client)) { throw 'Own client geometry query failed' }
    $scale = [MadoOwnedWindow]::GetDpiForWindow($p.MainWindowHandle) / 96.0
    if ($scale -le 0) { throw 'Own DPI query failed' }
    $normalSize = @((($client.right - $client.left) / $scale), (($client.bottom - $client.top) / $scale))
    if ($MinimizedClose) {
        if (-not $ExpectWrites) { throw 'Minimized close requires reviewed/enabled saving' }
        [MadoOwnedWindow]::ShowWindow($p.MainWindowHandle, 3) | Out-Null
        Start-Sleep -Milliseconds 600
        if (-not [MadoOwnedWindow]::IsZoomed($p.MainWindowHandle)) { throw 'Own maximize did not apply' }
        [MadoOwnedWindow]::ShowWindow($p.MainWindowHandle, 9) | Out-Null
        Start-Sleep -Milliseconds 600
        [MadoOwnedWindow]::ShowWindow($p.MainWindowHandle, 6) | Out-Null
        Start-Sleep -Milliseconds 300
        if (-not [MadoOwnedWindow]::IsIconic($p.MainWindowHandle)) { throw 'Own minimize did not apply' }
    }
    if (-not $p.CloseMainWindow()) { throw 'Own close request refused' }
    if (-not $ExpectWrites) {
        $deadline = [DateTime]::UtcNow.AddSeconds(10)
        $invoke = $null
        do {
            Start-Sleep -Milliseconds 100
            $invoke = FindInvokable $root 'Close without saving'
        } while ($null -eq $invoke -and [DateTime]::UtcNow -lt $deadline)
        if ($null -eq $invoke) { throw 'Close-without-saving action absent from own native accessibility tree' }
        $invoke.Invoke()
    }
    if (-not $p.WaitForExit(10000)) { throw 'Own normal close timed out' }
    if ($p.ExitCode -ne 0) { throw ('Explorer exit code ' + $p.ExitCode) }
    if ($ExpectWrites) {
        $saved = [IO.File]::ReadAllText($settings) | ConvertFrom-Json
        if ($saved.panes.Count -ne 2 -or $saved.panes[0].tabs.Count -ne 2 -or $saved.panes[1].active -ne 1 -or $saved.focused -ne 1 -or $saved.bookmarks.Count -ne 2) { throw 'Latest native close snapshot lost workspace state' }
        if ($saved.geometry.maximized) { throw 'Normal window unexpectedly persisted as maximized' }
        if ($MinimizedClose -and ([Math]::Abs($saved.geometry.size[0] - $normalSize[0]) -gt 2 -or [Math]::Abs($saved.geometry.size[1] - $normalSize[1]) -gt 2)) { throw 'Maximize/minimize replaced saved normal inner bounds' }
        Write-Output 'Native final save/normal close preserved panes, tabs, active/focus and bookmarks.'
        if ($MinimizedClose) { Write-Output 'Native maximize/restore/minimized-close retained normal logical size and completed the last save.' }
        $p = Start-Process -FilePath $exe -PassThru
        $deadline = [DateTime]::UtcNow.AddSeconds(30)
        do {
            Start-Sleep -Milliseconds 200
            $p.Refresh()
            if ($p.HasExited) { throw 'Explorer exited before saved-workspace restart' }
        } while ($p.MainWindowTitle -ne 'Mado Explorer' -and [DateTime]::UtcNow -lt $deadline)
        if ($p.MainWindowTitle -ne 'Mado Explorer') { throw 'Restarted own explorer window not found' }
        Start-Sleep -Seconds 1
        $root = [Windows.Automation.AutomationElement]::FromHandle($p.MainWindowHandle)
        $names = @($root.FindAll([Windows.Automation.TreeScope]::Descendants, [Windows.Automation.Condition]::TrueCondition) | ForEach-Object { $_.Current.Name })
        if (-not ($names -contains 'a-é-日本語') -or -not ($names -contains 'missing-folder')) { throw 'Saved workspace did not restore native tabs on restart' }
        if (-not $p.CloseMainWindow() -or -not $p.WaitForExit(10000) -or $p.ExitCode -ne 0) { throw 'Restarted own explorer did not save and close normally' }
        $restarted = [IO.File]::ReadAllText($settings) | ConvertFrom-Json
        if ($restarted.panes.Count -ne 2 -or $restarted.panes[0].tabs.Count -ne 2 -or $restarted.panes[1].active -ne 1 -or $restarted.focused -ne 1 -or $restarted.bookmarks.Count -ne 2) { throw 'Restart lost persisted workspace state' }
        Write-Output 'Native restart from the application-written settings restored Unicode/missing tabs and preserved workspace state.'
    } else {
        if ([IO.File]::ReadAllText($settings) -ne $before) { throw 'Disabled production gate unexpectedly wrote settings' }
        Write-Output 'Native accessible close action and disabled-save preservation passed.'
    }
    Write-Output 'Disposable native smoke only. Narrator speech, mixed DPI, disconnected monitors and interactive clipboard were not tested.'
} finally {
    if ($p -and -not $p.HasExited) { $p.Kill(); $p.WaitForExit(10000) | Out-Null }
    $env:MADO_SETTINGS_PATH = $originalOverride
    if (Test-Path -LiteralPath $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
}
