import { useState, useEffect } from "react";
import {
  HardDrive,
  RefreshCw,
  Trash2,
  AlertTriangle,
} from "lucide-react";
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from "recharts";
import { cn, formatBytes } from "../lib/utils";
import { scanCaches, cleanCache, openFolder } from "../lib/api";
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

export function CacheManager() {
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
    })
    .sort((a, b) => b.value - a.value);

  return (
    <div className="h-full flex flex-col">
      {/* 顶栏 */}
      <div className="px-4 py-2 border-b border-hairline bg-surface-1">
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
              disabled={loading}
              className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-[12px] text-ink-subtle hover:text-ink-muted hover:bg-surface-2 transition-colors disabled:opacity-50"
            >
              <RefreshCw size={14} className={loading ? "animate-spin" : ""} />
              {loading ? "刷新中" : "刷新"}
            </button>
          </div>
        </div>

        {message && (
          <div
            className={cn(
              "mt-1.5 flex items-center gap-2 px-3 py-1.5 rounded-md text-[12px]",
              message.type === "success" ? "bg-lv-success/10 text-lv-success" : "bg-lv-danger/10 text-lv-danger"
            )}
          >
            {message.text}
            <button onClick={() => setMessage(null)} className="ml-auto opacity-60 hover:opacity-100">×</button>
          </div>
        )}
      </div>

      <div className="flex-1 px-4 py-2 overflow-hidden">
        {loading && caches.length === 0 ? (
          /* 扫描中 */
          <div className="h-full flex flex-col items-center justify-center gap-3 text-ink-tertiary">
            <RefreshCw size={24} className="animate-spin text-accent" />
            <p className="text-[13px]">正在扫描缓存目录…</p>
            <p className="text-[11px] text-ink-tertiary/60">扫描已安装包管理器的缓存路径与大小</p>
          </div>
        ) : caches.length === 0 ? (
          /* 无数据 */
          <div className="h-full flex flex-col items-center justify-center gap-3 text-ink-tertiary">
            <HardDrive size={24} className="opacity-40" />
            <p className="text-[13px]">未检测到包管理器</p>
            <p className="text-[11px] text-ink-tertiary/60">安装包管理器后即可管理缓存</p>
          </div>
        ) : existingCaches.length === 0 ? (
          /* 有管理器但无缓存 */
          <div className="h-full flex flex-col items-center justify-center gap-3 text-ink-tertiary">
            <HardDrive size={24} className="opacity-40" />
            <p className="text-[13px]">暂无缓存数据</p>
            <p className="text-[11px] text-ink-tertiary/60">使用包管理器安装包后会产生缓存</p>
          </div>
        ) : (
          <div className="flex gap-2 h-full">
            {/* 饼图 - 固定不滚动 */}
            <div className="w-[260px] flex-shrink-0 bg-surface-1 rounded-lg border border-hairline p-3 flex flex-col">
              <h3 className="text-[11px] font-medium text-ink-subtle mb-2">缓存占比</h3>
              {chartData.length > 0 ? (
                <>
                  <div className="relative">
                    <div className="relative z-[1]">
                    <ResponsiveContainer width="100%" height={160}>
                      <PieChart>
                        <Pie
                          data={chartData}
                          cx="50%"
                          cy="50%"
                          innerRadius={48}
                          outerRadius={70}
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
                            backgroundColor: "var(--color-surface-1, #141516)",
                            border: "1px solid var(--color-hairline, #34343a)",
                            borderRadius: "8px",
                            color: "var(--color-ink, #f7f8f8)",
                            fontSize: "11px",
                          }}
                        />
                      </PieChart>
                    </ResponsiveContainer>
                    </div>
                    {/* 中心总大小 */}
                    <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none" style={{ top: 0, zIndex: 0 }}>
                      <span className="text-[14px] font-semibold text-ink">{formatBytes(totalSize)}</span>
                      <span className="text-[9px] text-ink-tertiary">总缓存</span>
                    </div>
                  </div>
                  {/* 图例 */}
                  <div className="mt-2 space-y-1 overflow-y-auto flex-1">
                    {chartData.map((item, index) => (
                      <div key={item.name} className="flex items-center gap-2 text-[11px]">
                        <span
                          className="w-2 h-2 rounded-full flex-shrink-0"
                          style={{ backgroundColor: COLORS[index % COLORS.length] }}
                        />
                        <span className="text-ink-subtle truncate flex-1">{item.name}</span>
                        <span className="text-ink font-medium flex-shrink-0">{formatBytes(item.value)}</span>
                      </div>
                    ))}
                  </div>
                </>
              ) : (
                <div className="h-[160px] flex items-center justify-center text-ink-tertiary text-[12px]">
                  暂无可视化数据
                </div>
              )}
            </div>

            {/* 列表 - 独立滚动 */}
            <div className="flex-1 overflow-y-auto space-y-1.5">
              {[...caches].sort((a, b) => b.size_bytes - a.size_bytes).map((cache) => {
                const pmInfo = PACKAGE_MANAGERS.find((p) => p.id === cache.package_manager);
                const isCleaning = cleaningPm === cache.package_manager;

                return (
                  <div
                    key={cache.package_manager}
                    className={cn(
                      "flex items-center gap-3 p-2.5 rounded-lg border",
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
                        <div className="flex items-center gap-2">
                          <p className="text-[11px] text-ink-subtle font-mono truncate">{cache.path}</p>
                          <button
                            onClick={() => openFolder(cache.path).catch(() => {})}
                            className="flex-shrink-0 text-[10px] text-accent hover:text-accent-hover hover:underline cursor-pointer transition-colors"
                          >
                            打开
                          </button>
                        </div>
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
        )}
      </div>
    </div>
  );
}
