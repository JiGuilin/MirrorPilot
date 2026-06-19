import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

export function formatLatency(ms: number | null): string {
  if (ms === null) return "-";
  if (ms < 1) return "<1ms";
  return Math.round(ms) + "ms";
}

export function formatSpeed(kbps: number | null): string {
  if (kbps === null) return "-";
  if (kbps < 1024) return Math.round(kbps) + " KB/s";
  return (kbps / 1024).toFixed(2) + " MB/s";
}

export function getLatencyColor(ms: number | null): string {
  if (ms === null) return "text-slate-400";
  if (ms < 100) return "text-green-400";
  if (ms < 500) return "text-yellow-400";
  return "text-red-400";
}

export function getRegionLabel(region: string): string {
  const labels: Record<string, string> = {
    cn: "中国",
    us: "美国",
    eu: "欧洲",
    custom: "自定义",
  };
  return labels[region] || region;
}

export function getRegionFlagClass(region: string): string {
  const classes: Record<string, string> = {
    cn: "fi fi-cn",
    us: "fi fi-us",
    eu: "fi fi-eu",
    custom: "",
  };
  return classes[region] || "fi fi-un";
}
