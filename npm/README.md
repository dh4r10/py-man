# super-py-man

> Windows installer for [PVM — Python Version Manager](https://github.com/dh4r10/py-man)

[![npm version](https://img.shields.io/npm/v/super-py-man?style=flat-square&color=7B2FBE)](https://www.npmjs.com/package/super-py-man)
[![Windows](https://img.shields.io/badge/Windows-0078D4?style=flat-square&logo=windows&logoColor=white)](https://github.com/dh4r10/py-man/releases/latest)
[![License MIT](https://img.shields.io/badge/license-MIT-22C55E?style=flat-square)](../LICENSE)

Installs the `pvm` binary on Windows via npm. Downloads the official release from GitHub automatically.

> **Linux users:** use the shell installer instead — see [main README](https://github.com/dh4r10/py-man#installation).

---

## Install

```bash
npm install -g super-py-man
```

This downloads `pvm.exe` from the [latest GitHub release](https://github.com/dh4r10/py-man/releases/latest) and places it next to the npm wrapper.

---

## Setup

Add this to your PowerShell `$PROFILE` so `pvm` configures your PATH on every session:

```powershell
pvm env | Out-String | Invoke-Expression
```

To find your profile path:

```powershell
echo $PROFILE
```

---

## Usage

```powershell
pvm install 3.13       # install latest 3.13 patch
pvm install 3.12.10    # install specific version
pvm use 3.13.3         # switch active version
pvm list               # list installed versions
pvm list-remote        # browse available versions
pvm venv .venv         # create virtual environment
pvm --help             # all commands
```

---

## Requirements

- Windows 10 / 11
- Node.js >= 14
- PowerShell 5.1+ or PowerShell 7+

---

## Links

- [Full documentation](https://github.com/dh4r10/py-man)
- [Releases](https://github.com/dh4r10/py-man/releases)
- [Report an issue](https://github.com/dh4r10/py-man/issues)
