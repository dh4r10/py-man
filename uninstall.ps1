# PVM — Desinstalador manual
# Ejecutar con: powershell -ExecutionPolicy Bypass -File uninstall.ps1

Write-Host "Desinstalador de PVM" -ForegroundColor Magenta
Write-Host ""
Write-Host "Esto eliminara:"
Write-Host "  * $env:USERPROFILE\.pvm\  (versiones, aliases, shims)"
Write-Host "  * PATH del usuario        (entrada de PVM)"
Write-Host "  * Perfil de PowerShell    (linea pvm env)"
Write-Host "  * $env:LOCALAPPDATA\pvm\  (binario pvm.exe)"
Write-Host ""

$confirm = Read-Host "Confirmar desinstalacion? [s/N]"
if ($confirm -notmatch '^(s|si|yes|y)$') {
    Write-Host "Cancelado."
    exit 0
}

Write-Host ""

# 1. Eliminar ~/.pvm/
$pvmHome = "$env:USERPROFILE\.pvm"
if (Test-Path $pvmHome) {
    Remove-Item -Recurse -Force $pvmHome
    Write-Host "✓ $pvmHome" -ForegroundColor Green
} else {
    Write-Host "· $pvmHome (no existia)" -ForegroundColor DarkGray
}

# 2. Limpiar PATH del usuario
$regKey = 'HKCU:\Environment'
$oldPath = (Get-ItemProperty -Path $regKey -Name PATH -ErrorAction SilentlyContinue).PATH
if ($oldPath) {
    $installDir = "$env:LOCALAPPDATA\pvm"
    $parts = $oldPath -split ';' | Where-Object {
        $_ -ne '' -and $_.TrimEnd('\') -ne $installDir.TrimEnd('\')
    }
    $newPath = $parts -join ';'
    Set-ItemProperty -Path $regKey -Name PATH -Value $newPath
    Write-Host "✓ PATH del usuario" -ForegroundColor Green
}

# 3. Limpiar perfil de PowerShell
if (Test-Path $PROFILE) {
    $lines = Get-Content $PROFILE
    $filtered = $lines | Where-Object { $_ -notmatch 'pvm env' -and $_ -notmatch '# pvm' }
    $filtered | Set-Content $PROFILE
    Write-Host "✓ Perfil de PowerShell" -ForegroundColor Green
}

# 4. Eliminar directorio de instalacion
$installDir = "$env:LOCALAPPDATA\pvm"
if (Test-Path $installDir) {
    Remove-Item -Recurse -Force $installDir
    Write-Host "✓ $installDir" -ForegroundColor Green
}

Write-Host ""
Write-Host "PVM desinstalado correctamente." -ForegroundColor Green
Write-Host "Abre una nueva terminal para que los cambios en PATH surtan efecto."
