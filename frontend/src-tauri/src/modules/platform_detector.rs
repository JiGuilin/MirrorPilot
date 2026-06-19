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

    /// 检测指定包管理器是否已安装
    pub fn detect_package_manager(pm: &PackageManager) -> PackageManagerStatus {
        let platform = Self::detect_platform();

        // 检查平台是否支持
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

        // 检查是否安装
        let (installed, version) = Self::check_installed(pm);

        // 如果安装了，获取当前源配置
        let current_source_url = if installed {
            crate::modules::config_manager::ConfigManager::read_current_source(pm)
        } else {
            None
        };

        // 获取配置文件路径
        let config_path = crate::modules::config_manager::ConfigManager::get_config_path(pm)
            .map(|p| p.to_string_lossy().to_string());

        PackageManagerStatus {
            package_manager: pm.id().to_string(),
            display_name: pm.display_name().to_string(),
            installed,
            version,
            current_source_url,
            config_path,
        }
    }

    /// 检测所有支持的包管理器
    pub fn detect_all() -> Vec<PackageManagerStatus> {
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
        ];

        pms.iter().map(Self::detect_package_manager).collect()
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
        };

        // On Windows, use `cmd /C` so PATH-resolved tools (nvm/fnm managed node, etc.) are found
        let output = if cfg!(windows) {
            let mut full_args = vec!["/C".to_string(), cmd.to_string()];
            full_args.extend(args.iter().map(|s| s.to_string()));
            std::process::Command::new("cmd")
                .args(&full_args)
                .output()
        } else {
            std::process::Command::new(cmd)
                .args(&args)
                .output()
        };

        match output {
            Ok(o) if o.status.success() => {
                let version_str = String::from_utf8_lossy(&o.stdout).trim().to_string();
                // 提取版本号（取第一行）
                let version = version_str.lines().next().map(|s| s.to_string());
                (true, version)
            }
            _ => (false, None),
        }
    }
}
