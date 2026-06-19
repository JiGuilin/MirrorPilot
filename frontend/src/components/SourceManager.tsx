import React, { useState, useEffect, useCallback } from "react";
import {
  Zap,
  Plus,
  Trash2,
  CheckCircle2,
  XCircle,
  Globe,
  RefreshCw,
  Play,
} from "lucide-react";
import { cn, formatLatency, formatSpeed, getLatencyColor, getRegionLabel, getRegionFlagClass } from "../lib/utils";
import {
  getSources,
  applySource,
  addCustomSource,
  deleteCustomSource,
  testSources,
  getCurrentSource,
  openConfigFile,
} from "../lib/api";
import type { Source, PackageManagerStatus, SpeedTestProgress } from "../types";
import { listen } from "@tauri-apps/api/event";

interface SourceManagerProps {
  selectedPm: string;
  pmStatus: PackageManagerStatus | null;
  onSourceApplied?: () => void;
}

export function SourceManager({ selectedPm, pmStatus, onSourceApplied }: SourceManagerProps) {
  const [sources, setSources] = useState<Source[]>([]);
  const [currentUrl, setCurrentUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [applyingId, setApplyingId] = useState<string | null>(null);
  const [testingIds, setTestingIds] = useState<Set<string>>(new Set());
  const [testingAll, setTestingAll] = useState(false);
  const [testProgress, setTestProgress] = useState<SpeedTestProgress | null>(null);
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [newName, setNewName] = useState("");
  const [newUrl, setNewUrl] = useState("");
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  const loadData = useCallback(async () => {
    setLoading(true);
    try {
      const [srcs, curUrl] = await Promise.all([
        getSources(selectedPm),
        getCurrentSource(selectedPm),
      ]);
      // If current URL is not in the source list, add it as a virtual entry
      if (curUrl && !srcs.some((s) => s.url === curUrl)) {
        srcs.unshift({
          id: "__current__",
          name: "当前源",
          url: curUrl,
          package_manager: selectedPm,
          is_builtin: false,
          is_custom: false,
          region: "custom",
          status: "active",
          latency: null,
          speed: null,
          last_tested: null,
        });
      }
      setSources(srcs);
      setCurrentUrl(curUrl);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  }, [selectedPm]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  // Listen for batch test progress - register eagerly via ref
  const unlistenRef = React.useRef<(() => void) | null>(null);
  useEffect(() => {
    listen<SpeedTestProgress>("speed-test-progress", (event) => {
      setTestProgress(event.payload);
      // Update source latency & speed as results come in
      event.payload.results.forEach((r) => {
        if (r.success) {
          setSources((prev) =>
            prev.map((s) =>
              s.id === r.source_id
                ? { ...s, latency: r.latency_ms, speed: r.speed_kbps, last_tested: new Date().toISOString() }
                : s
            )
          );
        }
      });
    }).then((fn) => { unlistenRef.current = fn; });
    return () => { unlistenRef.current?.(); };
  }, []);

  const handleApply = async (source: Source) => {
    setApplyingId(source.id);
    setMessage(null);
    try {
      const result = await applySource(selectedPm, source.url);
      setMessage({
        type: result.success ? "success" : "error",
        text: result.message,
      });
      if (result.success) {
        setCurrentUrl(source.url);
        onSourceApplied?.();
      }
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
    setApplyingId(null);
  };

  const handleTest = async (source: Source) => {
    setTestingIds((prev) => new Set(prev).add(source.id));
    try {
      const results = await testSources([[source.id, source.name, source.url]]);
      const r = results[0];
      if (r && r.success) {
        setSources((prev) =>
          prev.map((s) =>
            s.id === source.id
              ? { ...s, latency: r.latency_ms, speed: r.speed_kbps, last_tested: new Date().toISOString() }
              : s
          )
        );
      }
    } catch (e) {
      console.error(e);
    }
    setTestingIds((prev) => {
      const next = new Set(prev);
      next.delete(source.id);
      return next;
    });
  };

  const handleTestAll = async () => {
    if (sources.length === 0) return;
    setTestingAll(true);
    setTestProgress(null);
    try {
      const testData: Array<[string, string, string]> = sources.map((s) => [
        s.id, s.name, s.url,
      ]);
      await testSources(testData);
      // Real-time updates come from the event listener above
    } catch (e) {
      console.error(e);
    }
    setTestingAll(false);
    setTestProgress(null);
  };

  const handleAddCustom = async () => {
    if (!newName.trim() || !newUrl.trim()) return;
    try {
      await addCustomSource(newName.trim(), newUrl.trim(), selectedPm, "custom");
      setShowAddDialog(false);
      setNewName("");
      setNewUrl("");
      loadData();
      setMessage({ type: "success", text: "自定义源已添加" });
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
  };

  const handleDelete = async (source: Source) => {
    try {
      await deleteCustomSource(source.id);
      loadData();
      setMessage({ type: "success", text: "已删除自定义源" });
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
  };

  return (
    <div className="h-full flex flex-col">
      {/* 当前源 & 配置文件信息 */}
      <div className="px-6 py-3 border-b border-slate-700/50 bg-slate-800/30">
        <div className="flex items-center justify-between">
          <div className="min-w-0">
            <p className="text-xs text-slate-400 mb-0.5">当前源</p>
            <p className="text-sm font-mono text-blue-400 truncate">
              {currentUrl || "未配置"}
            </p>
          </div>
          {currentUrl && (
            <div className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-green-500/10 text-green-400 text-xs flex-shrink-0">
              <CheckCircle2 size={12} />
              已配置
            </div>
          )}
        </div>
        {pmStatus?.config_path && (
          <div className="mt-1 flex items-center gap-2">
            <p className="text-[10px] text-slate-500 font-mono truncate">
              配置文件: {pmStatus.config_path}
            </p>
            <button
              onClick={() => openConfigFile(selectedPm).catch(() => {})}
              className="flex-shrink-0 text-[10px] text-blue-400 hover:text-blue-300 transition-colors"
            >
              打开
            </button>
          </div>
        )}
      </div>

      {/* 消息提示 */}
      {message && (
        <div
          className={cn(
            "flex items-center gap-2 px-6 py-2 border-b border-slate-700/50 text-sm",
            message.type === "success"
              ? "bg-green-500/10 text-green-400"
              : "bg-red-500/10 text-red-400"
          )}
        >
          {message.type === "success" ? (
            <CheckCircle2 size={14} />
          ) : (
            <XCircle size={14} />
          )}
          {message.text}
          <button
            onClick={() => setMessage(null)}
            className="ml-auto opacity-60 hover:opacity-100"
          >
            ×
          </button>
        </div>
      )}

      {/* 源列表 */}
      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-sm font-medium text-slate-300">
            可用源 ({sources.length})
          </h3>
          <div className="flex items-center gap-2">
            <button
              onClick={handleTestAll}
              disabled={testingAll || sources.length === 0}
              className={cn(
                "flex items-center gap-1 px-2.5 py-1 rounded text-xs transition-colors",
                testingAll
                  ? "bg-orange-600/10 text-orange-300"
                  : "bg-orange-600/20 text-orange-400 hover:bg-orange-600/30"
              )}
            >
              {testingAll ? (
                <RefreshCw size={12} className="animate-spin" />
              ) : (
                <Play size={12} />
              )}
              {testingAll
                ? testProgress
                  ? `${testProgress.current}/${testProgress.total}`
                  : "测速中"
                : "一键测速"}
            </button>
            <button
              onClick={() => loadData()}
              className="flex items-center gap-1 px-2 py-1 rounded text-xs text-slate-400 hover:text-slate-200 hover:bg-slate-700/50"
            >
              <RefreshCw size={12} />
              刷新
            </button>
            <button
              onClick={() => setShowAddDialog(true)}
              className="flex items-center gap-1 px-2.5 py-1 rounded text-xs bg-blue-600/20 text-blue-400 hover:bg-blue-600/30 transition-colors"
            >
              <Plus size={12} />
              添加
            </button>
          </div>
        </div>

        {/* Batch test progress bar */}
        {testingAll && testProgress && (
          <div className="mb-3">
            <div className="flex items-center justify-between text-xs text-slate-400 mb-1">
              <span>
                测试 {testProgress.current}/{testProgress.total}: {testProgress.current_source_name}
              </span>
              <span>{Math.round((testProgress.current / testProgress.total) * 100)}%</span>
            </div>
            <div className="h-1.5 bg-slate-700 rounded-full overflow-hidden">
              <div
                className="h-full bg-orange-500 rounded-full transition-all duration-300"
                style={{ width: `${(testProgress.current / testProgress.total) * 100}%` }}
              />
            </div>
          </div>
        )}

        {loading ? (
          <div className="flex items-center justify-center py-12">
            <RefreshCw size={20} className="animate-spin text-slate-400" />
          </div>
        ) : (
          <div className="space-y-2">
            {sources.map((source) => {
              const isCurrent =
                currentUrl && source.url === currentUrl;
              const isApplying = applyingId === source.id;
              const isTesting = testingIds.has(source.id);

              return (
                <div
                  key={source.id}
                  className={cn(
                    "p-3 rounded-lg border transition-all",
                    isCurrent
                      ? "bg-blue-600/10 border-blue-500/40"
                      : "bg-slate-800/30 border-slate-700/30 hover:bg-slate-800/50"
                  )}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        {source.region !== "custom" ? (
                          <span className={getRegionFlagClass(source.region)} style={{fontSize: '14px'}} />
                        ) : (
                          <span className="text-xs">🔧</span>
                        )}
                        <span className="text-sm font-medium text-white">
                          {source.name}
                        </span>
                        {isCurrent && (
                          <span className="px-1.5 py-0.5 rounded text-[10px] bg-blue-500/20 text-blue-400">
                            当前
                          </span>
                        )}
                        {source.is_custom && (
                          <span className="px-1.5 py-0.5 rounded text-[10px] bg-purple-500/20 text-purple-400">
                            自定义
                          </span>
                        )}
                        <span className="text-[10px] text-slate-500">
                          {getRegionLabel(source.region)}
                        </span>
                      </div>
                      <p className="mt-0.5 text-xs font-mono text-slate-400 truncate">
                        {source.url}
                      </p>
                      {(source.latency !== null || source.speed !== null) && (
                        <p className="mt-0.5 text-xs text-slate-400">
                          {source.latency !== null && (
                            <span className={getLatencyColor(source.latency)}>
                              延迟: {formatLatency(source.latency)}
                            </span>
                          )}
                          {source.latency !== null && source.speed !== null && (
                            <span className="mx-1.5">·</span>
                          )}
                          {source.speed !== null && (
                            <span className="text-green-400">
                              速度: {formatSpeed(source.speed)}
                            </span>
                          )}
                        </p>
                      )}
                    </div>

                    <div className="flex items-center gap-1.5 ml-3 flex-shrink-0">
                      <button
                        onClick={() => handleTest(source)}
                        disabled={isTesting || testingAll}
                        className={cn(
                          "flex items-center gap-1 px-2 py-1 rounded text-xs transition-colors",
                          isTesting
                            ? "text-slate-500"
                            : "text-slate-300 hover:bg-slate-700/50"
                        )}
                      >
                        {isTesting ? (
                          <RefreshCw size={12} className="animate-spin" />
                        ) : (
                          <Zap size={12} />
                        )}
                        {isTesting ? "测试中" : "测速"}
                      </button>

                      {!isCurrent && (
                        <button
                          onClick={() => handleApply(source)}
                          disabled={isApplying}
                          className={cn(
                            "flex items-center gap-1 px-2.5 py-1 rounded text-xs transition-colors",
                            isApplying
                              ? "bg-blue-600/10 text-blue-300"
                              : "bg-blue-600/20 text-blue-400 hover:bg-blue-600/30"
                          )}
                        >
                          {isApplying ? (
                            <RefreshCw size={12} className="animate-spin" />
                          ) : (
                            <Globe size={12} />
                          )}
                          应用
                        </button>
                      )}

                      {source.is_custom && (
                        <button
                          onClick={() => handleDelete(source)}
                          className="flex items-center gap-1 px-2 py-1 rounded text-xs text-red-400 hover:bg-red-500/10 transition-colors"
                        >
                          <Trash2 size={12} />
                        </button>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}

            {sources.length === 0 && (
              <div className="text-center py-12 text-slate-500">
                <p>暂无可用源</p>
              </div>
            )}
          </div>
        )}
      </div>

      {/* 添加自定义源对话框 */}
      {showAddDialog && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-slate-800 rounded-xl border border-slate-600/50 p-6 w-96 shadow-2xl">
            <h3 className="text-base font-semibold text-white mb-4">
              添加自定义源
            </h3>
            <div className="space-y-3">
              <div>
                <label className="text-xs text-slate-400">名称</label>
                <input
                  value={newName}
                  onChange={(e) => setNewName(e.target.value)}
                  placeholder="例如：公司内部源"
                  className="w-full mt-1 px-3 py-2 rounded-lg bg-slate-700/50 border border-slate-600/50 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-blue-500/50"
                />
              </div>
              <div>
                <label className="text-xs text-slate-400">URL</label>
                <input
                  value={newUrl}
                  onChange={(e) => setNewUrl(e.target.value)}
                  placeholder="例如：https://registry.npm.example.com/"
                  className="w-full mt-1 px-3 py-2 rounded-lg bg-slate-700/50 border border-slate-600/50 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-blue-500/50 font-mono"
                />
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-5">
              <button
                onClick={() => {
                  setShowAddDialog(false);
                  setNewName("");
                  setNewUrl("");
                }}
                className="px-4 py-1.5 rounded-lg text-sm text-slate-300 hover:bg-slate-700/50 transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleAddCustom}
                disabled={!newName.trim() || !newUrl.trim()}
                className="px-4 py-1.5 rounded-lg text-sm bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                添加
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
