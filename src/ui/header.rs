use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    text::Text,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::config::Config;

use super::common::Common;

pub fn draw_header(frame: &mut Frame, area: Rect, text: &str) {
    let block = Common::block()
        .title(Config::TITLE_SOFTWARE)
        .title_alignment(Alignment::Left);
    let msg_p = Paragraph::new(Text::from(text.to_string()))
        .white()
        .alignment(Alignment::Center)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(msg_p, area);
}
