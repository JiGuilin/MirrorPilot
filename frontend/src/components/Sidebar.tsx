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
import type { PackageManagerStatus } from "../lib/api";

type Tab = "sources" | "cache";

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
    { id: "sources", label: "源管理", icon: <Package size={16} /> },
    { id: "cache", label: "本地缓存", icon: <HardDrive size={16} /> },
  ];

  const renderPmItem = (pm: PackageManagerInfo) => {
    const status = getStatusForPm(pm.id);
    const isActive = selectedPm === pm.id;

    return (
      <button
        key={pm.id}
        onClick={() => onSelectPm(pm.id)}
        className={cn(
          "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm transition-all duration-150",
          isActive
            ? "bg-blue-600/20 text-blue-400 border border-blue-500/30"
            : "text-slate-300 hover:bg-slate-700/50 hover:text-white border border-transparent"
        )}
      >
        <span className="text-base flex-shrink-0">{pm.icon}</span>
        <span className="flex-1 text-left font-medium">{pm.displayName}</span>
        {status?.installed && (
          <Check size={12} className="text-green-400 flex-shrink-0" />
        )}
        {status?.current_source_url && (
          <span className="w-1.5 h-1.5 rounded-full bg-blue-400 flex-shrink-0" />
        )}
      </button>
    );
  };

  return (
    <div className="w-60 h-full bg-slate-900/80 border-r border-slate-700/50 flex flex-col">
      {/* Logo */}
      <div
        className="px-4 py-4 border-b border-slate-700/50"
        data-tauri-drag-region
      >
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-sm">
            M
          </div>
          <div>
            <h1 className="text-base font-bold text-white">MirrorPilot</h1>
            <p className="text-[10px] text-slate-400">镜像领航员</p>
          </div>
        </div>
      </div>

      {/* Tab 选择 */}
      <div className="px-3 py-3 border-b border-slate-700/50 flex gap-1">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => onSelectTab(tab.id)}
            className={cn(
              "flex-1 flex flex-col items-center gap-1 px-2 py-1.5 rounded-md text-xs transition-all",
              selectedTab === tab.id
                ? "bg-blue-600/20 text-blue-400"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-700/50"
            )}
          >
            {tab.icon}
            <span>{tab.label}</span>
          </button>
        ))}
      </div>

      {/* 包管理器列表 */}
      <div className="flex-1 overflow-y-auto px-3 py-2">
        {installedPms.length > 0 && (
          <>
            <button
              onClick={() => setExpanded(!expanded)}
              className="w-full flex items-center justify-between px-2 py-1 text-xs text-slate-400 hover:text-slate-200"
            >
              <span>已安装 ({installedPms.length})</span>
              <ChevronDown
                size={12}
                className={cn(
                  "transition-transform",
                  expanded ? "rotate-0" : "-rotate-90"
                )}
              />
            </button>
            {expanded && (
              <div className="mt-1 space-y-0.5">
                {installedPms.map(renderPmItem)}
              </div>
            )}
          </>
        )}

        {notInstalledPms.length > 0 && (
          <div className="mt-4">
            <p className="px-2 py-1 text-xs text-slate-500">
              未安装 ({notInstalledPms.length})
            </p>
            <div className="mt-1 space-y-0.5">
              {notInstalledPms.map(renderPmItem)}
            </div>
          </div>
        )}
      </div>

      {/* 底部按钮 */}
      <div className="px-3 py-3 border-t border-slate-700/50 flex items-center gap-2">
        <button className="flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-md text-xs text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-all">
          <Settings size={14} />
          设置
        </button>
        <button className="flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-md text-xs text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-all">
          <Info size={14} />
        </button>
      </div>
    </div>
  );
}
