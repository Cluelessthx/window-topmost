//! 窗口操作模块

use crate::config::Config;
use crate::tray;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowLongW, SetWindowPos, GWL_EXSTYLE, HWND_NOTOPMOST,
    HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, WS_EX_TOPMOST,
};

/// 切换当前窗口的置顶状态
pub fn toggle_topmost(config: &Config) {
    unsafe {
        // 获取当前前台窗口
        let hwnd = GetForegroundWindow();

        if hwnd.0 == std::ptr::null_mut() {
            tray::show_notification("窗口置顶工具", "未找到活动窗口！");
            return;
        }

        // 检查是否已置顶
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        let is_topmost = (ex_style & WS_EX_TOPMOST.0) != 0;

        // 获取窗口标题用于提示
        let title = get_window_title(hwnd);

        if is_topmost {
            // 取消置顶
            let result = SetWindowPos(
                hwnd,
                HWND_NOTOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE,
            );

            if result.is_ok() {
                if config.settings.play_sound {
                    play_beep(600, 100);
                }
                if config.settings.show_notification {
                    tray::show_notification("取消置顶", &title);
                }
            }
        } else {
            // 设置置顶
            let result = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE,
            );

            if result.is_ok() {
                if config.settings.play_sound {
                    play_beep(800, 100);
                }
                if config.settings.show_notification {
                    tray::show_notification("已置顶", &title);
                }
            }
        }
    }
}

/// 获取窗口标题
fn get_window_title(hwnd: HWND) -> String {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextLengthW, GetWindowTextW};

    unsafe {
        let len = GetWindowTextLengthW(hwnd);
        if len == 0 {
            return "(无标题)".to_string();
        }

        let mut buffer = vec![0u16; (len + 1) as usize];
        let copied = GetWindowTextW(hwnd, &mut buffer);

        if copied > 0 {
            String::from_utf16_lossy(&buffer[..copied as usize])
        } else {
            "(无标题)".to_string()
        }
    }
}

/// 播放提示音
fn play_beep(frequency: u32, duration: u32) {
    use windows::Win32::System::Diagnostics::Debug::Beep;

    unsafe {
        let _ = Beep(frequency, duration);
    }
}
