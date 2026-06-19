use std::fs;
use std::path::PathBuf;

use crate::modules::types::{ApplySourceResult, PackageManager};

/// 配置文件管理器 - 读写各包管理器的配置文件
pub struct ConfigManager;

impl ConfigManager {
    /// 获取指定包管理器的配置文件路径
    pub fn get_config_path(pm: &PackageManager) -> Option<PathBuf> {
        match pm {
            PackageManager::Npm | PackageManager::Pnpm => {
                // 用户级 .npmrc
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
                // go env -w writes to this file
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
            PackageManager::Homebrew => {
                // 环境变量方式，无固定文件
                None
            }
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
            PackageManager::Go => {
                // 通过 go env GOPROXY 读取
                Self::read_go_env_proxy()
            }
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
            PackageManager::Apt | PackageManager::Yum | PackageManager::Homebrew | PackageManager::Chocolatey => {
                None
            }
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
                // yarnrc uses: registry "https://..."  or  registry: https://...  or  registry=https://...
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

    fn read_go_env_proxy() -> Option<String> {
        let output = if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(["/C", "go", "env", "GOPROXY"])
                .output()
                .ok()?
        } else {
            std::process::Command::new("go")
                .args(["env", "GOPROXY"])
                .output()
                .ok()?
        };
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
        // 简单解析 XML 中的 mirror url
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

        // Prefer sparse-registry, fall back to registry
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

    /// Replace or append a line matching `prefix` in a key-value text file (npmrc/yarnrc style)
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
        let output = if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(["/C", "go", "env", "-w", &format!("GOPROXY={}", url)])
                .output()
        } else {
            std::process::Command::new("go")
                .args(["env", "-w", &format!("GOPROXY={}", url)])
                .output()
        };

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
        // ponytail: Docker 写入可能因权限失败，保留单独的错误提示
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

        // Support sparse+https://... or plain https://... URLs
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
}
