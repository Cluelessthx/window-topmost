//! 系统托盘模块

use crate::config::Config;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::cell::RefCell;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
    NIM_MODIFY, NOTIFYICONDATAW, NIIF_INFO,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    GetCursorPos, LoadIconW, PostQuitMessage, RegisterClassW, SetForegroundWindow,
    TrackPopupMenu, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDI_APPLICATION, MF_STRING,
    TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_RIGHTBUTTON, WINDOW_EX_STYLE, WM_COMMAND, WM_DESTROY,
    WM_LBUTTONDBLCLK, WM_RBUTTONUP, WM_USER, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

/// 托盘消息
const WM_TRAYICON: u32 = WM_USER + 1;

/// 菜单项 ID
const MENU_TOGGLE: u16 = 1;
const MENU_CONFIG: u16 = 2;
const MENU_RELOAD: u16 = 3;
const MENU_EXIT: u16 = 4;

// 全局变量（用于窗口过程）
thread_local! {
    static RUNNING_FLAG: RefCell<Option<Arc<AtomicBool>>> = const { RefCell::new(None) };
    static GLOBAL_CONFIG: RefCell<Option<Config>> = const { RefCell::new(None) };
    static TRAY_HWND: RefCell<Option<HWND>> = const { RefCell::new(None) };
}

/// 托盘句柄
pub struct TrayHandle {
    hwnd: HWND,
}

impl Drop for TrayHandle {
    fn drop(&mut self) {
        remove_tray_icon(self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// 创建系统托盘
pub fn create_tray(running: Arc<AtomicBool>, config: Config) -> Result<TrayHandle> {
    RUNNING_FLAG.with(|f| *f.borrow_mut() = Some(running));
    GLOBAL_CONFIG.with(|c| *c.borrow_mut() = Some(config.clone()));

    unsafe {
        let instance = GetModuleHandleW(None)?;

        let class_name = w!("WindowTopMostTray");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("WindowTopMostTray"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        )?;

        TRAY_HWND.with(|h| *h.borrow_mut() = Some(hwnd));
        add_tray_icon(hwnd, &config)?;

        Ok(TrayHandle { hwnd })
    }
}

/// 添加托盘图标
fn add_tray_icon(hwnd: HWND, config: &Config) -> Result<()> {
    unsafe {
        // 使用 None 加载系统默认图标
        let icon = LoadIconW(None, IDI_APPLICATION)?;

        let tip = format!("窗口置顶工具
快捷键: {}", config.hotkey.display());
        let tip_wide: Vec<u16> = tip.encode_utf16().chain(std::iter::once(0)).collect();

        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: 1,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: WM_TRAYICON,
            hIcon: icon,
            ..Default::default()
        };

        let tip_len = tip_wide.len().min(nid.szTip.len());
        nid.szTip[..tip_len].copy_from_slice(&tip_wide[..tip_len]);

        if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
            anyhow::bail!("Failed to add tray icon");
        }
    }
    Ok(())
}

/// 移除托盘图标
fn remove_tray_icon(hwnd: HWND) {
    unsafe {
        let nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: 1,
            ..Default::default()
        };
        let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
    }
}

/// 显示通知气泡
pub fn show_notification(title: &str, message: &str) {
    TRAY_HWND.with(|h| {
        if let Some(hwnd) = *h.borrow() {
            unsafe {
                let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
                let msg_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

                let mut nid = NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    uFlags: NIF_INFO,
                    dwInfoFlags: NIIF_INFO,
                    ..Default::default()
                };

                let title_len = title_wide.len().min(nid.szInfoTitle.len());
                nid.szInfoTitle[..title_len].copy_from_slice(&title_wide[..title_len]);

                let msg_len = msg_wide.len().min(nid.szInfo.len());
                nid.szInfo[..msg_len].copy_from_slice(&msg_wide[..msg_len]);

                let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
            }
        }
    });

    #[cfg(debug_assertions)]
    println!("[{}] {}", title, message);
}

/// 显示右键菜单
fn show_context_menu(hwnd: HWND) {
    unsafe {
        let menu = CreatePopupMenu().unwrap();

        AppendMenuW(menu, MF_STRING, MENU_TOGGLE as usize, w!("📌 置顶当前窗口")).ok();
        AppendMenuW(menu, MF_STRING, MENU_CONFIG as usize, w!("⚙️ 打开配置文件")).ok();
        AppendMenuW(menu, MF_STRING, MENU_RELOAD as usize, w!("🔄 重新加载配置")).ok();
        AppendMenuW(menu, MF_STRING, MENU_EXIT as usize, w!("❌ 退出")).ok();

        let mut pt = windows::Win32::Foundation::POINT::default();
        let _ = GetCursorPos(&mut pt);
        let _ = SetForegroundWindow(hwnd);
        let _ = TrackPopupMenu(menu, TPM_LEFTALIGN | TPM_RIGHTBUTTON | TPM_BOTTOMALIGN, pt.x, pt.y, 0, hwnd, None);
        let _ = DestroyMenu(menu);
    }
}

/// 处理菜单命令
fn handle_menu_command(cmd: u16) {
    match cmd {
        MENU_TOGGLE => {
            GLOBAL_CONFIG.with(|c| {
                if let Some(config) = c.borrow().as_ref() {
                    crate::window::toggle_topmost(config);
                }
            });
        }
        MENU_CONFIG => {
            let config_path = crate::config::Config::config_path();
            let _ = std::process::Command::new("notepad.exe")
                .arg(&config_path)
                .spawn();
        }
        MENU_RELOAD => {
            show_notification("提示", "请重启程序以应用新配置");
        }
        MENU_EXIT => {
            RUNNING_FLAG.with(|f| {
                if let Some(running) = f.borrow().as_ref() {
                    running.store(false, Ordering::SeqCst);
                }
            });
            unsafe { PostQuitMessage(0); }
        }
        _ => {}
    }
}

/// 窗口过程
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            let event = lparam.0 as u32;
            match event {
                WM_RBUTTONUP => show_context_menu(hwnd),
                WM_LBUTTONDBLCLK => handle_menu_command(MENU_TOGGLE),
                _ => {}
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            handle_menu_command((wparam.0 & 0xFFFF) as u16);
            LRESULT(0)
        }
        WM_DESTROY => {
            remove_tray_icon(hwnd);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
