import { useState, useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { SourceManager } from "./components/SourceManager";
import { CacheManager } from "./components/CacheManager";
import { SettingsPage } from "./components/SettingsPage";
import { AboutPage } from "./components/AboutPage";
import { getPackageManagers } from "./lib/api";
import type { PackageManagerStatus } from "./types";
import { PACKAGE_MANAGERS } from "./types";

type Tab = "sources" | "cache" | "settings" | "about";

function App() {
  const [selectedPm, setSelectedPm] = useState("npm");
  const [selectedTab, setSelectedTab] = useState<Tab>("sources");
  const [pmStatuses, setPmStatuses] = useState<PackageManagerStatus[]>([]);

  // 加载包管理器状态
  const refreshStatuses = () => {
    getPackageManagers()
      .then(setPmStatuses)
      .catch(console.error);
  };

  useEffect(() => {
    getPackageManagers()
      .then((statuses) => {
        setPmStatuses(statuses);
        const firstInstalled = statuses.find((s) => s.installed);
        if (firstInstalled) {
          setSelectedPm(firstInstalled.package_manager);
        }
      })
      .catch(console.error);
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
        return <CacheManager selectedPm={selectedPm} />;
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
      />
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* 顶部标题栏 */}
        {selectedTab === "sources" || selectedTab === "cache" ? (
          <header
            className="flex items-center justify-between px-6 py-2.5 border-b border-hairline bg-surface-1"
            data-tauri-drag-region
          >
            <div className="flex items-center gap-2">
              <span className="text-base">{pmInfo?.icon}</span>
              <span className="text-[13px] font-medium text-ink tracking-tight">
                {pmInfo?.displayName}
              </span>
              {currentPmStatus?.installed && (
                <span className="text-[10px] text-ink-tertiary">
                  v{currentPmStatus.version}
                </span>
              )}
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
