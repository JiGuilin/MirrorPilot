import { useState } from "react";
import {
  Package,
  HardDrive,
  Settings,
  Info,
  ChevronDown,
  Check,
} from "lucide-react";
import { cn } from "../lib/utils";
import { PACKAGE_MANAGERS, type PackageManagerInfo } from "../types";
import type { PackageManagerStatus } from "../types";

type Tab = "sources" | "cache" | "settings" | "about";

interface SidebarProps {
  selectedPm: string;
  onSelectPm: (id: string) => void;
  selectedTab: Tab;
  onSelectTab: (tab: Tab) => void;
  pmStatuses: PackageManagerStatus[];
}

export function Sidebar({
  selectedPm,
  onSelectPm,
  selectedTab,
  onSelectTab,
  pmStatuses,
}: SidebarProps) {
  const [expanded, setExpanded] = useState(true);

  const getStatusForPm = (id: string) =>
    pmStatuses.find((s) => s.package_manager === id);

  const installedPms = PACKAGE_MANAGERS.filter(
    (pm) => getStatusForPm(pm.id)?.installed
  );
  const notInstalledPms = PACKAGE_MANAGERS.filter(
    (pm) => !getStatusForPm(pm.id)?.installed
  );

  const tabs: { id: Tab; label: string; icon: React.ReactNode }[] = [
    { id: "sources", label: "源管理", icon: <Package size={15} /> },
    { id: "cache", label: "本地缓存", icon: <HardDrive size={15} /> },
  ];

  const bottomTabs: { id: Tab; label: string; icon: React.ReactNode }[] = [
    { id: "settings", label: "设置", icon: <Settings size={15} /> },
    { id: "about", label: "关于", icon: <Info size={15} /> },
  ];

  const renderPmItem = (pm: PackageManagerInfo) => {
    const status = getStatusForPm(pm.id);
    const isActive = selectedPm === pm.id;

    return (
      <button
        key={pm.id}
        onClick={() => onSelectPm(pm.id)}
        className={cn(
          "w-full flex items-center gap-2 px-3 py-2.5 rounded-md text-[13px] transition-all duration-100",
          isActive
            ? "bg-accent/10 text-accent-hover"
            : "text-ink-muted hover:bg-surface-2 hover:text-ink"
        )}
      >
        <span className="text-sm flex-shrink-0">{pm.icon}</span>
        <span className="flex-1 text-left">{pm.displayName}</span>
        {status?.installed && (
          <Check size={11} className="text-lv-success flex-shrink-0" />
        )}
        {status?.current_source_url && (
          <span className="w-1 h-1 rounded-full bg-accent flex-shrink-0" />
        )}
      </button>
    );
  };

  return (
    <div className="w-56 h-full bg-canvas border-r border-hairline flex flex-col">
      {/* Logo */}
      <div
        className="px-4 py-3.5 border-b border-hairline"
        data-tauri-drag-region
      >
        <div className="flex items-center gap-2.5">
          <div className="w-7 h-7 rounded-md bg-gradient-to-br from-accent to-accent-hover flex items-center justify-center text-white font-semibold text-xs">
            M
          </div>
          <div>
            <h1 className="text-[13px] font-semibold text-ink tracking-tight">MirrorPilot</h1>
            <p className="text-[10px] text-ink-tertiary">镜像领航员</p>
          </div>
        </div>
      </div>

      {/* Tab 选择 */}
      <div className="px-2.5 py-2 border-b border-hairline flex gap-0.5">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => onSelectTab(tab.id)}
            className={cn(
              "flex-1 flex items-center justify-center gap-1.5 px-2 py-1.5 rounded-md text-[11px] transition-all",
              selectedTab === tab.id
                ? "bg-surface-2 text-ink"
                : "text-ink-subtle hover:text-ink-muted hover:bg-surface-1"
            )}
          >
            {tab.icon}
            <span>{tab.label}</span>
          </button>
        ))}
      </div>

      {/* 包管理器列表 */}
      <div className="flex-1 overflow-y-auto px-2 py-1.5">
        {installedPms.length > 0 && (
          <>
            <button
              onClick={() => setExpanded(!expanded)}
              className="w-full flex items-center justify-between px-2 py-1 text-[10px] text-ink-tertiary hover:text-ink-subtle"
            >
              <span>已安装 ({installedPms.length})</span>
              <ChevronDown
                size={10}
                className={cn(
                  "transition-transform",
                  expanded ? "rotate-0" : "-rotate-90"
                )}
              />
            </button>
            {expanded && (
              <div className="mt-1 space-y-1.5">
                {installedPms.map(renderPmItem)}
              </div>
            )}
          </>
        )}

        {notInstalledPms.length > 0 && (
          <div className="mt-5">
            <p className="px-2 py-1 text-[10px] text-ink-tertiary">
              未安装 ({notInstalledPms.length})
            </p>
            <div className="mt-1 space-y-1">
              {notInstalledPms.map(renderPmItem)}
            </div>
          </div>
        )}
      </div>

      {/* 底部标签 */}
      <div className="px-2.5 py-1.5 border-t border-hairline flex gap-0.5">
        {bottomTabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => onSelectTab(tab.id)}
            className={cn(
              "flex-1 flex items-center justify-center gap-1.5 px-2 py-1.5 rounded-md text-[11px] transition-all",
              selectedTab === tab.id
                ? "bg-surface-2 text-ink"
                : "text-ink-subtle hover:text-ink-muted hover:bg-surface-1"
            )}
          >
            {tab.icon}
            <span>{tab.label}</span>
          </button>
        ))}
      </div>
    </div>
  );
}
