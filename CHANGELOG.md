# 更新日志

所有重要更改均会记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.3.0] - 2026-06-20

### 新增

- 新增 **NuGet (dotnet)** 包管理器支持 — 通过 `dotnet nuget` 命令读写源，支持官方源和华为云镜像
- 新增 **WinGet** 包管理器支持 — 通过 CLI 命令 `winget source` 管理源，内置官方源和 Microsoft Store 源
- 新增 **Rustup** 包管理器支持 — 通过 `settings.toml` / 环境变量管理镜像，内置清华/中科大/字节跳动镜像
- 新增 `which` crate 依赖 — 用于解析可执行文件真实路径，解决 Windows `.cmd` 脚本无法直接调用的问题
- 新增 `silent_command` 模块 — 封装跨平台静默子进程调用，Windows 上自动判断 `.cmd/.bat` 并走 `cmd /C` 代理

### 修复

- 修复 Windows 下调用 npm/yarn/pnpm 等工具时弹黑色 CMD 窗口的问题（`CREATE_NO_WINDOW` 标志位）
- 修复 Windows 下 `.cmd` 脚本（npm/yarn/pnpm）无法被 `Command::new` 直接执行的问题 — 使用 `which` 解析路径 + `cmd /C` 代理
- 修复版本号显示多余 `v` 前缀的问题（如 winget 输出 `v1.28.240` → 显示 `1.28.240`）
- 修复版本号多行输出只取首行时包含非版本信息的问题（如 rustup 输出 `rustup 1.29.0 ...`）
- 修复 Windows 下 `explorer` 打开文件/文件夹时可能弹窗的问题

### 变更

- `.NET (NuGet)` 显示名称简化为 `NuGet`，新增加的 dotnet 路径同样显示为 `NuGet`
- 所有子进程调用统一走 `silent_command`，消除散落的平台判断代码
- 缓存清理新增 DotNet (`dotnet nuget locals --clear`) 支持

## [0.2.0] - 2026-06-19

### 新增

- UI/UX 优化 + Ponytail 代码精简 + 缓存管理增强

## [0.1.0] - 2026-06-19

### 新增

- 镜像源管理：支持 npm、Yarn、pnpm、pip、uv、Go、Maven、Gradle、Docker、Cargo、NuGet、Chocolatey
- 自动检测已安装的包管理器及当前镜像源配置
- 一键切换镜像源（官方源 ↔ 国内镜像）
- 网络测速：延迟测试与下载速度测试
- 一键测速：批量测试所有可用源
- 国旗图标区分镜像源地区
- 缓存管理模块
- 打开配置文件功能
- 自定义镜像源支持
- 内置清华、阿里云、腾讯云、华为云、中科大等国内镜像
