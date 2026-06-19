#!/bin/bash
# 在 CentOS 8 Runner 宿主机上以 root 执行，一次性安装 Tauri v2 所需全部系统依赖
set -e

echo "=== 1. 修复 CentOS 8 源 (已 EOL，切到 vault) ==="
sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*.repo
sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-*.repo

echo "=== 2. 安装 EPEL 和 PowerTools ==="
dnf install -y epel-release
dnf install -y dnf-plugins-core
dnf config-manager --set-enabled powertools || dnf config-manager --set-enabled PowerTools

echo "=== 3. 安装基础构建工具 ==="
dnf install -y gcc gcc-c++ make pkgconfig patchelf curl openssl-devel

echo "=== 4. 安装 webkit2gtk-4.1 及其依赖 ==="
# CentOS 8 源里只有 webkit2gtk3 (4.0)，没有 4.1
# 需要从 EPEL 或第三方源获取，或者用下面的替代方案
# 先试 EPEL：
dnf install -y webkit2gtk4.1-devel 2>/dev/null || {
    echo "webkit2gtk4.1-devel 不在源中，尝试 webkit2gtk3-devel (4.0)..."
    dnf install -y webkit2gtk3-devel
    echo ""
    echo "⚠️  WARNING: webkit2gtk3-devel 提供的是 4.0 版本，Tauri v2 需要 4.1"
    echo "如果编译失败，需要升级系统到 Rocky Linux 9 / AlmaLinux 9 / Fedora 39+"
    echo ""
}

echo "=== 5. 安装 libsoup 和 libappindicator ==="
dnf install -y libsoup3-devel 2>/dev/null || {
    echo "libsoup3-devel 不在源中，尝试 libsoup-devel (2.x)..."
    dnf install -y libsoup-devel
}
dnf install -y libappindicator-gtk3-devel librsvg2-devel 2>/dev/null || \
  dnf install -y libappindicator-devel librsvg2-devel 2>/dev/null || true

echo "=== 6. 安装 Node.js 22 (如未装) ==="
if ! command -v node >/dev/null 2>&1 || [ "$(node -v | cut -d. -f1)" != "v22" ]; then
    curl -fsSL https://rpm.nodesource.com/setup_22.x | bash -
    dnf install -y nodejs
fi
node --version

echo "=== 7. 安装 Rust (如未装) ==="
if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
source "$HOME/.cargo/env"
cargo --version

echo ""
echo "=== 检查关键依赖 ==="
pkg-config --modversion webkit2gtk-4.1 2>/dev/null && echo "✅ webkit2gtk-4.1 OK" || echo "❌ webkit2gtk-4.1 缺失"
pkg-config --modversion libsoup-3.0 2>/dev/null && echo "✅ libsoup-3.0 OK" || echo "❌ libsoup-3.0 缺失"
pkg-config --modversion javascriptcoregtk-4.1 2>/dev/null && echo "✅ javascriptcoregtk-4.1 OK" || echo "❌ javascriptcoregtk-4.1 缺失"
