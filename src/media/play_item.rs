use std::{
    cmp,
    ops::Add,
    time::{Duration, Instant},
};

use crate::{config::Config, media::media::Media};

// 播放状态
#[derive(PartialEq, Eq, Clone, Copy)]
enum PlayStatus {
    Playing(Instant, Duration), // 1. 播放的时间戳 2. 播放时累计的播放时长
    Wait(Duration),             // 已经播放的时长
}

#[derive(Clone)]
pub struct PlayItem {
    pub media: Media,
    status: PlayStatus, // 播放状态
}

impl PlayItem {
    pub fn new(media: Media) -> Self {
        Self {
            media,
            status: PlayStatus::Wait(Duration::from_secs(0)),
        }
    }
    pub fn play(&mut self) {
        self.status = PlayStatus::Playing(Instant::now(), Duration::from_secs(0));
    }
    pub fn play_offset(&mut self, d: Duration) {
        self.status = PlayStatus::Playing(Instant::now(), d);
    }
    // 暂停播放
    pub fn pause(&mut self) {
        if let PlayStatus::Playing(now, d) = self.status {
            self.status = PlayStatus::Wait(now.elapsed().add(d));
        }
    }
    // 恢复播放
    pub fn resume(&mut self) {
        if let PlayStatus::Wait(d) = self.status {
            self.status = PlayStatus::Playing(Instant::now(), d);
        }
    }
    // 前进
    pub fn forward(&mut self) {
        let max_postion = self.media.get_duration();

        self.pause();
        if let PlayStatus::Wait(d) = self.status {
            let d = cmp::min(max_postion, d.add(Config::FORWARD_AND_REVERSE_STEP));
            self.status = PlayStatus::Wait(d);
        }
    }
    // 后退
    pub fn reverse(&mut self) {
        self.pause();
        if let PlayStatus::Wait(d) = self.status {
            self.status = PlayStatus::Wait(
                d.checked_sub(Config::FORWARD_AND_REVERSE_STEP)
                    .unwrap_or(Duration::from_secs(0)),
            );
        }
    }
    // 进度
    pub fn progress(&self) -> (Duration, Duration) {
        let current = match self.status {
            PlayStatus::Playing(now, d) => now.elapsed().add(d),
            PlayStatus::Wait(d) => d,
        };
        (current, self.media.get_duration())
    }

    pub fn is_playing(&self) -> bool {
        if let PlayStatus::Playing(_, _) = self.status {
            true
        } else {
            false
        }
    }
}
