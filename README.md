# MirrorPilot - 镜像领航员

<p align="center">
  <strong>一站式管理开发环境中所有包管理器的镜像源配置</strong>
</p>

## ✨ 功能特性

- **🔍 自动检测** — 自动识别已安装的包管理器及其当前镜像源
- **🔄 一键切换** — 快速在官方源与各大镜像站之间切换
- **⚡ 网络测速** — 实时测试镜像源延迟与下载速度，智能推荐最优源
- **📦 广泛支持** — 覆盖主流包管理器

### 支持的包管理器

| 包管理器 | 配置文件 | 平台 |
|---------|---------|------|
| **npm** | `.npmrc` | 全平台 |
| **Yarn** | `.yarnrc` | 全平台 |
| **pnpm** | `.npmrc` | 全平台 |
| **pip** | `pip.ini` / `pip.conf` | 全平台 |
| **uv** | `uv.toml` | 全平台 |
| **Go** | `go env -w GOPROXY` | 全平台 |
| **Maven** | `settings.xml` | 全平台 |
| **Gradle** | `init.gradle` | 全平台 |
| **Docker** | `daemon.json` | 全平台 |
| **Cargo** | `config.toml` | 全平台 |
| **NuGet** | `NuGet.Config` | Windows |
| **Chocolatey** | `chocolatey.config` | Windows |

### 内置镜像源

预置了清华大学、阿里云、腾讯云、华为云、中科大等国内主流镜像站，以及官方源。

## 🖥️ 技术栈

- **前端**：React + TypeScript + Tailwind CSS
- **后端**：Rust (Tauri 2.0)
- **构建**：Vite

## 🚀 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) >= 1.77
- [pnpm](https://pnpm.io/)（推荐）

### 开发

```bash
cd frontend
pnpm install
pnpm tauri dev
```

### 构建

```bash
cd frontend
pnpm tauri build
```

## 📄 开源协议

[MIT License](LICENSE)
