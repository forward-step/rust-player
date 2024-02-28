use std::{ffi::OsStr, fs::DirEntry, time::Duration};

pub struct Config {}

impl Config {
    // UI刷新速率 ; 1000ms / 60fps = 16.67ms
    pub const REFRESH_RATE: Duration = Duration::from_millis(50);
    // 前进后退间隔时间
    pub const FORWARD_AND_REVERSE_STEP: Duration = Duration::from_secs(1);
    // 支持的文件后缀
    const ACCEPT_SUFFIX: [&'static str; 4] = ["mp3", "wav", "flac", "ts"];

    // layout
    pub const LAYOUT_LYRICS_WIDGET_WIDTH: u16 = 30; // 歌词组件宽度
    pub const LAYOUT_LYRICS_WIDGET_WIDTH_ON_FULL_SCREEN: u16 = 50; // 歌词组件全屏时宽度
    pub const LAYOUT_INFO_WIDGET_WIDTH: u16 = 20; // 信息组件宽度
    pub const LAYOUT_INFO_WIDGET_WIDTH_ON_FULL_SCREEN: u16 = 40; // 全屏状态下宽度

    // text
    pub const EMPTY: &'static str = "-"; // 空文本
    pub const FILE_SYSTEM_BACK_SYMBOL: &'static str = ".."; // 返回上一级
    pub const LIST_PREFIX_SYMBOL: &'static str = "> "; // 列表前缀标记

    // tip
    #[cfg(feature = "zh")]
    pub const TIP_DIR_IS_NOT_FOUND: &'static str = "目录不存在";
    #[cfg(feature = "en")]
    pub const TIP_DIR_IS_NOT_FOUND: &'static str = "path is not found";

    // tui block title
    pub const TITLE_SOFTWARE: &'static str = "RustPlayer - Player For Rust";
    #[cfg(feature = "zh")]
    pub const TITLE_LYRICS: &'static str = "歌词";
    #[cfg(feature = "en")]
    pub const TITLE_LYRICS: &'static str = "Lyrics";
    #[cfg(feature = "zh")]
    pub const TITLE_MEIDA_INFO: &'static str = "信息";
    #[cfg(feature = "en")]
    pub const TITLE_MEIDA_INFO: &'static str = "info";
    #[cfg(feature = "zh")]
    pub const TITLE_PLAY_LIST: &'static str = "播放列表";
    #[cfg(feature = "en")]
    pub const TITLE_PLAY_LIST: &'static str = "Play List";
    #[cfg(feature = "zh")]
    pub const TITLE_EXPLORER: &'static str = "文件夹";
    #[cfg(feature = "en")]
    pub const TITLE_EXPLORER: &'static str = "Explorer";
    #[cfg(feature = "zh")]
    pub const TITLE_CURRENT_FOLDER: &'static str = "当前文件夹";
    #[cfg(feature = "en")]
    pub const TITLE_CURRENT_FOLDER: &'static str = "Current Folder";
    #[cfg(feature = "zh")]
    pub const TITLE_NOW_PLAYING: &'static str = "正在播放";
    #[cfg(feature = "en")]
    pub const TITLE_NOW_PLAYING: &'static str = "Now Playing";
    #[cfg(feature = "zh")]
    pub const TITLE_COMING_SOON: &'static str = "即将播放";
    #[cfg(feature = "en")]
    pub const TITLE_COMING_SOON: &'static str = "Coming Soon";
    #[cfg(feature = "zh")]
    pub const TITLE_WAVE: &'static str = "音量";
    #[cfg(feature = "en")]
    pub const TITLE_WAVE: &'static str = "Wave";

    // shortcut key
    #[cfg(feature = "zh")]
    pub const SHORTCUT_KEY_PLAY_LIST: &'static str = "Enter(立即播放) T(置顶) Delete|Backspace(删除)";
    #[cfg(feature = "en")]
    pub const SHORTCUT_KEY_PLAY_LIST: &'static str = "Enter(play) T(top-post) Delete|Backspace(delete)";
    #[cfg(feature = "zh")]
    pub const SHORTCUT_KEY_EXPLORER: &'static str = "Enter(添加) Backspace(返回上一级)";
    #[cfg(feature = "en")]
    pub const SHORTCUT_KEY_EXPLORER: &'static str = "Enter(add) Backspace(back)";
    #[cfg(feature = "zh")]
    pub const SHORTCUT_KEY_COMMON: &'static str =
        "Tab(切换) Space(暂停|播放) I(详情) N(下一首) ⬅➡(前进后退) -+(调整音量) F(全屏) Q(退出)";
    #[cfg(feature = "en")]
    pub const SHORTCUT_KEY_COMMON: &'static str =
        "Tab(change) Space(pause|play) I(info) N(next) ⬅➡(forward|reverse) -+(volume) F(full screen) Q(quit)";

    // 可以解析的媒体文件
    pub fn is_accepted_file(f: &DirEntry) -> bool {
        let path = f.path();
        if path.is_file() {
            return if let Some(extension) = f.path().extension().and_then(OsStr::to_str) {
                Config::ACCEPT_SUFFIX
                    .iter()
                    .any(|&suffix| suffix == extension)
            } else {
                false
            };
        }
        false
    }

    // 可以解析的目录 ; 不显示隐藏目录
    pub fn is_accepted_dir(d: &DirEntry) -> bool {
        if d.path().is_dir() {
            let file_name = d.file_name().to_string_lossy().to_string();
            if file_name.starts_with('.') {
                return false;
            }
            return true;
        }
        false
    }
}
