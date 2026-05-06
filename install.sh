#!/usr/bin/env bash
set -euo pipefail

REPO="dh4r10/py-man"
INSTALL_DIR="$HOME/.local/bin"

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
  x86_64)          ASSET="pvm-linux-x86_64" ;;
  aarch64 | arm64) ASSET="pvm-linux-aarch64" ;;
  *)
    echo "Error: arquitectura no soportada: $ARCH" >&2
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/latest/download/$ASSET"

echo "Instalando pvm para Linux ($ARCH)..."

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

  if ! grep -qxF 'rehash' "$profile"; then
    printf '%s\n' 'rehash' >> "$profile"
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
