<div align="center">

<img src="img/logo_sin_fondo.png" alt="PVM — Python Version Manager" width="320" />

**Super PyMan - Python Version Manager**

[![Version](https://img.shields.io/badge/version-1.0.0-7B2FBE?style=flat-square)](https://github.com/dh4r10/py-man/releases)
[![Windows](https://img.shields.io/badge/Windows-0078D4?style=flat-square&logo=windows&logoColor=white)](https://github.com/dh4r10/py-man/releases/latest)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License MIT](https://img.shields.io/badge/license-MIT-22C55E?style=flat-square)](LICENSE)

_Gestiona múltiples versiones de Python desde la terminal._  
_Binario único. Sin dependencias. Sin permisos de administrador._

[Descargar](#instalación) · [Inicio rápido](#inicio-rápido) · [Comandos](#comandos)

</div>

---

## Instalación

Descarga el instalador de la [última release](https://github.com/dh4r10/py-man/releases/latest):

```
pvm-setup-X.X.X.exe
```

Ejecuta el instalador y sigue el wizard. Al finalizar:

- `pvm.exe` queda en `%LOCALAPPDATA%\pvm\`
- Esa ruta se añade al PATH del usuario automáticamente
- El perfil de PowerShell se configura opcionalmente

> No requiere permisos de administrador.

### Activar en PowerShell

Si no configuraste el perfil durante la instalación, añade esto a tu `$PROFILE`:

```powershell
pvm env | Out-String | Invoke-Expression
```

---

## Inicio rápido

```powershell
# Instalar una versión de Python
pvm install 3.12.4

# Activarla
pvm use 3.12.4

# Verificar
python -V
# Python 3.12.4
```

---

## Comandos

| Comando                         | Descripción                                          |
| ------------------------------- | ---------------------------------------------------- |
| `pvm install <version>`         | Descarga e instala una versión de Python             |
| `pvm use <version>`             | Cambia la versión activa                             |
| `pvm list`                      | Lista las versiones instaladas (`*` marca la activa) |
| `pvm list-remote`               | Lista versiones disponibles en python.org            |
| `pvm list-remote --filter 3.12` | Filtra por prefijo de versión                        |
| `pvm uninstall <version>`       | Elimina una versión instalada                        |
| `pvm default <version>`         | Establece la versión global por defecto              |
| `pvm env`                       | Imprime el comando para configurar el PATH del shell |
| `pvm venv <dir>`                | Crea un entorno virtual con la versión activa        |

---

## Entornos virtuales

PVM ancla cada venv a la ruta real de la versión, no al alias activo. Esto significa que si después haces `pvm use 3.14.0`, los venvs anteriores siguen apuntando a su versión original.

```powershell
# Crea el venv con la versión activa
pvm venv .venv

# Crea el venv con una versión específica sin cambiar el use activo
pvm -3.12.4 venv .venv
```

```powershell
# Activar el venv
.venv\Scripts\activate

# Verificar que está anclado a la versión correcta
python -V
```

---

## Shells soportados

`pvm env` detecta el shell automáticamente. También puedes especificarlo:

```powershell
pvm env --shell powershell   # $env:PATH = "...;$env:PATH"
pvm env --shell bash         # export PATH="...:$PATH"
pvm env --shell fish         # set -gx PATH "..." $PATH
pvm env --shell cmd          # @SET "PATH=...;%PATH%"
```

---

## Cómo funciona

`~/.pvm/bin/` contiene copias del propio binario `pvm.exe` con nombres de shim (`python.exe`, `pip.exe`, etc.). Cuando el sistema operativo ejecuta `python`, encuentra el shim, que resuelve la versión activa y lanza el Python real con `sys.executable` apuntando al directorio exacto de la versión.

```
~/.pvm/
├── versions/
│   ├── 3.12.4/
│   │   └── tools/python.exe
│   └── 3.13.1/
│       └── tools/python.exe
├── aliases/
│   └── current/   ← junction NTFS → versions/X.Y.Z
└── bin/
    ├── python.exe  ← shim (copia de pvm.exe)
    └── pip.exe     ← shim (copia de pvm.exe)
```

---

## Compilar desde código fuente

Requiere [Rust](https://rustup.rs/) 1.70+.

```powershell
git clone https://github.com/dh4r10/py-man
cd py-man

# Compilar
cargo build --release

# El binario queda en:
# target/release/pvm.exe
```

Para generar el instalador, instala [Inno Setup 6](https://jrsoftware.org/isdl.php) y ejecuta:

```powershell
cargo build --release --bin pvm
iscc installer\pvm.iss
# Instalador en: dist\pvm-setup-X.X.X.exe
```

---

## Inspiración

Inspirado en [fnm](https://github.com/Schniz/fnm) (Fast Node Manager) — misma filosofía aplicada al ecosistema Python.
