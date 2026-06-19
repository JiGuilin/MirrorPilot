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

/// 应用状态
struct AppState {
    registry: SourceRegistry,
}

/// 获取所有包管理器状态
#[tauri::command]
fn get_package_managers() -> Vec<PackageManagerStatus> {
    PlatformDetector::detect_all()
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

/// 快速测延迟
#[tauri::command]
async fn test_latency(url: String) -> Result<f64, String> {
    let tester = NetworkTester::new(10);
    tester.test_latency(&url).await
}

/// 获取应用配置
#[tauri::command]
fn get_app_config(state: State<AppState>) -> AppConfig {
    let auto_test = state
        .registry
        .get_config("auto_test_on_start")
        .map(|v| v == "true")
        .unwrap_or(false);
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
        auto_test_on_start: auto_test,
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
        .set_config("auto_test_on_start", &config.auto_test_on_start.to_string())?;
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

/// 打开配置文件所在目录（选中文件）
#[tauri::command]
fn open_config_file(package_manager: String) -> Result<String, String> {
    let pm = PackageManager::from_id(&package_manager)
        .ok_or_else(|| format!("未知的包管理器: {}", package_manager))?;
    let path = ConfigManager::get_config_path(&pm)
        .ok_or("该包管理器没有配置文件")?;
    // Ensure file exists so the OS can open it
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap_or_else(|| std::path::Path::new(".")))
            .map_err(|e| format!("创建目录失败: {}", e))?;
        std::fs::write(&path, "")
            .map_err(|e| format!("创建文件失败: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(format!("/select,{}", path.display()))
        .spawn().map_err(|e| format!("打开失败: {}", e))?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("-R")
        .arg(&path)
        .spawn().map_err(|e| format!("打开失败: {}", e))?;
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(path.parent().unwrap_or_else(|| std::path::Path::new(".")))
        .spawn().map_err(|e| format!("打开失败: {}", e))?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = SourceRegistry::new().expect("无法初始化数据库");

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
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
            test_latency,
            get_app_config,
            save_app_config,
            get_current_source,
            open_config_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
