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

const COLORS = [
  "#3b82f6",
  "#8b5cf6",
  "#06b6d4",
  "#22c55e",
  "#eab308",
  "#f97316",
  "#ef4444",
  "#ec4899",
  "#6366f1",
  "#14b8a6",
  "#f59e0b",
  "#84cc16",
];

interface CacheManagerProps {
  selectedPm: string;
}

export function CacheManager({ selectedPm }: CacheManagerProps) {
  const [caches, setCaches] = useState<CacheInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [cleaningPm, setCleaningPm] = useState<string | null>(null);
  const [confirmClean, setConfirmClean] = useState<string | null>(null);
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  useEffect(() => {
    loadCaches();
  }, []);

  const loadCaches = async () => {
    setLoading(true);
    try {
      const data = await scanCaches();
      setCaches(data);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  const handleClean = async (pm: string) => {
    setCleaningPm(pm);
    setMessage(null);
    try {
      const result = await cleanCache(pm);
      setMessage({ type: "success", text: result });
      loadCaches();
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
    setCleaningPm(null);
    setConfirmClean(null);
  };

  const totalSize = caches
    .filter((c) => c.exists)
    .reduce((sum, c) => sum + c.size_bytes, 0);

  const chartData = caches
    .filter((c) => c.exists && c.size_bytes > 0)
    .map((c) => {
      const pmInfo = PACKAGE_MANAGERS.find((p) => p.id === c.package_manager);
      return {
        name: pmInfo?.displayName || c.package_manager,
        value: c.size_bytes,
      };
    });

  return (
    <div className="h-full flex flex-col">
      {/* 顶部 */}
      <div className="px-6 py-4 border-b border-slate-700/50">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <HardDrive size={20} className="text-purple-400" />
            <h2 className="text-lg font-semibold text-white">本地缓存管理</h2>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-sm text-slate-400">
              总计: <span className="text-white font-medium">{formatBytes(totalSize)}</span>
            </span>
            <button
              onClick={loadCaches}
              className="flex items-center gap-1 px-2 py-1 rounded text-xs text-slate-400 hover:text-slate-200 hover:bg-slate-700/50"
            >
              <RefreshCw size={12} className={loading ? "animate-spin" : ""} />
              刷新
            </button>
          </div>
        </div>

        {/* 消息 */}
        {message && (
          <div
            className={cn(
              "mt-2 flex items-center gap-2 px-3 py-2 rounded-md text-sm",
              message.type === "success"
                ? "bg-green-500/10 text-green-400"
                : "bg-red-500/10 text-red-400"
            )}
          >
            {message.text}
            <button
              onClick={() => setMessage(null)}
              className="ml-auto opacity-60 hover:opacity-100"
            >
              ×
            </button>
          </div>
        )}
      </div>

      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="grid grid-cols-3 gap-4">
          {/* 饼图 */}
          <div className="col-span-1 bg-slate-800/30 rounded-xl border border-slate-700/30 p-4">
            <h3 className="text-sm font-medium text-slate-300 mb-2">
              缓存占比
            </h3>
            {chartData.length > 0 ? (
              <ResponsiveContainer width="100%" height={200}>
                <PieChart>
                  <Pie
                    data={chartData}
                    cx="50%"
                    cy="50%"
                    innerRadius={50}
                    outerRadius={80}
                    paddingAngle={2}
                    dataKey="value"
                  >
                    {chartData.map((_, index) => (
                      <Cell
                        key={`cell-${index}`}
                        fill={COLORS[index % COLORS.length]}
                      />
                    ))}
                  </Pie>
                  <Tooltip
                    formatter={(value: number) => formatBytes(value)}
                    contentStyle={{
                      backgroundColor: "#1e293b",
                      border: "1px solid #334155",
                      borderRadius: "8px",
                      color: "#f1f5f9",
                      fontSize: "12px",
                    }}
                  />
                </PieChart>
              </ResponsiveContainer>
            ) : (
              <div className="h-[200px] flex items-center justify-center text-slate-500 text-sm">
                暂无缓存数据
              </div>
            )}
          </div>

          {/* 列表 */}
          <div className="col-span-2 space-y-2">
            {caches.map((cache) => {
              const pmInfo = PACKAGE_MANAGERS.find(
                (p) => p.id === cache.package_manager
              );
              const isCleaning = cleaningPm === cache.package_manager;

              return (
                <div
                  key={cache.package_manager}
                  className={cn(
                    "flex items-center gap-3 p-3 rounded-lg border",
                    cache.exists
                      ? "bg-slate-800/30 border-slate-700/30"
                      : "bg-slate-800/10 border-slate-700/10 opacity-50"
                  )}
                >
                  <span className="text-xl flex-shrink-0">
                    {pmInfo?.icon}
                  </span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-medium text-white">
                        {pmInfo?.displayName}
                      </span>
                      {!cache.exists && (
                        <span className="text-[10px] text-slate-500">
                          未找到缓存
                        </span>
                      )}
                    </div>
                    {cache.exists && (
                      <p className="text-xs text-slate-400 font-mono truncate">
                        {cache.path}
                      </p>
                    )}
                  </div>

                  {cache.exists && (
                    <div className="flex items-center gap-3 flex-shrink-0">
                      <div className="text-right">
                        <p className="text-sm font-semibold text-white">
                          {formatBytes(cache.size_bytes)}
                        </p>
                        <p className="text-[10px] text-slate-500">
                          {cache.file_count.toLocaleString()} 个文件
                        </p>
                      </div>

                      {confirmClean === cache.package_manager ? (
                        <div className="flex items-center gap-1">
                          <button
                            onClick={() => handleClean(cache.package_manager)}
                            className="px-2 py-1 rounded text-xs bg-red-600 text-white hover:bg-red-700 transition-colors"
                          >
                            确认清理
                          </button>
                          <button
                            onClick={() => setConfirmClean(null)}
                            className="px-2 py-1 rounded text-xs text-slate-400 hover:text-slate-200"
                          >
                            取消
                          </button>
                        </div>
                      ) : (
                        <button
                          onClick={() =>
                            setConfirmClean(cache.package_manager)
                          }
                          disabled={isCleaning}
                          className="flex items-center gap-1 px-2 py-1 rounded text-xs text-red-400 hover:bg-red-500/10 transition-colors disabled:opacity-50"
                        >
                          {isCleaning ? (
                            <RefreshCw size={12} className="animate-spin" />
                          ) : (
                            <Trash2 size={12} />
                          )}
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
