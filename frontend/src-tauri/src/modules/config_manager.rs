use std::fs;
use std::path::PathBuf;

use crate::modules::silent_command::silent_command;
use crate::modules::types::{ApplySourceResult, PackageManager};

/// 配置文件管理器 - 读写各包管理器的配置文件
pub struct ConfigManager;

impl ConfigManager {
    /// 获取指定包管理器的配置文件路径
    pub fn get_config_path(pm: &PackageManager) -> Option<PathBuf> {
        match pm {
            PackageManager::Npm | PackageManager::Pnpm => {
                let home = dirs::home_dir()?;
                Some(home.join(".npmrc"))
            }
            PackageManager::Yarn => {
                let home = dirs::home_dir()?;
                Some(home.join(".yarnrc"))
            }
            PackageManager::Pip => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("pip").join("pip.ini"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".config").join("pip").join("pip.conf"))
                }
            }
            PackageManager::Uv => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("uv").join("uv.toml"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".config").join("uv").join("uv.toml"))
                }
            }
            PackageManager::Go => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("go").join("env"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".config").join("go").join("env"))
                }
            }
            PackageManager::Maven => {
                let home = dirs::home_dir()?;
                Some(home.join(".m2").join("settings.xml"))
            }
            PackageManager::Gradle => {
                let home = dirs::home_dir()?;
                Some(home.join(".gradle").join("init.gradle"))
            }
            PackageManager::Docker => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("Docker").join("settings.json"))
                } else if cfg!(target_os = "macos") {
                    let home = dirs::home_dir()?;
                    Some(home.join(".docker").join("daemon.json"))
                } else {
                    Some(PathBuf::from("/etc/docker/daemon.json"))
                }
            }
            PackageManager::Apt => {
                Some(PathBuf::from("/etc/apt/sources.list"))
            }
            PackageManager::Yum => {
                Some(PathBuf::from("/etc/yum.repos.d"))
            }
            PackageManager::Homebrew => None,
            PackageManager::Cargo => {
                let home = dirs::home_dir()?;
                Some(home.join(".cargo").join("config.toml"))
            }
            PackageManager::NuGet => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("NuGet").join("NuGet.Config"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".config").join("NuGet").join("NuGet.Config"))
                }
            }
            PackageManager::Chocolatey => {
                Some(PathBuf::from("C:\\ProgramData\\chocolatey\\config\\chocolatey.config"))
            }
            // ponytail: DotNet 用 dotnet nuget 子命令管理源，配置文件复用 NuGet 路径
            PackageManager::DotNet => {
                if cfg!(windows) {
                    let appdata = dirs::data_dir()?;
                    Some(appdata.join("NuGet").join("NuGet.Config"))
                } else {
                    let home = dirs::home_dir()?;
                    Some(home.join(".config").join("NuGet").join("NuGet.Config"))
                }
            }
            // ponytail: Winget 通过命令行管理源，无本地配置文件
            PackageManager::Winget => None,
            // ponytail: Rustup 镜像通过环境变量或 rustup 配置文件
            PackageManager::Rustup => {
                let home = dirs::home_dir()?;
                Some(home.join(".rustup").join("settings.toml"))
            }
        }
    }

    /// 读取当前配置的源地址
    pub fn read_current_source(pm: &PackageManager) -> Option<String> {
        match pm {
            PackageManager::Npm | PackageManager::Pnpm => {
                let path = Self::get_config_path(pm)?;
                Self::read_npmrc_registry(&path)
            }
            PackageManager::Yarn => {
                let path = Self::get_config_path(pm)?;
                Self::read_yarnrc_registry(&path)
            }
            PackageManager::Pip => {
                let path = Self::get_config_path(pm)?;
                Self::read_pip_index_url(&path)
            }
            PackageManager::Uv => {
                let path = Self::get_config_path(pm)?;
                Self::read_uv_index_url(&path)
            }
            PackageManager::Go => Self::read_go_env_proxy(),
            PackageManager::Maven => {
                let path = Self::get_config_path(pm)?;
                Self::read_maven_mirror(&path)
            }
            PackageManager::Gradle => {
                let path = Self::get_config_path(pm)?;
                Self::read_gradle_repo(&path)
            }
            PackageManager::Docker => {
                let path = Self::get_config_path(pm)?;
                Self::read_docker_mirror(&path)
            }
            PackageManager::Cargo => {
                let path = Self::get_config_path(pm)?;
                Self::read_cargo_source(&path)
            }
            PackageManager::NuGet => {
                let path = Self::get_config_path(pm)?;
                Self::read_nuget_source(&path)
            }
            PackageManager::DotNet => Self::read_dotnet_nuget_source(),
            PackageManager::Winget => Self::read_winget_source(),
            PackageManager::Rustup => Self::read_rustup_mirror(),
            PackageManager::Apt | PackageManager::Yum | PackageManager::Homebrew | PackageManager::Chocolatey => None,
        }
    }

    fn no_path_result() -> ApplySourceResult {
        ApplySourceResult { success: false, message: "无法确定配置文件路径".to_string(), backup_path: None }
    }

    fn unsupported_result() -> ApplySourceResult {
        ApplySourceResult { success: false, message: "该包管理器暂不支持自动切换源，请手动修改配置".to_string(), backup_path: None }
    }

    /// 应用源地址到配置文件
    pub fn apply_source(pm: &PackageManager, url: &str) -> ApplySourceResult {
        match pm {
            PackageManager::Npm | PackageManager::Pnpm =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_npmrc_registry(&p, url)),
            PackageManager::Yarn =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_yarnrc_registry(&p, url)),
            PackageManager::Pip =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_pip_index_url(&p, url)),
            PackageManager::Uv =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_uv_index_url(&p, url)),
            PackageManager::Go => Self::write_go_env_proxy(url),
            PackageManager::Maven =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_maven_mirror(&p, url)),
            PackageManager::Gradle =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_gradle_repo(&p, url)),
            PackageManager::Docker =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_docker_mirror(&p, url)),
            PackageManager::Cargo =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_cargo_source(&p, url)),
            PackageManager::NuGet =>
                Self::get_config_path(pm).map_or_else(Self::no_path_result, |p| Self::write_nuget_source(&p, url)),
            PackageManager::DotNet => Self::write_dotnet_nuget_source(url),
            PackageManager::Winget => Self::write_winget_source(url),
            PackageManager::Rustup => Self::write_rustup_mirror(url),
            PackageManager::Apt | PackageManager::Yum | PackageManager::Homebrew | PackageManager::Chocolatey =>
                Self::unsupported_result(),
        }
    }

    // ========= 读取方法 =========

    fn read_npmrc_registry(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("registry=") {
                return Some(rest.trim().to_string());
            }
            if let Some(rest) = trimmed.strip_prefix("registry =") {
                return Some(rest.trim().to_string());
            }
        }
        None
    }

    fn read_yarnrc_registry(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("registry") {
                let rest = line["registry".len()..].trim();
                let url = rest
                    .trim_start_matches(':')
                    .trim_start_matches('=')
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                if !url.is_empty() {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    fn read_pip_index_url(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let mut in_global = true;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_global = trimmed == "[global]";
                continue;
            }
            if in_global && trimmed.starts_with("index-url") {
                let rest = trimmed["index-url".len()..].trim();
                let url = rest.trim_start_matches('=').trim_start_matches(' ').trim();
                if !url.is_empty() {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    fn read_uv_index_url(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let value: toml::Value = toml::from_str(&content).ok()?;
        value.get("pip")
            .and_then(|v| v.get("index-url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    // ponytail: 直接调用 go 命令，不经过 cmd /C，CREATE_NO_WINDOW 防弹窗
    fn read_go_env_proxy() -> Option<String> {
        let output = silent_command("go")
            .args(["env", "GOPROXY"])
            .output()
            .ok()?;
        if output.status.success() {
            let proxy = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !proxy.is_empty() {
                return Some(proxy);
            }
        }
        None
    }

    fn read_maven_mirror(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        if let Some(start) = content.find("<url>") {
            if let Some(end) = content.find("</url>") {
                return Some(content[start + 6..end].trim().to_string());
            }
        }
        None
    }

    fn read_gradle_repo(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        if let Some(start) = content.find("url '") {
            let remaining = &content[start + 5..];
            if let Some(end) = remaining.find('\'') {
                return Some(remaining[..end].to_string());
            }
        }
        if let Some(start) = content.find("url \"") {
            let remaining = &content[start + 5..];
            if let Some(end) = remaining.find('"') {
                return Some(remaining[..end].to_string());
            }
        }
        None
    }

    fn read_docker_mirror(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let json: serde_json::Value = serde_json::from_str(&content).ok()?;
        if let Some(mirrors) = json.get("registry-mirrors").and_then(|v| v.as_array()) {
            if let Some(first) = mirrors.first() {
                return first.as_str().map(|s| s.to_string());
            }
        }
        None
    }

    fn read_cargo_source(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let value: toml::Value = toml::from_str(&content).ok()?;
        let replace_name = value.get("source")
            .and_then(|v| v.get("crates-io"))
            .and_then(|v| v.get("replace-with"))
            .and_then(|v| v.as_str())?;

        let mirror = value.get("source")
            .and_then(|v| v.get(replace_name))?;

        if let Some(url) = mirror.get("sparse-registry").and_then(|v| v.as_str()) {
            return Some(format!("sparse+{}", url));
        }
        mirror.get("registry")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn read_nuget_source(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        if let Some(start) = content.find("value=\"") {
            let remaining = &content[start + 7..];
            if let Some(end) = remaining.find('"') {
                return Some(remaining[..end].to_string());
            }
        }
        None
    }

    // ponytail: dotnet nuget list source — 解析输出获取第一个非官方源 URL
    fn read_dotnet_nuget_source() -> Option<String> {
        let output = silent_command("dotnet")
            .args(["nuget", "list", "source"])
            .output()
            .ok()?;
        if !output.status.success() { return None; }
        let text = String::from_utf8_lossy(&output.stdout);
        // 输出格式: "  1.  nuget.org [已启用]\n      https://api.nuget.org/v3/index.json"
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    // ponytail: winget source list — 解析输出获取非 msstore 源 URL
    fn read_winget_source() -> Option<String> {
        let output = silent_command("winget")
            .args(["source", "list"])
            .output()
            .ok()?;
        if !output.status.success() { return None; }
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("名称") || trimmed.starts_with("-") { continue; }
            // 格式: "winget      https://cdn.winget.microsoft.com/cache        false"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let url = parts[1];
                if url.starts_with("https://") || url.starts_with("http://") {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    // ponytail: Rustup 镜像通过 RUSTUP_DIST_SERVER 环境变量或 settings.toml
    fn read_rustup_mirror() -> Option<String> {
        // 优先读环境变量
        if let Ok(val) = std::env::var("RUSTUP_DIST_SERVER") {
            if !val.is_empty() {
                return Some(val);
            }
        }
        // 回退读 settings.toml
        if let Some(path) = dirs::home_dir() {
            let settings_path = path.join(".rustup").join("settings.toml");
            if let Ok(content) = fs::read_to_string(&settings_path) {
                for line in content.lines() {
                    if let Some(rest) = line.trim().strip_prefix("dist_server") {
                        let url = rest.trim_start_matches('=').trim().trim_matches('"').trim_matches('\'');
                        if !url.is_empty() {
                            return Some(url.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    // ========= 写入方法 =========

    fn backup_file(path: &PathBuf) -> Option<PathBuf> {
        if path.exists() {
            let backup = path.with_extension(format!("bak.{}", chrono::Utc::now().timestamp()));
            if fs::copy(path, &backup).is_ok() {
                return Some(backup);
            }
        }
        None
    }

    fn ensure_parent_dir(path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
    }

    fn write_result(success_msg: &str, backup: Option<PathBuf>, write_res: std::io::Result<()>) -> ApplySourceResult {
        match write_res {
            Ok(_) => ApplySourceResult {
                success: true,
                message: success_msg.to_string(),
                backup_path: backup.map(|p| p.to_string_lossy().to_string()),
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("写入失败: {}", e),
                backup_path: None,
            },
        }
    }

    fn replace_or_append_line(path: &PathBuf, url: &str, prefix: &str, new_line: &str, label: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let mut content = if path.exists() {
            fs::read_to_string(path).unwrap_or_default()
        } else {
            String::new()
        };

        let mut found = false;
        let lines: Vec<String> = content.lines().map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with(prefix) {
                found = true;
                new_line.to_string()
            } else {
                line.to_string()
            }
        }).collect();

        let result = if found {
            lines.join("\n")
        } else {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(new_line);
            content.push('\n');
            content
        };

        Self::write_result(&format!("已成功设置 {} 镜像源为 {}", label, url), backup, fs::write(path, result))
    }

    fn write_npmrc_registry(path: &PathBuf, url: &str) -> ApplySourceResult {
        Self::replace_or_append_line(path, url, "registry", &format!("registry={}", url), "npm")
    }

    fn write_yarnrc_registry(path: &PathBuf, url: &str) -> ApplySourceResult {
        Self::replace_or_append_line(path, url, "registry", &format!("registry \"{}\"", url), "Yarn")
    }

    fn write_pip_index_url(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = if path.exists() {
            fs::read_to_string(path).unwrap_or_default()
        } else {
            "[global]\n".to_string()
        };

        let new_line = format!("index-url = {}", url);
        let mut in_global = false;
        let mut found = false;
        let lines: Vec<String> = content.lines().map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_global = trimmed == "[global]";
                line.to_string()
            } else if in_global && trimmed.starts_with("index-url") {
                found = true;
                new_line.clone()
            } else {
                line.to_string()
            }
        }).collect();

        let mut result = if found {
            lines.join("\n")
        } else {
            let mut new_content = String::new();
            let mut global_found = false;
            for line in content.lines() {
                new_content.push_str(line);
                new_content.push('\n');
                if line.trim() == "[global]" {
                    global_found = true;
                    new_content.push_str(&new_line);
                    new_content.push('\n');
                }
            }
            if !global_found {
                new_content.push_str("[global]\n");
                new_content.push_str(&new_line);
                new_content.push('\n');
            }
            new_content
        };

        if !result.ends_with('\n') {
            result.push('\n');
        }

        Self::write_result(&format!("已成功设置 pip 镜像源为 {}", url), backup, fs::write(path, result))
    }

    fn write_uv_index_url(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = if path.exists() {
            fs::read_to_string(path).unwrap_or_default()
        } else {
            String::new()
        };

        let mut value: toml::Value = if content.is_empty() {
            toml::Value::Table(toml::map::Map::new())
        } else {
            toml::from_str(&content).unwrap_or(toml::Value::Table(toml::map::Map::new()))
        };

        if value.as_table_mut().is_none() {
            value = toml::Value::Table(toml::map::Map::new());
        }

        let table = value.as_table_mut().unwrap();
        if !table.contains_key("pip") {
            table.insert("pip".to_string(), toml::Value::Table(toml::map::Map::new()));
        }
        if let Some(pip) = table.get_mut("pip").and_then(|v| v.as_table_mut()) {
            pip.insert("index-url".to_string(), toml::Value::String(url.to_string()));
        }

        let result = toml::to_string_pretty(&value).unwrap_or_default();
        Self::write_result(&format!("已成功设置 uv 镜像源为 {}", url), backup, fs::write(path, result))
    }

    fn write_go_env_proxy(url: &str) -> ApplySourceResult {
        let output = silent_command("go")
            .args(["env", "-w", &format!("GOPROXY={}", url)])
            .output();

        match output {
            Ok(o) if o.status.success() => ApplySourceResult {
                success: true,
                message: format!("已成功设置 Go 代理为 {}", url),
                backup_path: None,
            },
            Ok(o) => ApplySourceResult {
                success: false,
                message: format!("设置失败: {}", String::from_utf8_lossy(&o.stderr)),
                backup_path: None,
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("执行 go 命令失败: {}", e),
                backup_path: None,
            },
        }
    }

    fn write_maven_mirror(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<settings xmlns="http://maven.apache.org/SETTINGS/1.2.0"
          xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
          xsi:schemaLocation="http://maven.apache.org/SETTINGS/1.2.0 https://maven.apache.org/xsd/settings-1.2.0.xsd">
  <mirrors>
    <mirror>
      <id>mirrorpilot-mirror</id>
      <name>MirrorPilot Mirror</name>
      <url>{}</url>
      <mirrorOf>central</mirrorOf>
    </mirror>
  </mirrors>
</settings>
"#, url);

        Self::write_result(&format!("已成功设置 Maven 镜像源为 {}", url), backup, fs::write(path, content))
    }

    fn write_gradle_repo(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = format!(
            r#"allprojects {{
    repositories {{
        maven {{ url '{}' }}
        mavenCentral()
    }}
}}
"#, url);

        Self::write_result(&format!("已成功设置 Gradle 镜像源为 {}", url), backup, fs::write(path, content))
    }

    fn write_docker_mirror(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let mut config: serde_json::Value = if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        if let Some(obj) = config.as_object_mut() {
            obj.insert("registry-mirrors".to_string(), serde_json::json!([url]));
        }

        let result = serde_json::to_string_pretty(&config).unwrap_or_default();
        match fs::write(path, result) {
            Ok(_) => ApplySourceResult {
                success: true,
                message: "已成功设置 Docker 镜像源，重启 Docker 后生效".to_string(),
                backup_path: backup.map(|p| p.to_string_lossy().to_string()),
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("写入失败: {}（可能需要管理员权限）", e),
                backup_path: None,
            },
        }
    }

    fn write_cargo_source(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = if let Some(sparse_url) = url.strip_prefix("sparse+") {
            format!(
                r#"[source.crates-io]
replace-with = 'mirrorpilot'

[source.mirrorpilot]
sparse-registry = '{}'
"#, sparse_url)
        } else {
            format!(
                r#"[source.crates-io]
replace-with = 'mirrorpilot'

[source.mirrorpilot]
registry = '{}'
"#, url)
        };

        Self::write_result(&format!("已成功设置 Cargo 镜像源为 {}", url), backup, fs::write(path, content))
    }

    fn write_nuget_source(path: &PathBuf, url: &str) -> ApplySourceResult {
        let backup = Self::backup_file(path);
        Self::ensure_parent_dir(path);

        let content = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <clear />
    <add key="MirrorPilot" value="{}" />
  </packageSources>
</configuration>
"#, url);

        Self::write_result(&format!("已成功设置 NuGet 镜像源为 {}", url), backup, fs::write(path, content))
    }

    // ponytail: dotnet nuget remove source + add source，用子命令而非手写 XML
    fn write_dotnet_nuget_source(url: &str) -> ApplySourceResult {
        // 先移除旧的 MirrorPilot 源（忽略失败）
        let _ = silent_command("dotnet")
            .args(["nuget", "remove", "source", "MirrorPilot"])
            .output();

        let output = silent_command("dotnet")
            .args(["nuget", "add", "source", url, "-n", "MirrorPilot"])
            .output();

        match output {
            Ok(o) if o.status.success() => ApplySourceResult {
                success: true,
                message: format!("已成功设置 .NET NuGet 镜像源为 {}", url),
                backup_path: None,
            },
            Ok(o) => ApplySourceResult {
                success: false,
                message: format!("设置失败: {}", String::from_utf8_lossy(&o.stderr)),
                backup_path: None,
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("执行 dotnet 命令失败: {}", e),
                backup_path: None,
            },
        }
    }

    // ponytail: winget source add — 用子命令管理
    fn write_winget_source(url: &str) -> ApplySourceResult {
        // 先移除旧源（忽略失败）
        let _ = silent_command("winget")
            .args(["source", "remove", "--name", "MirrorPilot"])
            .output();

        let output = silent_command("winget")
            .args(["source", "add", "--name", "MirrorPilot", "--arg", url])
            .output();

        match output {
            Ok(o) if o.status.success() => ApplySourceResult {
                success: true,
                message: format!("已成功设置 WinGet 镜像源为 {}", url),
                backup_path: None,
            },
            Ok(o) => ApplySourceResult {
                success: false,
                message: format!("设置失败: {}", String::from_utf8_lossy(&o.stderr)),
                backup_path: None,
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("执行 winget 命令失败: {}", e),
                backup_path: None,
            },
        }
    }

    // ponytail: Rustup 镜像写入 settings.toml
    fn write_rustup_mirror(url: &str) -> ApplySourceResult {
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return Self::no_path_result(),
        };
        let settings_path = home.join(".rustup").join("settings.toml");
        let _ = fs::create_dir_all(settings_path.parent().unwrap_or_else(|| std::path::Path::new(".")));

        let mut content = if settings_path.exists() {
            fs::read_to_string(&settings_path).unwrap_or_default()
        } else {
            String::new()
        };

        let new_line = format!("dist_server = \"{}\"", url);
        let mut found = false;
        let lines: Vec<String> = content.lines().map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("dist_server") {
                found = true;
                new_line.clone()
            } else {
                line.to_string()
            }
        }).collect();

        let result = if found {
            lines.join("\n")
        } else {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&new_line);
            content.push('\n');
            content
        };

        match fs::write(&settings_path, result) {
            Ok(_) => ApplySourceResult {
                success: true,
                message: format!("已成功设置 Rustup 镜像为 {}（重启终端生效）", url),
                backup_path: None,
            },
            Err(e) => ApplySourceResult {
                success: false,
                message: format!("写入失败: {}", e),
                backup_path: None,
            },
        }
    }
}
