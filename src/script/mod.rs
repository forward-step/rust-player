use std::time::Duration;
use std::{fs::DirEntry, path::PathBuf};

use ratatui::widgets::ListState;

use crate::config::Config;
use crate::media::{Media, PlayItem, Player};

#[derive(PartialEq, Clone, Copy)]
enum SortOrder {
    Asc,  // 升序 ; 默认
    Desc, // 降序
}

pub struct Script {
    current_dir: PathBuf,           // 当前目录
    sort_order: SortOrder,          // 排序方式
    pub list: Vec<DirEntry>,        // current_dir下的所有目录和文件
    pub list_index: ListState,      // 列表状态
    pub current_dir_string: String, // 当前目录 string
    pub volume: f32,                // 音量大小 ; [0, 1]
    pub player: Player,             // 音频播放控制器
    pub play_list: Vec<PlayItem>,   // 播放列表
    pub play_list_index: ListState,
    pub now_playing_name: String,
    pub next_play_name: String,
    pub header_text: String,
}

// 切换索引
fn change_list_index<F, F2>(
    len: usize,
    list_index: &mut ListState,
    change_index: F,
    deafult_index: F2,
) where
    F: Fn(usize) -> usize,
    F2: Fn(usize) -> usize,
{
    if let Some(selected) = list_index.selected() {
        list_index.select(Some((change_index(selected + len)) % len));
    } else {
        list_index.select(Some(deafult_index(len) % len))
    }
}

impl Script {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut list_index = ListState::default();
        list_index.select(Some(0));
        let player = Player::new();
        Ok(Self {
            current_dir: std::env::current_dir()?,
            list: vec![],
            list_index,
            sort_order: SortOrder::Asc,
            current_dir_string: String::default(),
            volume: player.volume(),
            player,
            play_list: vec![],
            play_list_index: ListState::default(),
            now_playing_name: String::default(),
            next_play_name: String::default(),
            header_text: String::default(),
        })
    }

    pub fn set_current_dir(&mut self, path: &PathBuf) {
        self.current_dir = PathBuf::from(path);
    }

    // 初始化脚本
    pub fn init(&mut self) {
        self.current_dir_string = self.current_dir.to_string_lossy().to_string();
        self.now_playing_name = if let Some(it) = self.play_list.first() {
            it.media.get_name()
        } else {
            String::default()
        };
        self.next_play_name = if let Some(it) = self.play_list.get(1) {
            it.media.get_name()
        } else {
            String::default()
        };

        // 设置音量
        self.player.set_volume(self.volume);

        // 播放完毕，下一首
        self.header_text = String::from(self.player.is_empty().to_string());
        if let Some(first) = self.play_list.first() {
            if first.is_playing() && self.player.is_empty() {
                self.on_next();
            }
        }

        self.list.clear();
        let mut dirs = vec![];
        let mut files = vec![];
        if self.current_dir.is_dir() {
            for entry in std::fs::read_dir(self.current_dir.clone()).unwrap() {
                if let Ok(entry) = entry {
                    if Config::is_accepted_dir(&entry) {
                        dirs.push(entry);
                    } else if Config::is_accepted_file(&entry) {
                        files.push(entry);
                    }
                }
            }
        }
        Self::sort(&mut dirs, SortOrder::Asc);
        Self::sort(&mut files, self.sort_order);
        self.list.extend(dirs);
        self.list.extend(files);
    }

    fn sort(arr: &mut Vec<DirEntry>, sort_order: SortOrder) {
        arr.sort_by(|a, b| {
            if sort_order == SortOrder::Asc {
                a.file_name().cmp(&b.file_name())
            } else if sort_order == SortOrder::Desc {
                b.file_name().cmp(&a.file_name())
            } else {
                a.file_name().cmp(&b.file_name())
            }
        })
    }

    // 当前目录是否存在上一级
    pub fn has_parent(&self) -> bool {
        self.current_dir.parent().is_some()
    }

    // 添加本地媒体文件
    pub fn add_local_file_to_play_list(&mut self, path: &PathBuf) {
        if let Some(media) = Media::new_local_file(PathBuf::from(path)) {
            self.play_list.push(PlayItem::new(media));
            if self.play_list.len() == 1 {
                self.play();
            }
        }
    }

    // 播放
    fn play(&mut self) {
        if let Some(play_item) = self.play_list.first_mut() {
            play_item.play();
            self.player.play(play_item);
        } else {
            self.player.clear();
        }
    }
    fn play_offset(&mut self, d: Duration) {
        if let Some(play_item) = self.play_list.first_mut() {
            self.player.play_offset(play_item, d);
            loop {
                if !self.player.is_empty() {
                    play_item.play_offset(d);
                    break;
                }
            }
        } else {
            self.player.clear();
        }
    }

    // 控制音量
    fn on_volume_change(&mut self, value: f32) {
        self.volume = self.volume + value;
        if self.volume < 0.0 {
            self.volume = 0.0;
        } else if self.volume > 1.0 {
            self.volume = 1.0;
        }
    }

    pub fn on_volume_incr(&mut self) {
        self.on_volume_change(0.1)
    }
    pub fn on_volume_decr(&mut self) {
        self.on_volume_change(-0.1)
    }
    pub fn on_list_down(&mut self) {
        let len = self.list.len();
        let len = if self.has_parent() { len + 1 } else { len };
        change_list_index(len, &mut self.list_index, |i| i + 1, |_| 0);
    }
    pub fn on_list_up(&mut self) {
        let len = self.list.len();
        let len = if self.has_parent() { len + 1 } else { len };
        change_list_index(len, &mut self.list_index, |i| i - 1, |len| len - 1);
    }
    pub fn on_play_list_down(&mut self) {
        let len = self.play_list.len();
        change_list_index(len, &mut self.play_list_index, |i| i + 1, |_| 0);
    }
    pub fn on_play_list_up(&mut self) {
        let len = self.play_list.len();
        change_list_index(len, &mut self.play_list_index, |i| i - 1, |len| len - 1);
    }
    // 添加到播放列表
    pub fn on_add_emdia_to_list(&mut self) {
        if let Some(selected) = self.list_index.selected() {
            let has_parent = self.has_parent();

            // 返回上一级
            if has_parent && selected == 0 {
                self.on_back_parent();
                return;
            }

            let index = if has_parent { selected - 1 } else { selected };
            if let Some(it) = self.list.get(index) {
                let path = it.path();
                // 进入目录
                if path.is_dir() {
                    self.current_dir = path;
                    self.list_index.select(Some(0));
                }
                // 添加到媒体文件
                else if path.is_file() {
                    self.add_local_file_to_play_list(&path);
                }
            }
        }
    }
    // 返回上一级菜单
    pub fn on_back_parent(&mut self) {
        if self.has_parent() {
            if let Some(d) = self.current_dir.parent() {
                self.current_dir = d.to_path_buf();
                self.list_index.select(Some(0));
            }
        }
    }
    // 置顶到播放列表
    pub fn on_post_top_play_list(&mut self) {
        if let Some(selected) = self.play_list_index.selected() {
            if selected != 0 {
                if let Some(elem) = self.play_list.get(selected).cloned() {
                    self.play_list.remove(selected);
                    self.play_list.insert(1, elem);
                }
            }
        }
    }
    // 插队播放当前歌曲
    pub fn on_play_now_play_list(&mut self) {
        if let Some(selected) = self.play_list_index.selected() {
            if selected != 0 {
                if let Some(elem) = self.play_list.get(selected).cloned() {
                    self.play_list.remove(selected);
                    self.play_list.remove(0);
                    self.play_list.insert(0, elem);
                    self.play();
                    self.play_list_index.select(Some(0));
                }
            }
        }
    }
    // 从播放列表删除
    pub fn on_remove_from_play_list(&mut self) {
        if let Some(selected) = self.play_list_index.selected() {
            if selected == 0 {
                self.on_next();
            } else if selected < self.play_list.len() {
                self.play_list.remove(selected);
            }
        }
    }
    // 播放下一首
    pub fn on_next(&mut self) {
        if self.play_list.len() > 0 {
            self.play_list.remove(0);
            self.play();
        }
    }
    // 暂停当前播放
    pub fn on_pause(&mut self) {
        if let Some(first) = self.play_list.first_mut() {
            first.pause();
            self.player.pause();
        }
    }
    // 恢复播放
    pub fn on_play(&mut self) {
        if let Some(first) = self.play_list.first_mut() {
            // 前进后退的时候，将不挂载资源
            if self.player.is_empty() {
                let (d, _) = first.progress();
                self.play_offset(d);
            } else {
                first.resume();
                self.player.resume();
            }
        }
    }
    // 切换暂停 or 播放
    pub fn on_change_pause_play(&mut self) {
        if let Some(first) = self.play_list.first_mut() {
            if first.is_playing() {
                self.on_pause();
            } else {
                self.on_play();
            }
        }
    }
    // 前进 ; 前进后退工程中，不挂载媒体资源，恢复播放的时候才挂载
    pub fn on_forward(&mut self) {
        if let Some(first) = self.play_list.first_mut() {
            first.forward();
            self.player.clear();
        }
    }
    // 后退
    pub fn on_reverse(&mut self) {
        if let Some(first) = self.play_list.first_mut() {
            first.reverse();
            self.player.clear();
        }
    }
}
