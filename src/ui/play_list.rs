use ratatui::{layout::Rect, widgets::ListItem, Frame};

use crate::{config::Config, script::Script};

use super::common::Common;

pub fn draw_play_list(frame: &mut Frame, area: Rect, script: &mut Script) {
    let mut items = vec![];

    for it in &script.play_list {
        items.push(ListItem::new(it.media.get_name()));
    }

    let block = Common::block().title(Config::TITLE_PLAY_LIST);
    let file_list = Common::list(items).block(block);
    frame.render_stateful_widget(file_list, area, &mut script.play_list_index);
}
