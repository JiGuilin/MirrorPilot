export interface Source {
  id: string;
  name: string;
  url: string;
  package_manager: string;
  is_builtin: boolean;
  is_custom: boolean;
  region: string;
  status: string;
  latency: number | null;
  speed: number | null;
  last_tested: string | null;
}

export interface PackageManagerStatus {
  package_manager: string;
  display_name: string;
  installed: boolean;
  version: string | null;
  current_source_url: string | null;
  config_path: string | null;
}

export interface SpeedTestResult {
  source_id: string;
  source_name: string;
  source_url: string;
  latency_ms: number | null;
  speed_kbps: number | null;
  success: boolean;
  error_message: string | null;
}

export interface SpeedTestProgress {
  current: number;
  total: number;
  current_source_name: string;
  results: SpeedTestResult[];
}

export interface CacheInfo {
  package_manager: string;
  path: string;
  size_bytes: number;
  file_count: number;
  exists: boolean;
}

export interface ApplySourceResult {
  success: boolean;
  message: string;
  backup_path: string | null;
}

export interface AppConfig {
  auto_test_on_start: boolean;
  test_timeout_seconds: number;
  max_concurrent_tests: number;
  theme: string;
  language: string;
}

export interface PackageManagerInfo {
  id: string;
  displayName: string;
  icon: string;
  color: string;
}

export const PACKAGE_MANAGERS: PackageManagerInfo[] = [
  { id: "npm", displayName: "npm", icon: "📦", color: "#CB3837" },
  { id: "yarn", displayName: "Yarn", icon: "🧶", color: "#2C8EBB" },
  { id: "pnpm", displayName: "pnpm", icon: "⚡", color: "#F69220" },
  { id: "pip", displayName: "pip", icon: "🐍", color: "#3776AB" },
  { id: "uv", displayName: "uv", icon: "⚡", color: "#DE5FE9" },
  { id: "go", displayName: "Go", icon: "🔵", color: "#00ADD8" },
  { id: "maven", displayName: "Maven", icon: "🏗️", color: "#C71A36" },
  { id: "gradle", displayName: "Gradle", icon: "🐘", color: "#02303A" },
  { id: "docker", displayName: "Docker", icon: "🐳", color: "#2496ED" },
  { id: "cargo", displayName: "Cargo", icon: "🦀", color: "#DEA584" },
  { id: "nuget", displayName: "NuGet", icon: "📋", color: "#004880" },
  { id: "chocolatey", displayName: "Chocolatey", icon: "🍫", color: "#80B5E3" },
];
