use tauri::{AppHandle, State};
use uuid::Uuid;

mod modules;

use modules::{
    config_manager::ConfigManager, file_scanner::FileScanner,
    network_tester::NetworkTester, platform_detector::PlatformDetector,
    source_registry::SourceRegistry,
};

use modules::types::{
    AppConfig, ApplySourceResult, CacheInfo, PackageManager, PackageManagerStatus, Source,
    SpeedTestResult,
};
use serde_json::json;

/// 应用状态
struct AppState {
    registry: SourceRegistry,
}

/// 获取所有包管理器状态（并行检测）
#[tauri::command]
async fn get_package_managers() -> Vec<PackageManagerStatus> {
    PlatformDetector::detect_all_parallel().await
}

/// 获取指定包管理器的源列表
#[tauri::command]
fn get_sources(state: State<AppState>, package_manager: String) -> Vec<Source> {
    state.registry.get_sources(&package_manager)
}

/// 应用源地址
#[tauri::command]
fn apply_source(package_manager: String, url: String) -> ApplySourceResult {
    match PackageManager::from_id(&package_manager) {
        Some(pm) => ConfigManager::apply_source(&pm, &url),
        None => ApplySourceResult {
            success: false,
            message: format!("未知的包管理器: {}", package_manager),
            backup_path: None,
        },
    }
}

/// 添加自定义源
#[tauri::command]
fn add_custom_source(state: State<AppState>, name: String, url: String, package_manager: String, region: String) -> Result<String, String> {
    let source = Source {
        id: Uuid::new_v4().to_string(),
        name,
        url,
        package_manager,
        is_builtin: false,
        is_custom: true,
        region,
        status: "active".to_string(),
        latency: None,
        speed: None,
        last_tested: None,
    };
    let id = source.id.clone();
    state.registry.add_custom_source(&source)?;
    Ok(id)
}

/// 删除自定义源
#[tauri::command]
fn delete_custom_source(state: State<AppState>, id: String) -> Result<(), String> {
    state.registry.delete_custom_source(&id)
}

/// 扫描缓存
#[tauri::command]
fn scan_caches() -> Vec<CacheInfo> {
    FileScanner::scan_all_caches()
}

// ponytail: scan_cache (single PM) removed, frontend only uses scan_caches

/// 清理缓存
#[tauri::command]
fn clean_cache(package_manager: String) -> Result<String, String> {
    match PackageManager::from_id(&package_manager) {
        Some(pm) => FileScanner::clean_cache(&pm),
        None => Err(format!("未知的包管理器: {}", package_manager)),
    }
}

/// 测速
#[tauri::command]
async fn test_sources(
    app_handle: AppHandle,
    sources: Vec<(String, String, String)>, // (id, name, url)
) -> Vec<SpeedTestResult> {
    let tester = NetworkTester::new(10);
    tester.test_sources(sources, &app_handle).await
}

/// 获取应用配置
#[tauri::command]
fn get_app_config(state: State<AppState>) -> AppConfig {
    let timeout = state
        .registry
        .get_config("test_timeout_seconds")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let max_concurrent = state
        .registry
        .get_config("max_concurrent_tests")
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    let theme = state
        .registry
        .get_config("theme")
        .unwrap_or_else(|| "system".to_string());
    let language = state
        .registry
        .get_config("language")
        .unwrap_or_else(|| "zh-CN".to_string());

    AppConfig {
        test_timeout_seconds: timeout,
        max_concurrent_tests: max_concurrent,
        theme,
        language,
    }
}

/// 保存应用配置
#[tauri::command]
fn save_app_config(state: State<AppState>, config: AppConfig) -> Result<(), String> {
    state
        .registry
        .set_config("test_timeout_seconds", &config.test_timeout_seconds.to_string())?;
    state
        .registry
        .set_config("max_concurrent_tests", &config.max_concurrent_tests.to_string())?;
    state.registry.set_config("theme", &config.theme)?;
    state.registry.set_config("language", &config.language)?;
    Ok(())
}

/// 获取当前源
#[tauri::command]
fn get_current_source(package_manager: String) -> Option<String> {
    PackageManager::from_id(&package_manager)
        .and_then(|pm| ConfigManager::read_current_source(&pm))
}

// ponytail: 提取跨平台打开逻辑，open_config_file 和 open_folder 共用
// ponytail: 使用 tauri-plugin-opener 的 reveal_item_in_dir / open_url 替代子进程，零弹窗
fn open_in_os(path: &std::path::Path, select: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // ponytail: explorer 是 GUI 程序本不弹窗，但加 CREATE_NO_WINDOW 更保险
        use std::os::windows::process::CommandExt;
        let arg = if select { format!("/select,{}", path.display()) } else { path.display().to_string() };
        std::process::Command::new("explorer")
            .arg(arg)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        if select {
            std::process::Command::new("open").args(["-R", &path.to_string_lossy()]).spawn().map_err(|e| format!("打开失败: {}", e))?;
        } else {
            std::process::Command::new("open").arg(path).spawn().map_err(|e| format!("打开失败: {}", e))?;
        }
    }
    #[cfg(target_os = "linux")]
    {
        let target = if select { path.parent().unwrap_or(path) } else { path };
        std::process::Command::new("xdg-open").arg(target).spawn().map_err(|e| format!("打开失败: {}", e))?;
    }
    Ok(())
}

/// 打开配置文件所在目录（选中文件）
#[tauri::command]
fn open_config_file(package_manager: String) -> Result<String, String> {
    let pm = PackageManager::from_id(&package_manager)
        .ok_or_else(|| format!("未知的包管理器: {}", package_manager))?;
    let path = ConfigManager::get_config_path(&pm)
        .ok_or("该包管理器没有配置文件")?;
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap_or_else(|| std::path::Path::new(".")))
            .map_err(|e| format!("创建目录失败: {}", e))?;
        std::fs::write(&path, "").map_err(|e| format!("创建文件失败: {}", e))?;
    }
    open_in_os(&path, true)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() { return Err(format!("目录不存在: {}", path)); }
    open_in_os(p, false)
}

/// 导出当前所有源配置为 JSON
#[tauri::command]
fn export_config(state: State<AppState>) -> Result<String, String> {
    let pms = [
        PackageManager::Npm, PackageManager::Yarn, PackageManager::Pnpm,
        PackageManager::Pip, PackageManager::Uv, PackageManager::Go,
        PackageManager::Maven, PackageManager::Gradle, PackageManager::Docker,
        PackageManager::Cargo, PackageManager::NuGet, PackageManager::DotNet,
        PackageManager::Winget, PackageManager::Rustup,
    ];
    let mut map = serde_json::Map::new();
    for pm in &pms {
        let id = pm.id().to_string();
        let current = ConfigManager::read_current_source(pm);
        let sources = state.registry.get_sources(&id);
        map.insert(id, json!({
            "current_source": current,
            "sources": sources.iter().map(|s| json!({
                "name": s.name, "url": s.url, "region": s.region, "is_custom": s.is_custom
            })).collect::<Vec<_>>()
        }));
    }
    serde_json::to_string_pretty(&json!(map)).map_err(|e| format!("导出失败: {}", e))
}

/// 从 JSON 导入源配置
#[tauri::command]
fn import_config(state: State<AppState>, json_str: String) -> Result<String, String> {
    let data: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON 解析失败: {}", e))?;
    let mut applied = 0;
    if let Some(obj) = data.as_object() {
        for (pm_id, val) in obj {
            if let Some(url) = val.get("current_source").and_then(|v| v.as_str()) {
                if !url.is_empty() {
                    if let Some(pm) = PackageManager::from_id(pm_id) {
                        let result = ConfigManager::apply_source(&pm, url);
                        if result.success { applied += 1; }
                    }
                }
            }
            // Import custom sources
            if let Some(sources) = val.get("sources").and_then(|v| v.as_array()) {
                for src in sources {
                    if src.get("is_custom").and_then(|v| v.as_bool()).unwrap_or(false) {
                        let name = src.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let url = src.get("url").and_then(|v| v.as_str()).unwrap_or("");
                        let region = src.get("region").and_then(|v| v.as_str()).unwrap_or("custom");
                        if !name.is_empty() && !url.is_empty() {
                            let source = Source {
                                id: Uuid::new_v4().to_string(),
                                name: name.to_string(),
                                url: url.to_string(),
                                package_manager: pm_id.clone(),
                                is_builtin: false,
                                is_custom: true,
                                region: region.to_string(),
                                status: "active".to_string(),
                                latency: None,
                                speed: None,
                                last_tested: None,
                            };
                            let _ = state.registry.add_custom_source(&source);
                        }
                    }
                }
            }
        }
    }
    Ok(format!("已导入 {} 个源配置", applied))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = SourceRegistry::new().expect("无法初始化数据库");

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(AppState { registry })
        .invoke_handler(tauri::generate_handler![
            get_package_managers,
            get_sources,
            apply_source,
            add_custom_source,
            delete_custom_source,
            scan_caches,
            clean_cache,
            test_sources,
            get_app_config,
            save_app_config,
            get_current_source,
            open_config_file,
            open_folder,
            export_config,
            import_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
