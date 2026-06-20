use crate::modules::silent_command::silent_command;
use crate::modules::types::{PackageManager, PackageManagerStatus, Platform};

/// 平台和包管理器检测模块
pub struct PlatformDetector;

impl PlatformDetector {
    /// 检测当前运行平台
    pub fn detect_platform() -> Platform {
        if cfg!(windows) {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else {
            Platform::Linux
        }
    }

    /// 快速检查命令是否在 PATH 中（纯 Rust，零子进程）
    fn is_on_path(cmd: &str) -> bool {
        which::which(cmd).is_ok()
    }

    /// 获取包管理器对应的命令名
    fn command_name(pm: &PackageManager) -> &'static str {
        match pm {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Pip => "pip",
            PackageManager::Uv => "uv",
            PackageManager::Go => "go",
            PackageManager::Maven => "mvn",
            PackageManager::Gradle => "gradle",
            PackageManager::Docker => "docker",
            PackageManager::Apt => "apt",
            PackageManager::Yum => "yum",
            PackageManager::Homebrew => "brew",
            PackageManager::Cargo => "cargo",
            PackageManager::NuGet => "nuget",
            PackageManager::Chocolatey => "choco",
            PackageManager::DotNet => "dotnet",
            PackageManager::Winget => "winget",
            PackageManager::Rustup => "rustup",
        }
    }

    /// 检测指定包管理器是否已安装
    pub fn detect_package_manager(pm: &PackageManager) -> PackageManagerStatus {
        let platform = Self::detect_platform();

        if !pm.supported_platforms().contains(&platform) {
            return PackageManagerStatus {
                package_manager: pm.id().to_string(),
                display_name: pm.display_name().to_string(),
                installed: false,
                version: None,
                current_source_url: None,
                config_path: None,
            };
        }

        let cmd = Self::command_name(pm);

        if !Self::is_on_path(cmd) {
            return PackageManagerStatus {
                package_manager: pm.id().to_string(),
                display_name: pm.display_name().to_string(),
                installed: false,
                version: None,
                current_source_url: None,
                config_path: crate::modules::config_manager::ConfigManager::get_config_path(pm)
                    .map(|p| p.to_string_lossy().to_string()),
            };
        }

        let (_, version) = Self::check_installed(pm);

        let current_source_url =
            crate::modules::config_manager::ConfigManager::read_current_source(pm);

        let config_path = crate::modules::config_manager::ConfigManager::get_config_path(pm)
            .map(|p| p.to_string_lossy().to_string());

        PackageManagerStatus {
            package_manager: pm.id().to_string(),
            display_name: pm.display_name().to_string(),
            installed: true,
            version,
            current_source_url,
            config_path,
        }
    }

    /// 并行检测所有支持的包管理器
    pub async fn detect_all_parallel() -> Vec<PackageManagerStatus> {
        let pms = [
            PackageManager::Npm,
            PackageManager::Yarn,
            PackageManager::Pnpm,
            PackageManager::Pip,
            PackageManager::Uv,
            PackageManager::Go,
            PackageManager::Maven,
            PackageManager::Gradle,
            PackageManager::Docker,
            PackageManager::Cargo,
            PackageManager::NuGet,
            PackageManager::Chocolatey,
            PackageManager::DotNet,
            PackageManager::Winget,
            PackageManager::Rustup,
        ];

        let handles: Vec<_> = pms
            .iter()
            .map(|pm| {
                let pm = pm.clone();
                tokio::task::spawn_blocking(move || Self::detect_package_manager(&pm))
            })
            .collect();

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            match handle.await {
                Ok(status) => results.push(status),
                Err(_) => results.push(PackageManagerStatus {
                    package_manager: String::new(),
                    display_name: String::new(),
                    installed: false,
                    version: None,
                    current_source_url: None,
                    config_path: None,
                }),
            }
        }
        results
    }

    fn check_installed(pm: &PackageManager) -> (bool, Option<String>) {
        let (cmd, args) = match pm {
            PackageManager::Npm => ("npm", vec!["--version"]),
            PackageManager::Yarn => ("yarn", vec!["--version"]),
            PackageManager::Pnpm => ("pnpm", vec!["--version"]),
            PackageManager::Pip => ("pip", vec!["--version"]),
            PackageManager::Uv => ("uv", vec!["--version"]),
            PackageManager::Go => ("go", vec!["version"]),
            PackageManager::Maven => ("mvn", vec!["--version"]),
            PackageManager::Gradle => ("gradle", vec!["--version"]),
            PackageManager::Docker => ("docker", vec!["--version"]),
            PackageManager::Apt => ("apt", vec!["--version"]),
            PackageManager::Yum => ("yum", vec!["--version"]),
            PackageManager::Homebrew => ("brew", vec!["--version"]),
            PackageManager::Cargo => ("cargo", vec!["--version"]),
            PackageManager::NuGet => ("nuget", vec!["help"]),
            PackageManager::Chocolatey => ("choco", vec!["--version"]),
            PackageManager::DotNet => ("dotnet", vec!["--version"]),
            PackageManager::Winget => ("winget", vec!["--version"]),
            PackageManager::Rustup => ("rustup", vec!["--version"]),
        };

        // ponytail: silent_command 自动处理 Windows .cmd 脚本，CREATE_NO_WINDOW 防弹窗
        let output = silent_command(cmd)
            .args(&args)
            .output();

        match output {
            Ok(o) if o.status.success() => {
                let version_str = String::from_utf8_lossy(&o.stdout).trim().to_string();
                // ponytail: 清理版本号 — 去 v 前缀，从多行输出中提取纯版本号
                let version = version_str.lines().next().map(|s| {
                    let s = s.trim().strip_prefix('v').unwrap_or(s.trim());
                    // "rustup 1.29.0 ..." → "1.29.0", "Go go1.24.4 ..." → "go1.24.4"
                    if let Some(pos) = s.find(|c: char| c.is_ascii_digit()) {
                        let rest = &s[pos..];
                        if let Some(end) = rest.find(' ') {
                            rest[..end].to_string()
                        } else {
                            rest.to_string()
                        }
                    } else {
                        s.to_string()
                    }
                });
                (true, version)
            }
            _ => (false, None),
        }
    }
}
