//! 全局快捷键管理模块

use crate::config::HotkeyConfig;
use anyhow::{anyhow, Result};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS,
};

/// 快捷键 ID
const HOTKEY_ID: i32 = 1;

/// 注册全局快捷键
pub fn register_hotkey(config: &HotkeyConfig) -> Result<i32> {
    let vk_code = config
        .get_vk_code()
        .ok_or_else(|| anyhow!("无效的快捷键: {}", config.key))?;

    let modifiers = HOT_KEY_MODIFIERS(config.get_modifiers());

    unsafe {
        RegisterHotKey(None, HOTKEY_ID, modifiers, vk_code)
            .map_err(|e| anyhow!("注册快捷键失败: {}。\n可能被其他程序占用。", e))?;
    }

    Ok(HOTKEY_ID)
}

/// 注销全局快捷键
pub fn unregister_hotkey(id: i32) {
    unsafe {
        let _ = UnregisterHotKey(None, id);
    }
}
