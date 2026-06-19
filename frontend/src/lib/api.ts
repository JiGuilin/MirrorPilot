import { invoke } from "@tauri-apps/api/core";
import type {
  Source,
  PackageManagerStatus,
  CacheInfo,
  ApplySourceResult,
  SpeedTestResult,
  AppConfig,
} from "../types";

export async function getPackageManagers(): Promise<PackageManagerStatus[]> {
  return invoke("get_package_managers");
}

export async function getSources(packageManager: string): Promise<Source[]> {
  return invoke("get_sources", { packageManager });
}

export async function applySource(
  packageManager: string,
  url: string
): Promise<ApplySourceResult> {
  return invoke("apply_source", { packageManager, url });
}

export async function addCustomSource(
  name: string,
  url: string,
  packageManager: string,
  region: string
): Promise<string> {
  return invoke("add_custom_source", { name, url, packageManager, region });
}

export async function deleteCustomSource(id: string): Promise<void> {
  return invoke("delete_custom_source", { id });
}

export async function scanCaches(): Promise<CacheInfo[]> {
  return invoke("scan_caches");
}

export async function cleanCache(
  packageManager: string
): Promise<string> {
  return invoke("clean_cache", { packageManager });
}

export async function testSources(
  sources: Array<[string, string, string]>
): Promise<SpeedTestResult[]> {
  return invoke("test_sources", { sources });
}

export async function testLatency(url: string): Promise<number> {
  return invoke("test_latency", { url });
}

export async function getAppConfig(): Promise<AppConfig> {
  return invoke("get_app_config");
}

export async function saveAppConfig(config: AppConfig): Promise<void> {
  return invoke("save_app_config", { config });
}

export async function getCurrentSource(
  packageManager: string
): Promise<string | null> {
  return invoke("get_current_source", { packageManager });
}

export async function openConfigFile(
  packageManager: string
): Promise<string> {
  return invoke("open_config_file", { packageManager });
}

export async function exportConfig(): Promise<string> {
  return invoke("export_config");
}

export async function importConfig(jsonStr: string): Promise<string> {
  return invoke("import_config", { jsonStr });
}
