import { ExternalLink } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";

const APP_VERSION = "0.2.0";

export function AboutPage() {
  const handleOpenGitHub = () => {
    openUrl("https://github.com/JiGuilin/MirrorPilot").catch(console.error);
  };

  return (
    <div className="h-full flex flex-col">
      <div className="px-4 py-2 border-b border-hairline bg-surface-1">
        <div className="flex items-center gap-2.5">
          <div className="w-6 h-6 rounded-md bg-gradient-to-br from-accent to-accent-hover flex items-center justify-center text-white font-semibold text-[10px]">M</div>
          <h2 className="text-[13px] font-semibold text-ink tracking-tight">关于</h2>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-4">
        <div className="max-w-sm mx-auto text-center space-y-3">
          {/* Logo */}
          <div>
            <div className="w-14 h-14 mx-auto rounded-lg bg-gradient-to-br from-accent to-accent-hover flex items-center justify-center text-white font-bold text-xl">
              M
            </div>
            <h1 className="mt-2 text-lg font-semibold text-ink tracking-tight">MirrorPilot</h1>
            <p className="text-[12px] text-ink-subtle">镜像领航员</p>
          </div>

          {/* Version pill */}
          <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-surface-2 border border-hairline">
            <span className="w-1.5 h-1.5 rounded-full bg-lv-success" />
            <span className="text-[11px] text-ink-muted">v{APP_VERSION}</span>
          </div>

          {/* Description */}
          <p className="text-[13px] text-ink-subtle leading-relaxed">
            一站式管理开发环境中所有包管理器的镜像源配置，支持 npm、pip、Go、Maven、Docker、Cargo 等主流工具。
          </p>

          {/* Link */}
          <button
            onClick={handleOpenGitHub}
            className="inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-md bg-surface-1 border border-hairline text-[12px] text-ink-muted hover:text-ink hover:bg-surface-2 transition-colors"
          >
            <ExternalLink size={12} />
            GitHub
          </button>

          {/* Tech Stack */}
          <div className="pt-3 border-t border-hairline">
            <p className="text-[10px] text-ink-tertiary mb-2 uppercase tracking-wider">技术栈</p>
            <div className="flex flex-wrap justify-center gap-1.5">
              {["Tauri 2.0", "React", "TypeScript", "Rust", "Tailwind CSS"].map((t) => (
                <span key={t} className="px-2 py-0.5 rounded text-[10px] bg-surface-2 text-ink-subtle border border-hairline">
                  {t}
                </span>
              ))}
            </div>
          </div>

          {/* License */}
          <div className="pt-3 border-t border-hairline">
            <p className="text-[10px] text-ink-tertiary">开源协议：MIT License</p>
            <p className="text-[10px] text-ink-tertiary mt-0.5">© 2026 MirrorPilot Team</p>
          </div>
        </div>
      </div>
    </div>
  );
}
