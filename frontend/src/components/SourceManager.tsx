import React, { useState, useEffect, useCallback, useMemo } from "react";
import {
  Zap,
  Plus,
  Trash2,
  CheckCircle2,
  XCircle,
  Globe,
  RefreshCw,
  Play,
  Search,
  ArrowUpDown,
  Trophy,
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
  const [searchQuery, setSearchQuery] = useState("");
  const [sortBy, setSortBy] = useState<"default" | "latency" | "speed">("default");
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [newName, setNewName] = useState("");
  const [newUrl, setNewUrl] = useState("");
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  const fastestSource = useMemo(() => {
    const tested = sources.filter((s) => s.latency !== null && s.id !== "__current__");
    if (tested.length === 0) return null;
    return tested.reduce((best, s) => (s.latency! < best.latency! ? s : best), tested[0]);
  }, [sources]);

  const displayedSources = useMemo(() => {
    let filtered = sources;
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      filtered = sources.filter(
        (s) => s.name.toLowerCase().includes(q) || s.url.toLowerCase().includes(q)
      );
    }
    if (sortBy === "latency") {
      return [...filtered].sort((a, b) => {
        if (a.latency === null) return 1;
        if (b.latency === null) return -1;
        return a.latency - b.latency;
      });
    }
    if (sortBy === "speed") {
      return [...filtered].sort((a, b) => {
        if (a.speed === null) return 1;
        if (b.speed === null) return -1;
        return b.speed - a.speed;
      });
    }
    return filtered;
  }, [sources, searchQuery, sortBy]);

  const loadData = useCallback(async () => {
    setLoading(true);
    try {
      const [srcs, curUrl] = await Promise.all([
        getSources(selectedPm),
        getCurrentSource(selectedPm),
      ]);
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

  useEffect(() => { loadData(); }, [loadData]);

  const unlistenRef = React.useRef<(() => void) | null>(null);
  useEffect(() => {
    listen<SpeedTestProgress>("speed-test-progress", (event) => {
      setTestProgress(event.payload);
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
      if (result.success) {
        try {
          const verifyUrl = await getCurrentSource(selectedPm);
          if (verifyUrl && verifyUrl.trim().replace(/\/$/, "") === source.url.trim().replace(/\/$/, "")) {
            setCurrentUrl(source.url);
            setMessage({ type: "success", text: `${result.message}（已验证）` });
          } else {
            setCurrentUrl(verifyUrl);
            setMessage({ type: "success", text: `${result.message}（回读: ${verifyUrl || "未检测到"}）` });
          }
        } catch {
          setCurrentUrl(source.url);
          setMessage({ type: "success", text: result.message });
        }
        onSourceApplied?.();
      } else {
        setMessage({ type: "error", text: result.message });
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
    } catch (e) { console.error(e); }
    setTestingIds((prev) => { const next = new Set(prev); next.delete(source.id); return next; });
  };

  const handleTestAll = async () => {
    if (sources.length === 0) return;
    setTestingAll(true);
    setTestProgress(null);
    try {
      const testData: Array<[string, string, string]> = sources.map((s) => [s.id, s.name, s.url]);
      await testSources(testData);
    } catch (e) { console.error(e); }
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
      {/* 当前源信息 */}
      <div className="px-6 py-3 border-b border-hairline bg-surface-1">
        <div className="flex items-center justify-between">
          <div className="min-w-0">
            <p className="text-[10px] text-ink-subtle mb-1">当前源</p>
            <p className="text-[13px] font-mono text-accent-hover truncate">
              {currentUrl || "未配置"}
            </p>
          </div>
          {currentUrl && (
            <div className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-lv-success/10 text-lv-success text-[10px] flex-shrink-0">
              <CheckCircle2 size={10} />
              已配置
            </div>
          )}
        </div>
        {pmStatus?.config_path && (
          <div className="mt-1 flex items-center gap-2">
            <p className="text-[10px] text-ink-tertiary font-mono truncate">
              配置文件: {pmStatus.config_path}
            </p>
            <button
              onClick={() => openConfigFile(selectedPm).catch(() => {})}
              className="flex-shrink-0 text-[10px] text-accent hover:text-accent-hover transition-colors"
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
            "flex items-center gap-2 px-5 py-2 border-b border-hairline text-[13px]",
            message.type === "success"
              ? "bg-lv-success/10 text-lv-success"
              : "bg-lv-danger/10 text-lv-danger"
          )}
        >
          {message.type === "success" ? <CheckCircle2 size={13} /> : <XCircle size={13} />}
          {message.text}
          <button onClick={() => setMessage(null)} className="ml-auto opacity-60 hover:opacity-100">×</button>
        </div>
      )}

      {/* 源列表 */}
      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-[13px] font-medium text-ink-muted">
            可用源 ({sources.length})
          </h3>
          <div className="flex items-center gap-2">
            <div className="relative">
              <Search size={11} className="absolute left-2 top-1/2 -translate-y-1/2 text-ink-tertiary" />
              <input
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="搜索源"
                className="pl-5.5 pr-2 py-1 rounded-md text-[11px] bg-surface-2 border border-hairline text-ink placeholder-ink-tertiary focus:outline-none focus:border-accent/50 w-28"
              />
            </div>
            <button
              onClick={() => {
                const next = sortBy === "default" ? "latency" : sortBy === "latency" ? "speed" : "default";
                setSortBy(next);
              }}
              className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] text-ink-subtle hover:text-ink-muted hover:bg-surface-2"
            >
              <ArrowUpDown size={11} />
              {sortBy === "latency" ? "延迟" : sortBy === "speed" ? "速度" : "默认"}
            </button>
            <button
              onClick={handleTestAll}
              disabled={testingAll || sources.length === 0}
              className={cn(
                "flex items-center gap-1 px-2.5 py-1 rounded-md text-[11px] transition-colors",
                testingAll
                  ? "bg-lv-orange/10 text-lv-orange"
                  : "bg-lv-orange/15 text-lv-orange hover:bg-lv-orange/25"
              )}
            >
              {testingAll ? <RefreshCw size={11} className="animate-spin" /> : <Play size={11} />}
              {testingAll ? (testProgress ? `${testProgress.current}/${testProgress.total}` : "测速中") : "一键测速"}
            </button>
            <button
              onClick={() => loadData()}
              className="flex items-center gap-1 px-2 py-1 rounded-md text-[11px] text-ink-subtle hover:text-ink-muted hover:bg-surface-2"
            >
              <RefreshCw size={11} />
            </button>
            <button
              onClick={() => setShowAddDialog(true)}
              className="flex items-center gap-1 px-2.5 py-1 rounded-md text-[11px] bg-accent/15 text-accent-hover hover:bg-accent/25 transition-colors"
            >
              <Plus size={11} />
              添加
            </button>
          </div>
        </div>

        {/* 进度条 */}
        {testingAll && testProgress && (
          <div className="mb-4">
            <div className="flex items-center justify-between text-[10px] text-ink-subtle mb-1">
              <span>测试 {testProgress.current}/{testProgress.total}: {testProgress.current_source_name}</span>
              <span>{Math.round((testProgress.current / testProgress.total) * 100)}%</span>
            </div>
            <div className="h-1 bg-surface-3 rounded-full overflow-hidden">
              <div
                className="h-full bg-accent rounded-full transition-all duration-300"
                style={{ width: `${(testProgress.current / testProgress.total) * 100}%` }}
              />
            </div>
          </div>
        )}

        {loading ? (
          <div className="flex items-center justify-center py-12">
            <RefreshCw size={18} className="animate-spin text-ink-subtle" />
          </div>
        ) : (
          <div className="space-y-3">
            {/* 最快推荐 */}
            {fastestSource && fastestSource.latency !== null && (
              <div className="flex items-center gap-2 px-4 py-3 rounded-lg bg-lv-warning/5 border border-lv-warning/15">
                <Trophy size={13} className="text-lv-warning flex-shrink-0" />
                <span className="text-[11px] text-lv-warning">推荐最快：</span>
                <span className="text-[11px] text-ink font-medium">{fastestSource.name}</span>
                <span className={cn("text-[11px]", getLatencyColor(fastestSource.latency))}>
                  {formatLatency(fastestSource.latency)}
                </span>
                {fastestSource.speed !== null && (
                  <span className="text-[11px] text-lv-success">· {formatSpeed(fastestSource.speed)}</span>
                )}
                {currentUrl !== fastestSource.url && (
                  <button
                    onClick={() => handleApply(fastestSource)}
                    disabled={applyingId === fastestSource.id}
                    className="ml-auto flex items-center gap-1 px-2 py-0.5 rounded text-[10px] bg-lv-warning/15 text-lv-warning hover:bg-lv-warning/25 transition-colors"
                  >
                    <Globe size={10} />
                    切换
                  </button>
                )}
              </div>
            )}

            {displayedSources.map((source) => {
              const isCurrent = currentUrl && source.url === currentUrl;
              const isApplying = applyingId === source.id;
              const isTesting = testingIds.has(source.id);

              return (
                <div
                  key={source.id}
                  className={cn(
                    "p-4 rounded-lg border transition-all",
                    isCurrent
                      ? "bg-accent/8 border-accent/25"
                      : "bg-surface-1 border-hairline hover:bg-surface-2"
                  )}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        {source.region !== "custom" ? (
                          <span className={getRegionFlagClass(source.region)} style={{fontSize: '13px'}} />
                        ) : (
                          <span className="text-[11px]">🔧</span>
                        )}
                        <span className="text-[13px] font-medium text-ink">{source.name}</span>
                        {isCurrent && (
                          <span className="px-1.5 py-0.5 rounded text-[10px] bg-accent/15 text-accent-hover">当前</span>
                        )}
                        {source.is_custom && (
                          <span className="px-1.5 py-0.5 rounded text-[10px] bg-accent/10 text-accent">自定义</span>
                        )}
                        <span className="text-[10px] text-ink-tertiary">{getRegionLabel(source.region)}</span>
                      </div>
                      <p className="mt-0.5 text-[11px] font-mono text-ink-subtle truncate">{source.url}</p>
                      {(source.latency !== null || source.speed !== null) && (
                        <p className="mt-0.5 text-[11px] text-ink-subtle">
                          {source.latency !== null && (
                            <span className={getLatencyColor(source.latency)}>延迟: {formatLatency(source.latency)}</span>
                          )}
                          {source.latency !== null && source.speed !== null && <span className="mx-1.5">·</span>}
                          {source.speed !== null && <span className="text-lv-success">速度: {formatSpeed(source.speed)}</span>}
                        </p>
                      )}
                    </div>

                    <div className="flex items-center gap-1.5 ml-3 flex-shrink-0">
                      <button
                        onClick={() => handleTest(source)}
                        disabled={isTesting || testingAll}
                        className={cn(
                          "flex items-center gap-1 px-2 py-1 rounded text-[11px] transition-colors",
                          isTesting ? "text-ink-tertiary" : "text-ink-subtle hover:bg-surface-2"
                        )}
                      >
                        {isTesting ? <RefreshCw size={11} className="animate-spin" /> : <Zap size={11} />}
                        {isTesting ? "..." : "测速"}
                      </button>

                      {!isCurrent && (
                        <button
                          onClick={() => handleApply(source)}
                          disabled={isApplying}
                          className={cn(
                            "flex items-center gap-1 px-2.5 py-1 rounded text-[11px] transition-colors",
                            isApplying
                              ? "bg-accent/10 text-accent-focus"
                              : "bg-accent/15 text-accent-hover hover:bg-accent/25"
                          )}
                        >
                          {isApplying ? <RefreshCw size={11} className="animate-spin" /> : <Globe size={11} />}
                          应用
                        </button>
                      )}

                      {source.is_custom && (
                        <button
                          onClick={() => handleDelete(source)}
                          className="flex items-center gap-1 px-1.5 py-1 rounded text-[11px] text-lv-danger hover:bg-lv-danger/10 transition-colors"
                        >
                          <Trash2 size={11} />
                        </button>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}

            {displayedSources.length === 0 && (
              <div className="text-center py-12 text-ink-tertiary text-[13px]">暂无可用源</div>
            )}
          </div>
        )}
      </div>

      {/* 添加自定义源弹窗 */}
      {showAddDialog && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
          <div className="bg-surface-1 rounded-lg border border-hairline-strong p-5 w-96">
            <h3 className="text-[13px] font-semibold text-ink mb-3">添加自定义源</h3>
            <div className="space-y-3">
              <div>
                <label className="text-[11px] text-ink-subtle">名称</label>
                <input
                  value={newName}
                  onChange={(e) => setNewName(e.target.value)}
                  placeholder="例如：公司内部源"
                  className="w-full mt-1 px-3 py-1.5 rounded-md bg-surface-2 border border-hairline text-[13px] text-ink placeholder-ink-tertiary focus:outline-none focus:border-accent/50"
                />
              </div>
              <div>
                <label className="text-[11px] text-ink-subtle">URL</label>
                <input
                  value={newUrl}
                  onChange={(e) => setNewUrl(e.target.value)}
                  placeholder="例如：https://registry.npm.example.com/"
                  className="w-full mt-1 px-3 py-1.5 rounded-md bg-surface-2 border border-hairline text-[13px] text-ink placeholder-ink-tertiary focus:outline-none focus:border-accent/50 font-mono"
                />
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-4">
              <button
                onClick={() => { setShowAddDialog(false); setNewName(""); setNewUrl(""); }}
                className="px-3 py-1.5 rounded-md text-[13px] text-ink-muted hover:bg-surface-2 transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleAddCustom}
                disabled={!newName.trim() || !newUrl.trim()}
                className="px-3 py-1.5 rounded-md text-[13px] bg-accent text-ink hover:bg-accent-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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
