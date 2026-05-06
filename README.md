<div align="center">

<img src="img/logo_sin_fondo.png" alt="PVM — Python Version Manager" width="320" />

**Super PyMan - Python Version Manager**

[![Version](https://img.shields.io/badge/version-1.1.0-7B2FBE?style=flat-square)](https://github.com/dh4r10/py-man/releases)
[![Windows](https://img.shields.io/badge/Windows-0078D4?style=flat-square&logo=windows&logoColor=white)](https://github.com/dh4r10/py-man/releases/latest)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black)](https://github.com/dh4r10/py-man/releases/latest)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License MIT](https://img.shields.io/badge/license-MIT-22C55E?style=flat-square)](LICENSE)

_Gestiona múltiples versiones de Python desde la terminal._  
_Binario único. Sin dependencias. Sin permisos de administrador._

[Descargar](#instalación) · [Inicio rápido](#inicio-rápido) · [Comandos](#comandos)

</div>

---

## Instalación

### Linux

```bash
curl -fsSL https://raw.githubusercontent.com/dh4r10/py-man/master/install.sh | bash
```

El script detecta tu arquitectura (x86\_64 o aarch64), descarga el binario correcto y configura tu shell automáticamente. Reinicia la terminal o ejecuta:

```bash
export PATH="$HOME/.local/bin:$PATH" && eval "$(pvm env)"
```

Para configurarlo de forma permanente, añade esto a tu `~/.bashrc` o `~/.zshrc`:

```bash
eval "$(pvm env)"
```

### Windows — Vía npm

```bash
npm install -g super-py-man
```

Luego añade esto a tu `$PROFILE` de PowerShell:

```powershell
pvm env | Out-String | Invoke-Expression
```

### Windows — Vía instalador

Descarga el instalador de la [última release](https://github.com/dh4r10/py-man/releases/latest):

```
pvm-setup-X.X.X.exe
```

Ejecuta el instalador y sigue el wizard. Al finalizar:

- `pvm.exe` queda en `%LOCALAPPDATA%\pvm\`
- Esa ruta se añade al PATH del usuario automáticamente
- El perfil de PowerShell se configura opcionalmente

> No requiere permisos de administrador.

---

## Inicio rápido

**Linux / macOS**

```bash
# Instalar una versión de Python
pvm install 3.12.13

# Activarla
pvm use 3.12.13

# Verificar
python3 --version
# Python 3.12.13
```

**Windows**

```powershell
# Instalar una versión de Python
pvm install 3.12.13

# Activarla
pvm use 3.12.13

# Verificar
python -V
# Python 3.12.13
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
| `pvm uninstall-self`            | Desinstala PVM del sistema                           |

---

## Entornos virtuales

PVM ancla cada venv a la ruta real de la versión, no al alias activo. Esto significa que si después haces `pvm use 3.14.0`, los venvs anteriores siguen apuntando a su versión original.

```bash
# Crea el venv con la versión activa
pvm venv .venv

# Crea el venv con una versión específica sin cambiar el use activo
pvm -3.12.13 venv .venv
```

**Linux**
```bash
source .venv/bin/activate
python --version
```

**Windows**
```powershell
.venv\Scripts\activate
python -V
```

---

## Shells soportados

`pvm env` detecta el shell automáticamente. También puedes especificarlo:

```bash
pvm env --shell bash         # export PATH="...:$PATH"
pvm env --shell zsh          # export PATH="...:$PATH"
pvm env --shell fish         # set -gx PATH "..." $PATH
pvm env --shell powershell   # $env:PATH = "...;$env:PATH"
pvm env --shell cmd          # @SET "PATH=...;%PATH%"
```

---

## Cómo funciona

`~/.pvm/bin/` contiene copias del propio binario `pvm` con nombres de shim (`python`, `pip`, etc.). Cuando el sistema operativo ejecuta `python`, encuentra el shim, que resuelve la versión activa y lanza el Python real con `sys.executable` apuntando al directorio exacto de la versión.

**Linux**
```
~/.pvm/
├── versions/
│   └── 3.12.13/
│       ├── bin/
│       │   ├── python3
│       │   └── pip3
│       └── lib/
├── aliases/
│   └── current  ← symlink → versions/3.12.13
└── bin/
    ├── python   ← shim (copia de pvm)
    └── pip      ← shim (copia de pvm)
```

**Windows**
```
~/.pvm/
├── versions/
│   └── 3.12.13/
│       └── tools/
│           ├── python.exe
│           └── Scripts/pip.exe
├── aliases/
│   └── current/  ← junction NTFS → versions/3.12.13
└── bin/
    ├── python.exe  ← shim (copia de pvm.exe)
    └── pip.exe     ← shim (copia de pvm.exe)
```

---

## Compilar desde código fuente

Requiere [Rust](https://rustup.rs/) 1.70+.

**Linux**
```bash
git clone https://github.com/dh4r10/py-man
cd py-man
cargo build --release
# Binario en: target/release/pvm
```

**Windows**
```powershell
git clone https://github.com/dh4r10/py-man
cd py-man
cargo build --release
# Binario en: target\release\pvm.exe
```

Para generar el instalador de Windows, instala [Inno Setup 6](https://jrsoftware.org/isdl.php) y ejecuta:

```powershell
cargo build --release --bin pvm
iscc installer\pvm.iss
# Instalador en: dist\pvm-setup-X.X.X.exe
```

---

## Inspiración

Inspirado en [fnm](https://github.com/Schniz/fnm) (Fast Node Manager) — misma filosofía aplicada al ecosistema Python.
