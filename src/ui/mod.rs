use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{config::Config, script::Script};

use self::{fs::draw_fs, header::draw_header, media::draw_media, play_list::draw_play_list};

mod common;
mod fs;
mod header;
mod media;
mod play_list;

// Tab切换
#[derive(PartialEq, Eq)]
enum TabMode {
    Explorer, // 本地文件夹
    PlayList, // 播放列表
}

pub struct UI {
    script: Script,
    full_screen: bool, // 是否全屏展示
    show_info: bool,   // 是否展示media tag
    tab: TabMode,
}

impl UI {
    pub fn new(script: Script, full_screen: bool, show_info: bool) -> Result<(), std::io::Error> {
        let mut this = Self {
            script,
            full_screen,
            show_info,
            tab: TabMode::Explorer,
        };

        // 初始化终端
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // 刷新界面
        this.refresh(&mut terminal)?;

        // 恢复终端
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn refresh<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error>
    where
        B: Backend,
    {
        loop {
            // 初始化脚本
            self.script.init();
            // 绘制UI
            self.draw(terminal)?;
            std::thread::sleep(Config::REFRESH_RATE);
            // 处理事件
            if crossterm::event::poll(Config::REFRESH_RATE)? {
                if let Event::Key(key) = event::read()? {
                    // 是否可以操作目录列表
                    let can_operator_explorer = !self.full_screen && self.tab == TabMode::Explorer;
                    // 是否可以操作播放列表
                    let can_operator_play_list = !self.full_screen && self.tab == TabMode::PlayList;
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            // 关闭应用
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            // 列表操作
                            KeyCode::Up => {
                                if can_operator_explorer {
                                    self.script.on_list_up()
                                } else if can_operator_play_list {
                                    self.script.on_play_list_up()
                                }
                            }
                            KeyCode::Down => {
                                if can_operator_explorer {
                                    self.script.on_list_down()
                                } else if can_operator_play_list {
                                    self.script.on_play_list_down()
                                }
                            }
                            KeyCode::Enter => {
                                if can_operator_explorer {
                                    self.script.on_add_emdia_to_list()
                                } else if can_operator_play_list {
                                    self.script.on_play_now_play_list()
                                }
                            }
                            // 删除按钮
                            KeyCode::Delete => {
                                if can_operator_play_list {
                                    self.script.on_remove_from_play_list();
                                }
                            }
                            // 回退按钮
                            KeyCode::Backspace => {
                                if can_operator_play_list {
                                    self.script.on_remove_from_play_list();
                                } else if can_operator_explorer {
                                    self.script.on_back_parent();
                                }
                            }
                            // 切换展示info
                            KeyCode::Char('i' | 'I') => {
                                self.show_info = !self.show_info;
                            }
                            // 全屏播放 ; F11与大多数终端冲突了
                            KeyCode::Char('f' | 'F') => {
                                self.full_screen = !self.full_screen;
                            }
                            KeyCode::Esc => {
                                if self.full_screen {
                                    self.full_screen = false;
                                }
                            }
                            // 切换功能
                            KeyCode::Tab => match self.tab {
                                TabMode::Explorer => {
                                    self.tab = TabMode::PlayList;
                                    self.script.play_list_index.select(Some(0));
                                }
                                TabMode::PlayList => self.tab = TabMode::Explorer,
                            },
                            // 媒体播放
                            KeyCode::Media(media_key_code) => match media_key_code {
                                event::MediaKeyCode::Play => self.script.on_play(),
                                event::MediaKeyCode::Pause => self.script.on_pause(),
                                event::MediaKeyCode::PlayPause => {
                                    self.script.on_change_pause_play()
                                }
                                // event::MediaKeyCode::Reverse => todo!(),
                                // event::MediaKeyCode::Stop => todo!(),
                                // event::MediaKeyCode::FastForward => todo!(),
                                // event::MediaKeyCode::Rewind => todo!(),
                                // event::MediaKeyCode::TrackNext => todo!(),
                                // event::MediaKeyCode::TrackPrevious => todo!(),
                                // event::MediaKeyCode::Record => todo!(),
                                event::MediaKeyCode::LowerVolume => self.script.on_volume_decr(),
                                event::MediaKeyCode::RaiseVolume => self.script.on_volume_incr(),
                                // event::MediaKeyCode::MuteVolume => todo!(),
                                _ => {}
                            },
                            KeyCode::Char(' ') => self.script.on_change_pause_play(),
                            KeyCode::Char('n') => self.script.on_next(),
                            KeyCode::Char('-' | '_') => self.script.on_volume_decr(),
                            KeyCode::Char('=' | '+') => self.script.on_volume_incr(),
                            KeyCode::Char('t' | 'T') => {
                                if can_operator_play_list {
                                    self.script.on_post_top_play_list();
                                }
                            }
                            KeyCode::Left => self.script.on_reverse(),
                            KeyCode::Right => self.script.on_forward(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error>
    where
        B: Backend,
    {
        terminal.draw(|frame| {
            if self.full_screen {
                draw_media(
                    frame,
                    frame.size(),
                    &mut self.script,
                    Config::LAYOUT_LYRICS_WIDGET_WIDTH_ON_FULL_SCREEN,
                    Config::LAYOUT_INFO_WIDGET_WIDTH_ON_FULL_SCREEN,
                    self.show_info,
                )
            } else {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Percentage(100)])
                    .margin(2)
                    .split(frame.size());

                #[cfg(feature = "debug")]
                let text = self.script.header_text.clone();
                #[cfg(not(feature = "debug"))]
                let text = match self.tab {
                    TabMode::Explorer => Config::SHORTCUT_KEY_EXPLORER,
                    TabMode::PlayList => Config::SHORTCUT_KEY_PLAY_LIST,
                }
                .to_string()
                    + " "
                    + Config::SHORTCUT_KEY_COMMON;
                draw_header(frame, layout[0], &text);

                let main_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(layout[1]);
                match self.tab {
                    TabMode::Explorer => draw_fs(frame, main_layout[0], &mut self.script),
                    TabMode::PlayList => draw_play_list(frame, main_layout[0], &mut self.script),
                }
                draw_media(
                    frame,
                    main_layout[1],
                    &mut self.script,
                    Config::LAYOUT_LYRICS_WIDGET_WIDTH,
                    Config::LAYOUT_INFO_WIDGET_WIDTH,
                    self.show_info,
                );
            }
        })?;
        Ok(())
    }
}
