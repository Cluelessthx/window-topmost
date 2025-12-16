// build.rs - Windows 资源文件编译配置
fn main() {
    // 仅在 Windows 上编译资源
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        // 设置应用程序图标（如果存在）
        if std::path::Path::new("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        }

        // 设置版本信息
        res.set("ProductName", "窗口置顶工具");
        res.set("FileDescription", "Windows 窗口置顶工具 - 快捷键置顶窗口");
        res.set("LegalCopyright", "幽浮喵 (猫娘工程师) ฅ'ω'ฅ");

        // 编译资源
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile resources: {}", e);
        }
    }
}
