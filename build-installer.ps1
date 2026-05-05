# build-installer.ps1
# Genera pvm-setup.exe — instalador autonomo con pvm.exe embebido.
#
# Uso:
#   .\build-installer.ps1
#   .\build-installer.ps1 -SkipPvm    # si pvm.exe release ya esta compilado

param(
    [switch]$SkipPvm
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root     = $PSScriptRoot
$DistDir  = Join-Path $Root "dist"
$PvmExe   = Join-Path $Root "target\release\pvm.exe"
$SetupExe = Join-Path $Root "target\release\pvm-setup.exe"

# ── 1. Compilar pvm.exe ───────────────────────────────────────────────────
if (-not $SkipPvm) {
    Write-Host "Compilando pvm.exe..." -ForegroundColor Cyan
    cargo build --release --bin pvm
    if ($LASTEXITCODE -ne 0) { throw "cargo build pvm fallo" }
    Write-Host "OK: pvm.exe listo" -ForegroundColor Green
}

if (-not (Test-Path $PvmExe)) {
    throw "No se encontro $PvmExe. Ejecuta sin -SkipPvm."
}

# ── 2. Compilar pvm-setup.exe (embebe pvm.exe via build.rs) ──────────────
Write-Host "Compilando pvm-setup.exe (embebiendo pvm.exe)..." -ForegroundColor Cyan
cargo build --release --bin pvm-setup
if ($LASTEXITCODE -ne 0) { throw "cargo build pvm-setup fallo" }
Write-Host "OK: pvm-setup.exe listo" -ForegroundColor Green

# ── 3. Copiar a dist/ ────────────────────────────────────────────────────
if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir | Out-Null
}

$versionLine = Select-String -Path (Join-Path $Root "Cargo.toml") -Pattern '^version\s*=\s*"(.+)"' |
               Select-Object -First 1
$version = $versionLine.Matches[0].Groups[1].Value

$output = Join-Path $DistDir "pvm-setup-$version.exe"
Copy-Item $SetupExe $output -Force

$sizeBytes = (Get-Item $output).Length
$sizeMB    = [math]::Round($sizeBytes / 1MB, 2)

Write-Host ""
Write-Host "Instalador listo:" -ForegroundColor Green
Write-Host "  $output"
Write-Host "  Tamanio: $sizeMB MB"
Write-Host ""
Write-Host "Distribuye ese unico archivo. No requiere instalaciones adicionales."
