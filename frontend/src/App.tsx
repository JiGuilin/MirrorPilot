import { useState, useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { SourceManager } from "./components/SourceManager";
import { CacheManager } from "./components/CacheManager";
import { getPackageManagers } from "./lib/api";
import type { PackageManagerStatus } from "./types";
import { PACKAGE_MANAGERS } from "./types";

type Tab = "sources" | "cache";

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
      default:
        return null;
    }
  };

  return (
    <div className="h-full flex bg-slate-900">
      <Sidebar
        selectedPm={selectedPm}
        onSelectPm={setSelectedPm}
        selectedTab={selectedTab}
        onSelectTab={setSelectedTab}
        pmStatuses={pmStatuses}
      />
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* 顶部标题栏 */}
        <header
          className="flex items-center justify-between px-6 py-2.5 border-b border-slate-700/50 bg-slate-900/50"
          data-tauri-drag-region
        >
          <div className="flex items-center gap-2">
            <span className="text-lg">{pmInfo?.icon}</span>
            <span className="text-sm font-medium text-white">
              {pmInfo?.displayName}
            </span>
            {currentPmStatus?.installed && (
              <span className="text-[10px] text-slate-400">
                v{currentPmStatus.version}
              </span>
            )}
          </div>
          <div className="flex items-center gap-2">
            {currentPmStatus?.current_source_url && (
              <span className="text-[10px] text-slate-500 font-mono max-w-64 truncate">
                {currentPmStatus.current_source_url}
              </span>
            )}
          </div>
        </header>

        {/* 主内容区 */}
        <div className="flex-1 overflow-hidden">{renderContent()}</div>
      </main>
    </div>
  );
}

export default App;
