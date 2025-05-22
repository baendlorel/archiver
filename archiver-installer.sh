#!/bin/sh
# arv installer: 自动从 GitHub Releases 下载并安装最新版本
# 支持 macOS 和 Linux

set -e
REPO="baendlorel/archiver"
BINARY="arv"
INSTALL_DIR="$HOME/.local/bin"
ROOT_DIR="$HOME/.archiver"

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
dl_url="https://github.com/$REPO/releases/download/$VERSION/${BINARY}-${PLATFORM}-${VERSION}"
tmpfile="${ROOT_DIR}/${BINARY}-${PLATFORM}-${VERSION}"

echo "Clearing old version and temp files ..."
rm -f "$tmpfile" # 删除下载过的旧文件
rm -f "$INSTALL_DIR/$BINARY" # 删除旧文件

echo "Downloading: $dl_url"
curl -L --fail -o "$tmpfile" "$dl_url"
chmod +x "$tmpfile"

# 创建安装目录
mkdir -p "$INSTALL_DIR"

# 安装到 ~/.local/bin
mv "$tmpfile" "$INSTALL_DIR/$BINARY" # 这里其实顺带连名字也一起改成了arv
echo "Installation finished: $INSTALL_DIR/$BINARY"

# 检查 ~/.local/bin 是否在 PATH 中
if ! echo ":$PATH:" | grep -q ":$HOME/.local/bin:"; then
    # 检测当前 shell
    shell_name="$(basename "$SHELL")"
    if [ "$shell_name" = "zsh" ]; then
        rc_file="$HOME/.zshrc"
    elif [ "$shell_name" = "bash" ]; then
        rc_file="$HOME/.bashrc"
    else
        echo "Unknown shell: $shell_name"
        echo "Please add $HOME/.local/bin to your PATH manually."
        exit 0
    fi
    echo "Adding $HOME/.local/bin into $rc_file"
    # 防止重复追加
    grep -qxF 'export PATH="$HOME/.local/bin:$PATH"' "$rc_file" || echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc_file"
    echo "Please reopen your shell or execute: source $rc_file"
fi

# 显示版本
"$INSTALL_DIR/$BINARY" --version
