//! 配置文件管理模块

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 快捷键配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Ctrl 键
    #[serde(default)]
    pub ctrl: bool,
    /// Alt 键
    #[serde(default)]
    pub alt: bool,
    /// Shift 键
    #[serde(default)]
    pub shift: bool,
    /// Win 键
    #[serde(default)]
    pub win: bool,
    /// 主键 (如 "Space", "T", "F9" 等)
    pub key: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            ctrl: true,
            alt: false,
            shift: false,
            win: false,
            key: "Space".to_string(),
        }
    }
}

impl HotkeyConfig {
    /// 获取显示用的快捷键字符串
    pub fn display(&self) -> String {
        let mut parts = Vec::new();

        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.win {
            parts.push("Win");
        }
        parts.push(&self.key);

        parts.join(" + ")
    }

    /// 将按键名称转换为虚拟键码
    pub fn get_vk_code(&self) -> Option<u32> {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;

        let key_upper = self.key.to_uppercase();

        Some(match key_upper.as_str() {
            // 功能键
            "F1" => VK_F1.0 as u32,
            "F2" => VK_F2.0 as u32,
            "F3" => VK_F3.0 as u32,
            "F4" => VK_F4.0 as u32,
            "F5" => VK_F5.0 as u32,
            "F6" => VK_F6.0 as u32,
            "F7" => VK_F7.0 as u32,
            "F8" => VK_F8.0 as u32,
            "F9" => VK_F9.0 as u32,
            "F10" => VK_F10.0 as u32,
            "F11" => VK_F11.0 as u32,
            "F12" => VK_F12.0 as u32,

            // 特殊键
            "SPACE" => VK_SPACE.0 as u32,
            "ENTER" | "RETURN" => VK_RETURN.0 as u32,
            "TAB" => VK_TAB.0 as u32,
            "ESCAPE" | "ESC" => VK_ESCAPE.0 as u32,
            "BACKSPACE" => VK_BACK.0 as u32,
            "DELETE" | "DEL" => VK_DELETE.0 as u32,
            "INSERT" | "INS" => VK_INSERT.0 as u32,
            "HOME" => VK_HOME.0 as u32,
            "END" => VK_END.0 as u32,
            "PAGEUP" | "PGUP" => VK_PRIOR.0 as u32,
            "PAGEDOWN" | "PGDN" => VK_NEXT.0 as u32,

            // 方向键
            "UP" => VK_UP.0 as u32,
            "DOWN" => VK_DOWN.0 as u32,
            "LEFT" => VK_LEFT.0 as u32,
            "RIGHT" => VK_RIGHT.0 as u32,

            // 数字键盘
            "NUMPAD0" | "NUM0" => VK_NUMPAD0.0 as u32,
            "NUMPAD1" | "NUM1" => VK_NUMPAD1.0 as u32,
            "NUMPAD2" | "NUM2" => VK_NUMPAD2.0 as u32,
            "NUMPAD3" | "NUM3" => VK_NUMPAD3.0 as u32,
            "NUMPAD4" | "NUM4" => VK_NUMPAD4.0 as u32,
            "NUMPAD5" | "NUM5" => VK_NUMPAD5.0 as u32,
            "NUMPAD6" | "NUM6" => VK_NUMPAD6.0 as u32,
            "NUMPAD7" | "NUM7" => VK_NUMPAD7.0 as u32,
            "NUMPAD8" | "NUM8" => VK_NUMPAD8.0 as u32,
            "NUMPAD9" | "NUM9" => VK_NUMPAD9.0 as u32,

            // 字母键 (A-Z)
            s if s.len() == 1 && s.chars().next().unwrap().is_ascii_alphabetic() => {
                s.chars().next().unwrap() as u32
            }

            // 数字键 (0-9)
            s if s.len() == 1 && s.chars().next().unwrap().is_ascii_digit() => {
                s.chars().next().unwrap() as u32
            }

            // 符号键
            "`" | "~" => VK_OEM_3.0 as u32,
            "-" | "_" => VK_OEM_MINUS.0 as u32,
            "=" | "+" => VK_OEM_PLUS.0 as u32,
            "[" | "{" => VK_OEM_4.0 as u32,
            "]" | "}" => VK_OEM_6.0 as u32,
            "\\" | "|" => VK_OEM_5.0 as u32,
            ";" | ":" => VK_OEM_1.0 as u32,
            "'" | "\"" => VK_OEM_7.0 as u32,
            "," | "<" => VK_OEM_COMMA.0 as u32,
            "." | ">" => VK_OEM_PERIOD.0 as u32,
            "/" | "?" => VK_OEM_2.0 as u32,

            _ => return None,
        })
    }

    /// 获取修饰键标志
    pub fn get_modifiers(&self) -> u32 {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;

        let mut mods = MOD_NOREPEAT; // 防止按住时重复触发

        if self.ctrl {
            mods |= MOD_CONTROL;
        }
        if self.alt {
            mods |= MOD_ALT;
        }
        if self.shift {
            mods |= MOD_SHIFT;
        }
        if self.win {
            mods |= MOD_WIN;
        }

        mods.0
    }
}

/// 设置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// 是否显示提示气泡
    #[serde(default = "default_true")]
    pub show_notification: bool,
    /// 是否播放提示音
    #[serde(default = "default_true")]
    pub play_sound: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_notification: true,
            play_sound: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 快捷键配置
    #[serde(default)]
    pub hotkey: HotkeyConfig,
    /// 设置
    #[serde(default)]
    pub settings: Settings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig::default(),
            settings: Settings::default(),
        }
    }
}

impl Config {
    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        let exe_path = std::env::current_exe().unwrap_or_default();
        let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
        exe_dir.join("config.toml")
    }

    /// 加载配置文件
    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if path.exists() {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("无法读取配置文件: {:?}", path))?;
            let config: Config = toml::from_str(&content)
                .with_context(|| "配置文件格式错误")?;
            Ok(config)
        } else {
            // 创建默认配置文件
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        let content = self.to_toml_string();
        fs::write(&path, content)
            .with_context(|| format!("无法写入配置文件: {:?}", path))?;
        Ok(())
    }

    /// 生成带注释的 TOML 字符串
    fn to_toml_string(&self) -> String {
        format!(
            r#"# =====================================================
# 窗口置顶工具配置文件
# Window TopMost Tool Configuration
# =====================================================

# 快捷键配置
# Hotkey Configuration
[hotkey]
# 修饰键 (true/false)
ctrl = {}
alt = {}
shift = {}
win = {}

# 主键 (支持: A-Z, 0-9, F1-F12, Space, Enter, Tab, Escape, 方向键等)
# Examples: "Space", "T", "F9", "Enter"
key = "{}"

# 设置
# Settings
[settings]
# 是否显示提示气泡 (true/false)
show_notification = {}

# 是否播放提示音 (true/false)
play_sound = {}
"#,
            self.hotkey.ctrl,
            self.hotkey.alt,
            self.hotkey.shift,
            self.hotkey.win,
            self.hotkey.key,
            self.settings.show_notification,
            self.settings.play_sound,
        )
    }
}
