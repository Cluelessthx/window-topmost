//! 窗口置顶工具 - Windows Window TopMost Tool
//!
//! 使用快捷键让当前窗口置顶/取消置顶
//! 作者: 幽浮喵 (猫娘工程师) ฅ'ω'ฅ

#![windows_subsystem = "windows"] // 隐藏控制台窗口

mod config;
mod hotkey;
mod tray;
mod window;

use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY, WM_QUIT,
};

/// 程序入口
fn main() -> Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    // 运行标志
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // 注册全局快捷键
    let hotkey_id = hotkey::register_hotkey(&config.hotkey)?;

    // 创建系统托盘
    let _tray = tray::create_tray(running_clone, config.clone())?;

    // 显示启动提示
    tray::show_notification("窗口置顶工具", &format!("程序已启动！\n快捷键: {}", config.hotkey.display()));

    // 消息循环
    unsafe {
        let mut msg = MSG::default();

        while running.load(Ordering::SeqCst) {
            let ret = GetMessageW(&mut msg, None, 0, 0);

            if ret.0 == 0 || ret.0 == -1 {
                break;
            }

            match msg.message {
                WM_HOTKEY => {
                    if msg.wParam.0 as i32 == hotkey_id {
                        window::toggle_topmost(&config);
                    }
                }
                WM_QUIT => break,
                _ => {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }

        // 注销快捷键
        hotkey::unregister_hotkey(hotkey_id);
    }

    Ok(())
}
