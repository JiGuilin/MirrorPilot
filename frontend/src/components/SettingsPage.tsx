import { useState, useEffect } from "react";
import { Settings, Moon, Sun, Monitor, Save, RotateCcw, Download, Upload } from "lucide-react";
import { cn, applyTheme } from "../lib/utils";
import { getAppConfig, saveAppConfig, exportConfig, importConfig } from "../lib/api";
import type { AppConfig } from "../types";

const DEFAULT_CONFIG: AppConfig = {
  test_timeout_seconds: 10,
  max_concurrent_tests: 5,
  theme: "system",
  language: "zh-CN",
};

export function SettingsPage() {
  const [config, setConfig] = useState<AppConfig>(DEFAULT_CONFIG);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [importing, setImporting] = useState(false);
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  useEffect(() => {
    getAppConfig()
      .then(setConfig)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  const handleSave = async () => {
    setSaving(true);
    setMessage(null);
    try {
      await saveAppConfig(config);
      setMessage({ type: "success", text: "设置已保存" });
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
    setSaving(false);
  };

  const handleReset = () => setConfig(DEFAULT_CONFIG);

  const handleExport = async () => {
    try {
      const json = await exportConfig();
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `mirrorpilot-config-${new Date().toISOString().slice(0, 10)}.json`;
      a.click();
      URL.revokeObjectURL(url);
      setMessage({ type: "success", text: "配置已导出" });
    } catch (e: any) {
      setMessage({ type: "error", text: e.toString() });
    }
  };

  const handleImport = async () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".json";
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      setImporting(true);
      try {
        const text = await file.text();
        const result = await importConfig(text);
        setMessage({ type: "success", text: result });
      } catch (e: any) {
        setMessage({ type: "error", text: e.toString() });
      }
      setImporting(false);
    };
    input.click();
  };

  if (loading) return <div className="flex items-center justify-center h-full text-ink-subtle"><Settings size={18} className="animate-spin" /></div>;

  const themeOptions = [
    { value: "light", label: "浅色", icon: <Sun size={13} /> },
    { value: "dark", label: "深色", icon: <Moon size={13} /> },
    { value: "system", label: "跟随系统", icon: <Monitor size={13} /> },
  ] as const;

  return (
    <div className="h-full flex flex-col">
      <div className="px-4 py-2 border-b border-hairline bg-surface-1">
        <div className="flex items-center gap-2.5">
          <Settings size={16} className="text-accent" />
          <h2 className="text-[13px] font-semibold text-ink tracking-tight">设置</h2>
        </div>
      </div>

      {message && (
        <div className={cn(
          "flex items-center gap-2 px-5 py-2 border-b border-hairline text-[12px]",
          message.type === "success" ? "bg-lv-success/10 text-lv-success" : "bg-lv-danger/10 text-lv-danger"
        )}>
          {message.text}
          <button onClick={() => setMessage(null)} className="ml-auto opacity-60 hover:opacity-100">×</button>
        </div>
      )}

      <div className="flex-1 overflow-y-auto px-4 py-3 space-y-5 max-w-xl">
        {/* 主题 */}
        <section>
          <h3 className="text-[11px] font-medium text-ink-subtle mb-1.5 uppercase tracking-wider">外观</h3>
          <div className="flex gap-2">
            {themeOptions.map((opt) => (
              <button
                key={opt.value}
                onClick={() => { const v = opt.value; setConfig({ ...config, theme: v }); applyTheme(v); }}
                className={cn(
                  "flex items-center gap-2 px-3 py-1.5 rounded-md border text-[12px] transition-all",
                  config.theme === opt.value
                    ? "bg-accent/15 border-accent/30 text-accent-hover"
                    : "bg-surface-1 border-hairline text-ink-muted hover:bg-surface-2"
                )}
              >
                {opt.icon}
                {opt.label}
              </button>
            ))}
          </div>
        </section>

        {/* 测速 */}
        <section>
          <h3 className="text-[11px] font-medium text-ink-subtle mb-2 uppercase tracking-wider">测速</h3>
          <div className="space-y-2.5 bg-surface-1 rounded-lg border border-hairline p-3">
            <label className="flex items-center justify-between">
              <span className="text-[13px] text-ink-muted">超时时间（秒）</span>
              <input
                type="number"
                min={3}
                max={60}
                value={config.test_timeout_seconds}
                onChange={(e) => setConfig({ ...config, test_timeout_seconds: Number(e.target.value) || 10 })}
                className="w-16 px-2 py-1 rounded-md bg-surface-2 border border-hairline text-[13px] text-ink text-center focus:outline-none focus:border-accent/50"
              />
            </label>

            <div className="h-px bg-hairline" />

            <label className="flex items-center justify-between">
              <span className="text-[13px] text-ink-muted">最大并发数</span>
              <input
                type="number"
                min={1}
                max={20}
                value={config.max_concurrent_tests}
                onChange={(e) => setConfig({ ...config, max_concurrent_tests: Number(e.target.value) || 5 })}
                className="w-16 px-2 py-1 rounded-md bg-surface-2 border border-hairline text-[13px] text-ink text-center focus:outline-none focus:border-accent/50"
              />
            </label>
          </div>
        </section>

        {/* 导入导出 */}
        <section>
          <h3 className="text-[11px] font-medium text-ink-subtle mb-1.5 uppercase tracking-wider">配置导入导出</h3>
          <div className="flex gap-2">
            <button
              onClick={handleExport}
              className="flex items-center gap-2 px-3 py-2 rounded-md border text-[12px] transition-all bg-surface-1 border-hairline text-ink-muted hover:bg-surface-2"
            >
              <Download size={13} />
              导出配置
            </button>
            <button
              onClick={handleImport}
              disabled={importing}
              className="flex items-center gap-2 px-3 py-2 rounded-md border text-[12px] transition-all bg-surface-1 border-hairline text-ink-muted hover:bg-surface-2 disabled:opacity-50"
            >
              <Upload size={13} />
              {importing ? "导入中..." : "导入配置"}
            </button>
          </div>
          <p className="mt-1 text-[10px] text-ink-tertiary">导出包含当前所有源配置和自定义源，可分享给团队成员</p>
        </section>

        {/* 语言 */}
        <section>
          <h3 className="text-[11px] font-medium text-ink-subtle mb-1.5 uppercase tracking-wider">语言</h3>
          <select
            value={config.language}
            onChange={(e) => setConfig({ ...config, language: e.target.value })}
            className="px-3 py-1.5 rounded-md bg-surface-2 border border-hairline text-[13px] text-ink focus:outline-none focus:border-accent/50"
          >
            <option value="zh-CN">简体中文</option>
            <option value="en">English</option>
          </select>
        </section>
      </div>

      {/* 底部 */}
      <div className="px-4 py-1.5 border-t border-hairline flex items-center justify-end gap-2">
        <button
          onClick={handleReset}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-[12px] text-ink-subtle hover:text-ink-muted hover:bg-surface-2 transition-colors"
        >
          <RotateCcw size={13} />
          恢复默认
        </button>
        <button
          onClick={handleSave}
          disabled={saving}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-[12px] bg-accent text-ink hover:bg-accent-hover disabled:opacity-50 transition-colors"
        >
          <Save size={13} />
          {saving ? "保存中" : "保存"}
        </button>
      </div>
    </div>
  );
}
