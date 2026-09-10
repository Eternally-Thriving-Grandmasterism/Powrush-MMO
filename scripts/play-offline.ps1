# Steam Offline 1.0 door — lived client only.
# No POWRUSH_NET. No listen. No 0.0.0.0. Title Online stays grey.
# Contact: info@Rathor.ai
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
if (-not $Root) { $Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path }
Set-Location $Root

# Force Offline SKU: clear then set off so a lab shell cannot light net.
Remove-Item Env:POWRUSH_NET -ErrorAction SilentlyContinue
$env:POWRUSH_NET = "off"

Write-Host "powrush: Offline door → cargo run -p powrush-client (POWRUSH_NET=off)"
Write-Host "powrush: Title Online stays grey. No public bind."
cargo run -p powrush-client @args
