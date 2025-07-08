#!/bin/sh

set -e

if [ "$OS" = "Windows_NT" ]; then
  echo "Windows is not yet supported by this install script."
  exit 1
else
	case $(uname -sm) in
	"Darwin x86_64") target="macos-x86_64" ;;
	"Darwin arm64") target="macos-aarch64" ;;
  *) 
    echo "Unsupported OS or architecture: $(uname -sm)"
    exit 1
    ;;
	esac
fi

# Initialize variables
should_run_shell_setup=false

toxo_uri="https://github.com/oscarotero/toxojs/releases/latest/download/toxo-${target}.tar.gz"
toxo_install="${TOXO_INSTALL:-$HOME/.toxo}"
bin_dir="$toxo_install/bin"
exe="$bin_dir/toxo"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.tar.gz" "$toxo_uri"
cd "$bin_dir" && tar -xvf "$exe.tar.gz"
chmod +x "$exe"
rm "$exe.tar.gz"

echo "TOXO was installed successfully to $exe"
echo
if [ -f "$HOME/.zshrc" ]; then
  if ! grep -q "export PATH=\"$bin_dir:\$PATH\"" "$HOME/.zshrc"; then
    echo "\n# TOXO\nexport PATH=\"$bin_dir:\$PATH\"" >> "$HOME/.zshrc"
    source "$HOME/.zshrc"
    echo "Added TOXO to your $HOME/.zshrc file"
  fi
elif [ -f "$HOME/.bashrc" ]; then
  if ! grep -q "export PATH=\"$bin_dir:\$PATH\"" "$HOME/.bashrc"; then
    echo "\n# TOXO\nexport PATH=\"$bin_dir:\$PATH\"" >> "$HOME/.bashrc"
    source "$HOME/.bashrc"
    echo "Added TOXO to your $HOME/.bashrc file"
  fi
else
  echo "Add the directory to your $HOME/.bashrc (or similar):"
  echo "export PATH=\"$bin_dir:\$PATH\""
fi

echo

if command -v toxo >/dev/null; then
  echo "Run 'toxo' to get started"
else
  echo "Run '$exe' to get started"
fi
