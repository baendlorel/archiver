#!/usr/bin/env bash
# arv installer: 自动从 GitHub Releases 下载并安装最新版本
# 支持 macOS 和 Linux

set -e
REPO="aldia-dev/archiver"
BINARY="arv"
INSTALL_DIR="$HOME/.local/bin"

# 检测平台
OS="$(uname -s)"
case "$OS" in
    Linux*)   PLATFORM="linux";;
    Darwin*)  PLATFORM="darwin";;
    *)        echo "Unsupported system: $OS"; exit 1;;
esac

# 获取最新版本号（始终获取最新）
VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | head -1 | cut -d '"' -f4)
if [ -z "$VERSION" ]; then
    echo "Cannot get latest version"; exit 1
fi

echo "Preparing $BINARY $VERSION ($PLATFORM) ..."

# 下载二进制文件
dl_url="https://github.com/$REPO/releases/download/$VERSION/${BINARY}-${PLATFORM}-v${VERSION#v}"
tmpfile="/tmp/${BINARY}-${PLATFORM}-v${VERSION#v}"
echo "Downloading: $dl_url"
curl -L --fail -o "$tmpfile" "$dl_url"
chmod +x "$tmpfile"

# 创建安装目录
mkdir -p "$INSTALL_DIR"

# 安装到 ~/.local/bin
mv "$tmpfile" "$INSTALL_DIR/$BINARY"
echo "Installation finished: $INSTALL_DIR/$BINARY"

# 检查 ~/.local/bin 是否在 PATH 中
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    # 检测当前 shell
    shell_name="$(basename \"$SHELL\")"
    if [ "$shell_name" = "zsh" ]; then
        rc_file="$HOME/.zshrc"
    else
        rc_file="$HOME/.bashrc"
    fi
    echo "Adding $HOME/.local/bin into $rc_file。"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc_file"
    echo "Please reopen your shell or execute: source $rc_file"
fi

# 显示版本
"$INSTALL_DIR/$BINARY" --version
