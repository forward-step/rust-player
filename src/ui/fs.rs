use std::fs::DirEntry;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{config::Config, script::Script};

use super::common::Common;

pub fn draw_fs(frame: &mut Frame, area: Rect, script: &mut Script) {
    let fs_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)])
        .split(area);

    draw_fs_current_path(frame, fs_layout[0], script);
    draw_fs_list(frame, fs_layout[1], script);
}

fn draw_fs_current_path(frame: &mut Frame, area: Rect, script: &mut Script) {
    let folder = Paragraph::new(script.current_dir_string.clone())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(Common::block().title(Config::TITLE_CURRENT_FOLDER));
    frame.render_widget(folder, area);
}

fn draw_fs_list(frame: &mut Frame, area: Rect, script: &mut Script) {
    let mut items = vec![];
    if script.has_parent() {
        items.push(draw_back())
    }
    for it in &script.list {
        let path = it.path();
        if path.is_dir() {
            items.push(draw_dir_item(it))
        } else if path.is_file() {
            items.push(draw_file_item(it))
        }
    }

    let block = Common::block().title(Config::TITLE_EXPLORER);
    let file_list = Common::list(items).block(block);
    frame.render_stateful_widget(file_list, area, &mut script.list_index);
}

fn draw_back<'a>() -> ListItem<'a> {
    ListItem::new(Config::FILE_SYSTEM_BACK_SYMBOL)
}

fn draw_dir_item<'a>(entry: &DirEntry) -> ListItem<'a> {
    let file_name = String::from(entry.file_name().to_str().unwrap()) + "/";
    ListItem::new(file_name)
}

fn draw_file_item<'a>(entry: &DirEntry) -> ListItem<'a> {
    let file_name = String::from(entry.file_name().to_str().unwrap());
    ListItem::new(file_name)
}
