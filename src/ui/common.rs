use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::config::Config;

pub struct Common;

/// common style
impl Common {
    pub const THEME: Color = Color::Cyan;
    pub const LIGHT_THEME:Color = Color::LightCyan;
    pub const LINE_GAUGE_BACKGROUND: Color = Color::White; // 进度条颜色

    pub fn block<'a>() -> Block<'a> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
    }
    pub fn list<'a, T>(items: T) -> List<'a>
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
    {
        List::new(items)
            .highlight_style(Style::default().bg(Common::THEME)) // 高亮样式
            .highlight_symbol(Config::LIST_PREFIX_SYMBOL) // 列表前缀
    }
}
