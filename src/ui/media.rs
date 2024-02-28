use std::{cmp, time::Duration};

use rand::Rng;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Bar, BarChart, BarGroup, LineGauge, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};

use crate::{config::Config, script::Script};

use super::common::Common;

pub fn draw_media(
    frame: &mut Frame,
    area: Rect,
    script: &mut Script,
    lyrics_width: u16,
    info_width: u16,
    show_info: bool,
) {
    let media_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);
    draw_header(frame, media_layout[0], script);
    draw_stage(
        frame,
        media_layout[1],
        script,
        lyrics_width,
        info_width,
        show_info,
    );
    draw_control(frame, media_layout[2], script);
}

fn draw_header(frame: &mut Frame, area: Rect, script: &mut Script) {
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let playing_text = if script.now_playing_name == "" {
        Config::EMPTY
    } else {
        &script.now_playing_name
    };
    let next_play_text = if script.next_play_name == "" {
        Config::EMPTY
    } else {
        &script.next_play_name
    };

    let now_playing = Paragraph::new(playing_text)
        .block(Common::block().title(Config::TITLE_NOW_PLAYING))
        .alignment(Alignment::Center)
        .style(Style::default());
    let next_playing = Paragraph::new(next_play_text)
        .block(Common::block().title(Config::TITLE_COMING_SOON))
        .alignment(Alignment::Center)
        .style(Style::default());
    frame.render_widget(now_playing, header_layout[0]);
    frame.render_widget(next_playing, header_layout[1]);
}

fn draw_stage(
    frame: &mut Frame,
    area: Rect,
    script: &mut Script,
    lyrics_width: u16,
    info_width: u16,
    show_info: bool,
) {
    let mut wave_chart = BarChart::default().block(Common::block().title(Config::TITLE_WAVE));

    let mut lyric_widget_length = 0;
    let mut lyric_widget = List::default();
    let mut lyric_index = ListState::default();

    let mut tag_widget_length = 0;
    let mut tag_widget = List::default();

    if let Some(first) = script.play_list.first() {
        // lyrics
        let lyrics = first.media.get_lyrics();
        let length = lyrics.list.len();
        if length > 0 {
            let mut items = vec![];
            let (current_pos, _) = first.progress();

            let (top_num, bottom_num) = {
                // border-top and border-bottom is 2
                let lines = (area.height as usize).checked_sub(2).unwrap_or(0);
                let half = lines / 2;
                if lines % 2 == 0 {
                    (half, half)
                } else {
                    (half + 1, half)
                }
            };

            let mid_index = match lyrics
                .list
                .iter()
                .enumerate()
                .find(|&(_, it)| it.time >= current_pos)
            {
                Some((value, _)) => value,
                None => length,
            };

            let end_index = cmp::min(mid_index + bottom_num, length - 1);
            let start_index = mid_index.checked_sub(top_num).unwrap_or(0);
            let selected_index = mid_index.checked_sub(start_index + 1).unwrap_or(0);

            lyric_index.select(Some(selected_index));
            for it in &lyrics.list[start_index..=end_index] {
                items.push(ListItem::new(it.content.clone()))
            }
            lyric_widget_length = lyrics_width;
            lyric_widget = Common::list(items).block(Common::block().title(Config::TITLE_LYRICS));
        }
        // wave
        if first.is_playing() {
            let mut items = vec![];
            let width = 3; // 柱子宽度
            let gap = 1; // 柱子间距
            let total = (area.width - gap) / (width + gap); // 柱子数量 ; width * total + gap * (total + 1) = area.width;
            let max_value = 100;
            for _ in 0..total {
                let value = rand::thread_rng().gen_range(0..=max_value);
                items.push(Bar::default().value(value).text_value("".to_string()))
            }
            wave_chart = BarChart::default()
                .block(
                    Common::block()
                        .title(Config::TITLE_WAVE)
                        .padding(Padding::horizontal(gap)),
                )
                .bar_width(width)
                .bar_gap(gap)
                .bar_style(Style::default().fg(Common::LIGHT_THEME).bg(Color::Black))
                .value_style(Style::default().fg(Common::LIGHT_THEME).bg(Color::Black))
                .data(BarGroup::default().bars(&items))
                .max(max_value);
        }
        // mp3 info
        let tags = first.media.get_id3_tag();
        if tags.len() > 0 && show_info {
            let mut items = vec![];
            for tag in tags.iter() {
                let text = if tag.0 == "COMM" {
                    tag.2.to_string()
                } else {
                    format!("{}: {}", tag.1, tag.2)
                };
                items.push(ListItem::new(text))
            }
            tag_widget_length = info_width;
            tag_widget = Common::list(items).block(Common::block().title(Config::TITLE_MEIDA_INFO));
        }
    }

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(lyric_widget_length),
            Constraint::Min(0),
            Constraint::Length(tag_widget_length),
        ])
        .split(area);

    frame.render_stateful_widget(lyric_widget, layout[0], &mut lyric_index);
    frame.render_widget(wave_chart, layout[1]);
    frame.render_widget(tag_widget, layout[2]);
}

fn draw_control(frame: &mut Frame, area: Rect, script: &mut Script) {
    let control_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(25), Constraint::Min(0)])
        .split(area);
    draw_volume(frame, control_layout[0], script);
    draw_progress(frame, control_layout[1], script);
}

fn draw_volume(frame: &mut Frame, area: Rect, script: &mut Script) {
    let volume = script.volume;
    let label = if let Some(first) = script.play_list.first() {
        if first.is_playing() {
            "||"
        } else {
            "▶"
        }
    } else {
        "VOL"
    };

    let volume = LineGauge::default()
        .block(Common::block())
        .label(label)
        .ratio(volume.into())
        .line_set(symbols::line::THICK)
        .gauge_style(
            Style::default()
                .fg(Common::LIGHT_THEME)
                .bg(Common::LINE_GAUGE_BACKGROUND)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(volume, area);
}

fn draw_progress(frame: &mut Frame, area: Rect, script: &mut Script) {
    let (current, total) = if let Some(first) = script.play_list.first() {
        first.progress()
    } else {
        (Duration::default(), Duration::default())
    };
    let current = current.as_secs();
    let total = total.as_secs();

    let current_m = current / 60;
    let current_s = current % 60;

    let total_m = total / 60;
    let total_s = total % 60;

    let ratio = if total <= 0 {
        0.0
    } else if total >= current {
        current as f64 / total as f64
    } else {
        1.0
    };

    let progress = LineGauge::default()
        .block(Common::block())
        .label(format!(
            "{:0>2}:{:0>2} / {:0>2}:{:0>2}",
            current_m, current_s, total_m, total_s
        ))
        .ratio(ratio)
        .line_set(symbols::line::THICK)
        .gauge_style(
            Style::default()
                .fg(Common::LIGHT_THEME)
                .bg(Common::LINE_GAUGE_BACKGROUND)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(progress, area);
}
