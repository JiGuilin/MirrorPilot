use std::path::PathBuf;

use crate::modules::types::{CacheInfo, PackageManager};

/// 文件扫描模块 - 扫描包管理器的缓存目录
pub struct FileScanner;

impl FileScanner {
    /// 获取指定包管理器的默认缓存路径
    pub fn get_cache_path(pm: &PackageManager) -> Option<PathBuf> {
        match pm {
            PackageManager::Npm => {
                if cfg!(windows) {
                    let local = dirs::cache_dir()?;
                    Some(local.join("npm-cache"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".npm"))
                }
            }
            PackageManager::Yarn => {
                if cfg!(windows) {
                    let local = dirs::cache_dir()?;
                    Some(local.join("Yarn").join("Cache"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".cache").join("yarn"))
                }
            }
            PackageManager::Pnpm => {
                let home = dirs::home_dir()?;
                Some(home.join(".pnpm-store"))
            }
            PackageManager::Pip => {
                if cfg!(windows) {
                    let local = dirs::cache_dir()?;
                    Some(local.join("pip"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".cache").join("pip"))
                }
            }
            PackageManager::Uv => {
                if cfg!(windows) {
                    let local = dirs::cache_dir()?;
                    Some(local.join("uv"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".cache").join("uv"))
                }
            }
            PackageManager::Go => {
                let home = dirs::home_dir()?;
                Some(home.join("go").join("pkg").join("mod"))
            }
            PackageManager::Maven => {
                let home = dirs::home_dir()?;
                Some(home.join(".m2").join("repository"))
            }
            PackageManager::Gradle => {
                let home = dirs::home_dir()?;
                Some(home.join(".gradle").join("caches"))
            }
            PackageManager::Cargo => {
                let home = dirs::home_dir()?;
                Some(home.join(".cargo").join("registry").join("cache"))
            }
            PackageManager::Docker => {
                if cfg!(windows) {
                    Some(PathBuf::from(r"C:\ProgramData\Docker"))
                } else if cfg!(target_os = "macos") {
                    let home = dirs::home_dir()?;
                    Some(home.join("Library").join("Containers").join("com.docker.docker").join("Data"))
                } else {
                    Some(PathBuf::from("/var/lib/docker"))
                }
            }
            PackageManager::Apt => {
                Some(PathBuf::from("/var/cache/apt/archives"))
            }
            PackageManager::Yum => {
                Some(PathBuf::from("/var/cache/yum"))
            }
            PackageManager::Homebrew => {
                if cfg!(target_os = "macos") {
                    Some(PathBuf::from("/usr/local/Cellar"))
                } else {
                    Some(PathBuf::from("/home/linuxbrew/.linuxbrew/Cellar"))
                }
            }
            PackageManager::NuGet => {
                let home = dirs::home_dir()?;
                Some(home.join(".nuget").join("packages"))
            }
            PackageManager::Chocolatey => {
                Some(PathBuf::from(r"C:\ProgramData\chocolatey\lib"))
            }
        }
    }

    /// 扫描所有包管理器的缓存信息
    pub fn scan_all_caches() -> Vec<CacheInfo> {
        let package_managers = [
            PackageManager::Npm,
            PackageManager::Yarn,
            PackageManager::Pnpm,
            PackageManager::Pip,
            PackageManager::Uv,
            PackageManager::Go,
            PackageManager::Maven,
            PackageManager::Gradle,
            PackageManager::Cargo,
            PackageManager::Docker,
            PackageManager::NuGet,
            PackageManager::Chocolatey,
        ];

        package_managers
            .iter()
            .filter_map(|pm| Self::scan_cache(pm).ok())
            .collect()
    }

    /// 扫描指定包管理器的缓存
    pub fn scan_cache(pm: &PackageManager) -> Result<CacheInfo, String> {
        let path = Self::get_cache_path(pm)
            .ok_or_else(|| format!("无法确定 {} 的缓存路径", pm.display_name()))?;

        let path_str = path.to_string_lossy().to_string();
        let exists = path.exists();

        if !exists {
            return Ok(CacheInfo {
                package_manager: pm.id().to_string(),
                path: path_str,
                size_bytes: 0,
                file_count: 0,
                exists: false,
            });
        }

        let (size, count) = Self::calculate_dir_size(&path);

        Ok(CacheInfo {
            package_manager: pm.id().to_string(),
            path: path_str,
            size_bytes: size,
            file_count: count,
            exists: true,
        })
    }

    /// 递归计算目录大小
    fn calculate_dir_size(path: &PathBuf) -> (u64, u64) {
        let mut total_size: u64 = 0;
        let mut total_files: u64 = 0;

        for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e: Result<walkdir::DirEntry, _>| e.ok()) {
            if entry.file_type().is_file() {
                total_files += 1;
                total_size += entry.metadata().map(|m: std::fs::Metadata| m.len()).unwrap_or(0);
            }
        }

        (total_size, total_files)
    }

    /// 清理指定包管理器的缓存
    pub fn clean_cache(pm: &PackageManager) -> Result<String, String> {
        let path = Self::get_cache_path(pm)
            .ok_or_else(|| format!("无法确定 {} 的缓存路径", pm.display_name()))?;

        if !path.exists() {
            return Ok("缓存目录不存在，无需清理".to_string());
        }

        let (size_before, _) = Self::calculate_dir_size(&path);

        match pm {
            PackageManager::Npm => {
                let output = std::process::Command::new("npm")
                    .args(["cache", "clean", "--force"])
                    .output()
                    .map_err(|e| format!("执行 npm cache clean 失败: {}", e))?;
                if !output.status.success() {
                    return Err(format!("清理失败: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
            PackageManager::Yarn => {
                let output = std::process::Command::new("yarn")
                    .args(["cache", "clean"])
                    .output()
                    .map_err(|e| format!("执行 yarn cache clean 失败: {}", e))?;
                if !output.status.success() {
                    return Err(format!("清理失败: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
            PackageManager::Go => {
                let output = std::process::Command::new("go")
                    .args(["clean", "-modcache"])
                    .output()
                    .map_err(|e| format!("执行 go clean -modcache 失败: {}", e))?;
                if !output.status.success() {
                    return Err(format!("清理失败: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
            _ => {
                // 通用清理：直接删除缓存目录
                std::fs::remove_dir_all(&path)
                    .map_err(|e| format!("删除缓存目录失败: {}", e))?;
            }
        }

        let size_mb = size_before as f64 / (1024.0 * 1024.0);
        Ok(format!("已清理 {:.2} MB 缓存", size_mb))
    }
}
