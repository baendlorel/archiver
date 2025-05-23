#!/bin/sh
# arv installer: 自动从 GitHub Releases 下载并安装最新版本
# 支持 macOS 和 Linux

set -e
REPO="baendlorel/archiver"
BINARY="arv"
INSTALL_DIR="$HOME/.local/bin"
ROOT_DIR="$HOME/.archiver"

# 检查并创建安装目录和根目录
[ -d "$INSTALL_DIR" ] || mkdir -p "$INSTALL_DIR"
[ -d "$ROOT_DIR" ] || mkdir -p "$ROOT_DIR"

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
echo "ver:$VERSION"
echo "verv:${VERSION#v}"

echo "Preparing $BINARY $VERSION ($PLATFORM) ..."
echo "BINARY $BINARY"
echo "VERSION $VERSION"
echo "PLATFORM $PLATFORM"