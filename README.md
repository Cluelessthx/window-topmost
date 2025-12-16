# 窗口置顶工具 (Window TopMost Tool) 📌

一个使用 Rust 编写的轻量级 Windows 窗口置顶工具，使用快捷键快速将任意窗口设置为置顶/取消置顶。

## 功能特性

- ✅ **快捷键置顶** - 一键切换窗口置顶状态
- ✅ **自定义快捷键** - 通过配置文件自定义快捷键组合
- ✅ **系统托盘** - 托盘图标，右键菜单操作
- ✅ **状态提示** - 声音提示反馈
- ✅ **轻量级** - 编译后体积小，资源占用低
- ✅ **无依赖** - 独立 EXE，无需安装运行时

## 使用方法

### 基本操作

1. 运行 `window-topmost.exe`，程序会在系统托盘显示图标
2. 点击要置顶的窗口，使其获得焦点
3. 按下快捷键（默认 `Ctrl + Space`）即可切换置顶状态
4. 再次按下快捷键可取消置顶

### 托盘菜单

右键点击托盘图标可以：

- 📌 **置顶当前窗口** - 手动触发置顶
- ⚙️ **打开配置文件** - 用记事本编辑配置
- 🔄 **重新加载配置** - 提示重启以应用新配置
- ❌ **退出** - 关闭程序

## 配置文件

配置文件 `config.toml` 会在首次运行时自动创建，与 exe 文件在同一目录。

```toml
# 快捷键配置
[hotkey]
# 修饰键 (true/false)
ctrl = true
alt = false
shift = false
win = false

# 主键 (支持: A-Z, 0-9, F1-F12, Space, Enter, Tab, Escape, 方向键等)
key = "Space"

# 设置
[settings]
# 是否显示提示气泡
show_notification = true

# 是否播放提示音
play_sound = true
```

### 快捷键配置示例

| 配置 | 实际快捷键 |
|------|-----------|
| `ctrl=true, key="Space"` | Ctrl + Space |
| `ctrl=true, key="T"` | Ctrl + T |
| `ctrl=true, alt=true, key="T"` | Ctrl + Alt + T |
| `win=true, key="T"` | Win + T |
| `key="F9"` | F9 |
| `ctrl=true, key="F9"` | Ctrl + F9 |

### 支持的按键

- **字母键**: A-Z
- **数字键**: 0-9
- **功能键**: F1-F12
- **特殊键**: Space, Enter, Tab, Escape, Backspace, Delete, Insert, Home, End, PageUp, PageDown
- **方向键**: Up, Down, Left, Right
- **数字键盘**: Numpad0-Numpad9

## 编译方法

### 前置要求

1. 安装 [Rust](https://www.rust-lang.org/tools/install)
2. Windows 系统 (需要 MSVC 工具链)

### 编译命令

```bash
# Debug 版本
cargo build

# Release 版本 (推荐，体积更小)
cargo build --release
```

编译后的 exe 文件位于：
- Debug: `target/debug/window-topmost.exe`
- Release: `target/release/window-topmost.exe`

### 添加自定义图标

1. 准备一个 `.ico` 格式的图标文件
2. 放置到 `assets/icon.ico`
3. 重新编译即可

## 开机自启

### 方法一：快捷方式

1. 右键 `window-topmost.exe` → 创建快捷方式
2. 按 `Win + R`，输入 `shell:startup`，回车
3. 将快捷方式移动到打开的文件夹中

### 方法二：注册表

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run]
"WindowTopMost"="\"C:\\path\\to\\window-topmost.exe\""
```

## 系统要求

- Windows 7 / 8 / 10 / 11
- x86_64 架构

## 项目结构

```
窗口置顶工具/
├── Cargo.toml          # 项目配置
├── build.rs            # 构建脚本（资源编译）
├── README.md           # 说明文档
├── assets/             # 资源文件
│   └── icon.ico        # 应用图标（可选）
└── src/
    ├── main.rs         # 程序入口
    ├── config.rs       # 配置管理
    ├── hotkey.rs       # 快捷键注册
    ├── tray.rs         # 系统托盘
    └── window.rs       # 窗口操作
```

## 常见问题

### Q: 快捷键没有反应？

1. 检查是否有其他程序占用了相同的快捷键
2. 尝试修改配置文件使用其他快捷键组合
3. 以管理员身份运行程序

### Q: 某些窗口无法置顶？

部分系统窗口或特殊应用可能有保护机制，无法被外部程序修改置顶状态。

### Q: 如何完全退出程序？

右键托盘图标 → 退出，或在任务管理器中结束进程。

### Q: 修改配置后不生效？

修改配置文件后需要重启程序才能生效。

## 技术实现

- 使用 Windows API `SetWindowPos` 设置 `HWND_TOPMOST` 标志
- 通过检测窗口扩展样式 `WS_EX_TOPMOST` 判断当前状态
- 使用 `RegisterHotKey` 注册全局快捷键
- 使用 Shell NotifyIcon API 实现系统托盘
