import { useState, useEffect } from "react";
import {
  HardDrive,
  RefreshCw,
  Trash2,
  AlertTriangle,
} from "lucide-react";
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from "recharts";
import { cn, formatBytes } from "../lib/utils";
import { scanCaches, cleanCache } from "../lib/api";
import type { CacheInfo } from "../types";
import { PACKAGE_MANAGERS } from "../types";

/* Linear palette — muted lavender range */
const COLORS = [
  "#5e6ad2",
  "#828fff",
  "#7a7fad",
  "#6b72c4",
  "#9ba3e8",
  "#4e55b8",
  "#a8adf0",
  "#6870cc",
  "#b3b7f5",
  "#5560d6",
  "#c4c7fa",
  "#3f4ba8",
];

interface CacheManagerProps {
  selectedPm: string;
}

export function CacheManager(_: CacheManagerProps) {
  const [caches, setCaches] = useState<CacheInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [cleaningPm, setCleaningPm] = useState<string | null>(null);
  const [confirmClean, setConfirmClean] = useState<string | null>(null);
  const [confirmCleanAll, setConfirmCleanAll] = useState(false);
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  const loadCaches = async () => {
    setLoading(true);
    try {
      const data = await scanCaches();
      setCaches(data);
    } catch (e) { console.error(e); }
    setLoading(false);
  };

  useEffect(() => { loadCaches(); }, []);

  const handleClean = async (pm: string) => {
    setCleaningPm(pm);
    setMessage(null);
    try {
      const result = await cleanCache(pm);
      setMessage({ type: "success", text: result });
      loadCaches();
    } catch (_e: unknown) {
      setMessage({ type: "error", text: String(_e) });
    }
    setCleaningPm(null);
    setConfirmClean(null);
  };

  const existingCaches = caches.filter((c) => c.exists);

  const handleCleanAll = async () => {
    setConfirmCleanAll(false);
    for (const cache of existingCaches) {
      setCleaningPm(cache.package_manager);
        try { await cleanCache(cache.package_manager); } catch { /* continue */ }
    }
    setCleaningPm(null);
    setMessage({ type: "success", text: `已清理 ${existingCaches.length} 个缓存` });
    loadCaches();
  };

  const totalSize = caches.filter((c) => c.exists).reduce((sum, c) => sum + c.size_bytes, 0);

  const chartData = caches
    .filter((c) => c.exists && c.size_bytes > 0)
    .map((c) => {
      const pmInfo = PACKAGE_MANAGERS.find((p) => p.id === c.package_manager);
      return { name: pmInfo?.displayName || c.package_manager, value: c.size_bytes };
    });

  return (
    <div className="h-full flex flex-col">
      {/* 顶栏 */}
      <div className="px-6 py-3.5 border-b border-hairline bg-surface-1">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2.5">
            <HardDrive size={16} className="text-accent" />
            <h2 className="text-[13px] font-semibold text-ink tracking-tight">本地缓存管理</h2>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[12px] text-ink-subtle">
              总计: <span className="text-ink font-medium">{formatBytes(totalSize)}</span>
            </span>
            {existingCaches.length > 1 && (
              confirmCleanAll ? (
                <div className="flex items-center gap-1">
                  <button
                    onClick={handleCleanAll}
                    className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] bg-lv-danger text-ink hover:bg-lv-danger/80 transition-colors"
                  >
                    <AlertTriangle size={11} />
                    确认清理全部
                  </button>
                  <button
                    onClick={() => setConfirmCleanAll(false)}
                    className="px-2 py-1 rounded-md text-[11px] text-ink-subtle hover:text-ink-muted"
                  >
                    取消
                  </button>
                </div>
              ) : (
                <button
                  onClick={() => setConfirmCleanAll(true)}
                  className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] text-lv-danger hover:bg-lv-danger/10 transition-colors"
                >
                  <Trash2 size={11} />
                  清理全部
                </button>
              )
            )}
            <button
              onClick={loadCaches}
              className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] text-ink-subtle hover:text-ink-muted hover:bg-surface-2"
            >
              <RefreshCw size={11} className={loading ? "animate-spin" : ""} />
            </button>
          </div>
        </div>

        {message && (
          <div
            className={cn(
              "mt-2 flex items-center gap-2 px-3 py-2 rounded-md text-[12px]",
              message.type === "success" ? "bg-lv-success/10 text-lv-success" : "bg-lv-danger/10 text-lv-danger"
            )}
          >
            {message.text}
            <button onClick={() => setMessage(null)} className="ml-auto opacity-60 hover:opacity-100">×</button>
          </div>
        )}
      </div>

      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="grid grid-cols-3 gap-3">
          {/* 饼图 */}
          <div className="col-span-1 bg-surface-1 rounded-lg border border-hairline p-4">
            <h3 className="text-[11px] font-medium text-ink-subtle mb-2">缓存占比</h3>
            {chartData.length > 0 ? (
              <ResponsiveContainer width="100%" height={180}>
                <PieChart>
                  <Pie
                    data={chartData}
                    cx="50%"
                    cy="50%"
                    innerRadius={45}
                    outerRadius={72}
                    paddingAngle={2}
                    dataKey="value"
                  >
                    {chartData.map((_, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip
                    formatter={(value: number) => formatBytes(value)}
                    contentStyle={{
                      backgroundColor: "#141516",
                      border: "1px solid #34343a",
                      borderRadius: "8px",
                      color: "#f7f8f8",
                      fontSize: "11px",
                    }}
                  />
                </PieChart>
              </ResponsiveContainer>
            ) : (
              <div className="h-[180px] flex items-center justify-center text-ink-tertiary text-[12px]">
                暂无缓存数据
              </div>
            )}
          </div>

          {/* 列表 */}
          <div className="col-span-2 space-y-3">
            {caches.map((cache) => {
              const pmInfo = PACKAGE_MANAGERS.find((p) => p.id === cache.package_manager);
              const isCleaning = cleaningPm === cache.package_manager;

              return (
                <div
                  key={cache.package_manager}
                  className={cn(
                    "flex items-center gap-3 p-4 rounded-lg border",
                    cache.exists
                      ? "bg-surface-1 border-hairline"
                      : "bg-surface-1/30 border-hairline/50 opacity-40"
                  )}
                >
                  <span className="text-lg flex-shrink-0">{pmInfo?.icon}</span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-1.5">
                      <span className="text-[13px] font-medium text-ink">{pmInfo?.displayName}</span>
                      {!cache.exists && <span className="text-[10px] text-ink-tertiary">未找到缓存</span>}
                    </div>
                    {cache.exists && (
                      <p className="text-[11px] text-ink-subtle font-mono truncate">{cache.path}</p>
                    )}
                  </div>

                  {cache.exists && (
                    <div className="flex items-center gap-3 flex-shrink-0">
                      <div className="text-right">
                        <p className="text-[13px] font-semibold text-ink">{formatBytes(cache.size_bytes)}</p>
                        <p className="text-[10px] text-ink-tertiary">{cache.file_count.toLocaleString()} 个文件</p>
                      </div>

                      {confirmClean === cache.package_manager ? (
                        <div className="flex items-center gap-1">
                          <button
                            onClick={() => handleClean(cache.package_manager)}
                            className="px-2 py-1 rounded-md text-[11px] bg-lv-danger text-ink hover:bg-lv-danger/80 transition-colors"
                          >
                            确认清理
                          </button>
                          <button
                            onClick={() => setConfirmClean(null)}
                            className="px-2 py-1 rounded-md text-[11px] text-ink-subtle hover:text-ink-muted"
                          >
                            取消
                          </button>
                        </div>
                      ) : (
                        <button
                          onClick={() => setConfirmClean(cache.package_manager)}
                          disabled={isCleaning}
                          className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] text-lv-danger hover:bg-lv-danger/10 transition-colors disabled:opacity-50"
                        >
                          {isCleaning ? <RefreshCw size={11} className="animate-spin" /> : <Trash2 size={11} />}
                          清理
                        </button>
                      )}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
