use serde::{Deserialize, Serialize};

/// 包管理器类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Pip,
    Uv,
    Go,
    Maven,
    Gradle,
    Docker,
    Apt,
    Yum,
    Homebrew,
    Cargo,
    NuGet,
    Chocolatey,
}

impl PackageManager {
    pub fn id(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Pip => "pip",
            PackageManager::Uv => "uv",
            PackageManager::Go => "go",
            PackageManager::Maven => "maven",
            PackageManager::Gradle => "gradle",
            PackageManager::Docker => "docker",
            PackageManager::Apt => "apt",
            PackageManager::Yum => "yum",
            PackageManager::Homebrew => "homebrew",
            PackageManager::Cargo => "cargo",
            PackageManager::NuGet => "nuget",
            PackageManager::Chocolatey => "chocolatey",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "Yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Pip => "pip",
            PackageManager::Uv => "uv",
            PackageManager::Go => "Go",
            PackageManager::Maven => "Maven",
            PackageManager::Gradle => "Gradle",
            PackageManager::Docker => "Docker",
            PackageManager::Apt => "apt",
            PackageManager::Yum => "yum/dnf",
            PackageManager::Homebrew => "Homebrew",
            PackageManager::Cargo => "Cargo",
            PackageManager::NuGet => "NuGet",
            PackageManager::Chocolatey => "Chocolatey",
        }
    }

    // ponytail: icon() was identical to id(), removed

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "npm" => Some(PackageManager::Npm),
            "yarn" => Some(PackageManager::Yarn),
            "pnpm" => Some(PackageManager::Pnpm),
            "pip" => Some(PackageManager::Pip),
            "uv" => Some(PackageManager::Uv),
            "go" => Some(PackageManager::Go),
            "maven" => Some(PackageManager::Maven),
            "gradle" => Some(PackageManager::Gradle),
            "docker" => Some(PackageManager::Docker),
            "apt" => Some(PackageManager::Apt),
            "yum" => Some(PackageManager::Yum),
            "homebrew" => Some(PackageManager::Homebrew),
            "cargo" => Some(PackageManager::Cargo),
            "nuget" => Some(PackageManager::NuGet),
            "chocolatey" => Some(PackageManager::Chocolatey),
            _ => None,
        }
    }

    /// 该包管理器支持的平台
    pub fn supported_platforms(&self) -> Vec<Platform> {
        match self {
            PackageManager::Apt | PackageManager::Yum => vec![Platform::Linux],
            PackageManager::Homebrew => vec![Platform::MacOS, Platform::Linux],
            PackageManager::NuGet | PackageManager::Chocolatey => vec![Platform::Windows],
            _ => vec![Platform::Windows, Platform::MacOS, Platform::Linux],
        }
    }
}

/// 运行平台
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
}

/// 源地址
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub url: String,
    pub package_manager: String,
    pub is_builtin: bool,
    pub is_custom: bool,
    pub region: String,
    pub status: String,
    pub latency: Option<f64>,
    pub speed: Option<f64>,
    pub last_tested: Option<String>,
}

/// 测速结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub source_id: String,
    pub source_name: String,
    pub source_url: String,
    pub latency_ms: Option<f64>,
    pub speed_kbps: Option<f64>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// 测速进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestProgress {
    pub current: usize,
    pub total: usize,
    pub current_source_name: String,
    pub results: Vec<SpeedTestResult>,
}

/// 缓存扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub package_manager: String,
    pub path: String,
    pub size_bytes: u64,
    pub file_count: u64,
    pub exists: bool,
}

/// 包管理器检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerStatus {
    pub package_manager: String,
    pub display_name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub current_source_url: Option<String>,
    pub config_path: Option<String>,
}

/// 应用源的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplySourceResult {
    pub success: bool,
    pub message: String,
    pub backup_path: Option<String>,
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub auto_test_on_start: bool,
    pub test_timeout_seconds: u64,
    pub max_concurrent_tests: usize,
    pub theme: String,
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            auto_test_on_start: false,
            test_timeout_seconds: 10,
            max_concurrent_tests: 5,
            theme: "system".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}
