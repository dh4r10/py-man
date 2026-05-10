#!/usr/bin/env bash
set -euo pipefail

REPO="dh4r10/py-man"
INSTALL_DIR="$HOME/.local/bin"

# Get latest release version
VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*"v\([^"]*\)".*/\1/')
if [ -z "$VERSION" ]; then
  echo "Error: no se pudo obtener la última versión de GitHub" >&2
  exit 1
fi

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
  x86_64)          ASSET="pvm-linux-x86_64-v${VERSION}" ;;
  aarch64 | arm64) ASSET="pvm-linux-aarch64-v${VERSION}" ;;
  *)
    echo "Error: arquitectura no soportada: $ARCH" >&2
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/download/v${VERSION}/$ASSET"

echo "Instalando pvm v${VERSION} para Linux ($ARCH)..."

# Download binary
mkdir -p "$INSTALL_DIR"
curl -fsSL "$URL" -o "$INSTALL_DIR/pvm"
chmod +x "$INSTALL_DIR/pvm"

echo "Binario instalado en $INSTALL_DIR/pvm"

# Add ~/.local/bin to PATH and pvm env to shell profiles
add_to_profile() {
  local profile="$1"
  local path_line='export PATH="$HOME/.local/bin:$PATH"'
  local pvm_line='eval "$(pvm env)"'

  if [ ! -f "$profile" ]; then
    return
  fi

  local changed=0

  if ! grep -qF '.local/bin' "$profile"; then
    printf '\n%s\n' "$path_line" >> "$profile"
    changed=1
  fi

  if ! grep -qF 'pvm env' "$profile"; then
    printf '%s\n' "$pvm_line" >> "$profile"
    changed=1
  fi

  if ! grep -qF 'function pvm' "$profile" && ! grep -qF 'pvm()' "$profile"; then
    printf '%s\n' 'pvm() { command pvm "$@"; local s=$?; [ "$1" = "use" ] || [ "$1" = "default" ] && hash -r 2>/dev/null; return $s; }' >> "$profile"
    changed=1
  fi

  if [ "$changed" -eq 1 ]; then
    echo "Configurado en $profile"
  fi
}

add_to_profile "$HOME/.bashrc"
add_to_profile "$HOME/.zshrc"

# Fish shell
FISH_CONFIG="$HOME/.config/fish/config.fish"
if [ -f "$FISH_CONFIG" ]; then
  if ! grep -qF 'pvm env' "$FISH_CONFIG"; then
    printf '\n%s\n' 'fish_add_path "$HOME/.local/bin"' >> "$FISH_CONFIG"
    printf '%s\n' 'pvm env --shell fish | source' >> "$FISH_CONFIG"
    echo "Configurado en $FISH_CONFIG"
  fi
fi

echo ""
echo "pvm instalado correctamente."
echo "Reinicia tu shell o ejecuta:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\" && eval \"\$(pvm env)\""
