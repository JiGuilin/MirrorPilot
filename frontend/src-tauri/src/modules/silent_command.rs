use std::process::Command;

// ponytail: Windows 子进程辅助 — 自动处理 .cmd 脚本，加 CREATE_NO_WINDOW 防弹黑窗
// Windows 上 npm/yarn/pnpm 等是 .cmd 脚本，Command::new("npm") 无法直接执行
// 用 which crate 解析完整路径，若为 .cmd/.bat 则走 cmd /C 代理
#[cfg(target_os = "windows")]
pub fn silent_command(program: &str) -> Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // 尝试用 which 找到完整路径
    if let Ok(resolved) = which::which(program) {
        let path_str = resolved.to_string_lossy();
        // .cmd / .bat 脚本必须通过 cmd /C 代理
        let lower = path_str.to_lowercase();
        if lower.ends_with(".cmd") || lower.ends_with(".bat") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &path_str])
                .creation_flags(CREATE_NO_WINDOW);
            return cmd;
        }
        // 真正的 .exe，直接调用
        let mut cmd = Command::new(&*path_str);
        cmd.creation_flags(CREATE_NO_WINDOW);
        return cmd;
    }

    // which 找不到，降级为原始名称
    let mut cmd = Command::new(program);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

#[cfg(not(target_os = "windows"))]
pub fn silent_command(program: &str) -> Command {
    Command::new(program)
}
