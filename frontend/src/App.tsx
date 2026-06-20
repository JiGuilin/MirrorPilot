import { useState, useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { SourceManager } from "./components/SourceManager";
import { CacheManager } from "./components/CacheManager";
import { SettingsPage } from "./components/SettingsPage";
import { AboutPage } from "./components/AboutPage";
import { getPackageManagers, getAppConfig } from "./lib/api";
import { applyTheme } from "./lib/utils";
import type { PackageManagerStatus } from "./types";
import { PACKAGE_MANAGERS } from "./types";

type Tab = "sources" | "cache" | "settings" | "about";

// ponytail: 提取到组件外，只计算一次
const INITIAL_STATUSES: PackageManagerStatus[] = PACKAGE_MANAGERS.map((pm) => ({
  package_manager: pm.id,
  display_name: pm.displayName,
  installed: false,
  version: null,
  current_source_url: null,
  config_path: null,
}));

function App() {
  const [selectedPm, setSelectedPm] = useState(PACKAGE_MANAGERS[0].id);
  const [selectedTab, setSelectedTab] = useState<Tab>("sources");
  const [pmStatuses, setPmStatuses] = useState<PackageManagerStatus[]>(INITIAL_STATUSES);
  const [detecting, setDetecting] = useState(true);

  // 加载包管理器状态
  const refreshStatuses = () => {
    setDetecting(true);
    getPackageManagers()
      .then((statuses) => {
        setPmStatuses(statuses);
        // ponytail: 只在用户还没手动选过时自动切到第一个已安装的
        setSelectedPm((prev) => {
          const firstInstalled = statuses.find((s) => s.installed);
          if (firstInstalled && prev === PACKAGE_MANAGERS[0].id) return firstInstalled.package_manager;
          return prev;
        });
      })
      .catch(console.error)
      .finally(() => setDetecting(false));
  };

  useEffect(() => {
    // 启动时立即应用主题，避免闪烁
    getAppConfig()
      .then((cfg) => applyTheme(cfg.theme))
      .catch(() => {});

    // 异步加载检测结果（侧边栏已经用静态列表渲染了）
    refreshStatuses();
  }, []);

  // ponytail: 只在 system 模式时响应系统主题变化
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: light)");
    const handler = () => {
      if (document.documentElement.dataset.theme === "system") applyTheme("system");
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  const currentPmStatus = pmStatuses.find(
    (s) => s.package_manager === selectedPm
  );
  const pmInfo = PACKAGE_MANAGERS.find((p) => p.id === selectedPm);

  const renderContent = () => {
    switch (selectedTab) {
      case "sources":
        return (
          <SourceManager
            selectedPm={selectedPm}
            pmStatus={currentPmStatus || null}
            onSourceApplied={refreshStatuses}
          />
        );
      case "cache":
        return <CacheManager />;
      case "settings":
        return <SettingsPage />;
      case "about":
        return <AboutPage />;
      default:
        return null;
    }
  };

  return (
    <div className="h-full flex bg-canvas">
      <Sidebar
        selectedPm={selectedPm}
        onSelectPm={setSelectedPm}
        selectedTab={selectedTab}
        onSelectTab={setSelectedTab}
        pmStatuses={pmStatuses}
        detecting={detecting}
      />
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* 顶部标题栏 */}
        {selectedTab === "sources" ? (
          <header
            className="flex items-center justify-between px-4 py-1.5 border-b border-hairline bg-surface-1"
            data-tauri-drag-region
          >
            <div className="flex items-center gap-2">
              <span className="text-base">{pmInfo?.icon}</span>
              <span className="text-[13px] font-medium text-ink tracking-tight">
                {pmInfo?.displayName}
              </span>
              {currentPmStatus?.installed && currentPmStatus.version ? (
                <span className="text-[10px] text-ink-tertiary">
                  v{currentPmStatus.version}
                </span>
              ) : detecting ? (
                <span className="text-[10px] text-ink-tertiary">检测中...</span>
              ) : null}
            </div>
            <div className="flex items-center gap-2">
              {currentPmStatus?.current_source_url && (
                <span className="text-[10px] text-ink-tertiary font-mono max-w-64 truncate">
                  {currentPmStatus.current_source_url}
                </span>
              )}
            </div>
          </header>
        ) : null}

        {/* 主内容区 */}
        <div className="flex-1 overflow-hidden">{renderContent()}</div>
      </main>
    </div>
  );
}

export default App;
