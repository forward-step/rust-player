use std::{fs::File, io::BufReader, time::Duration};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use super::{media::Media, PlayItem};

pub struct Player {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,                 // 音轨
    pub current_time: Duration, // 当前播放时间
    pub total_time: Duration,   // 总的播放时间
}

impl Player {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            current_time: Duration::from_secs(0),
            total_time: Duration::from_secs(0),
            sink,
            _stream: stream,
            _stream_handle: stream_handle,
        }
    }

    // 设置音量
    pub fn set_volume(&mut self, value: f32) {
        self.sink.set_volume(value)
    }

    // 获取音量
    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    // 添加播放源
    fn source(&mut self, play_item: &PlayItem, duration: Duration) {
        match &play_item.media {
            Media::LocalFile { path, .. } => {
                let file = File::open(path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                self.sink.append(source.skip_duration(duration));
            }
        }
    }

    // 单音乐播放
    pub fn play(&mut self, play_item: &PlayItem) {
        self.play_offset(play_item, Duration::from_secs(0))
    }

    // 指定开始播放位置
    pub fn play_offset(&mut self, play_item: &PlayItem, duration: Duration) {
        self.sink.clear();
        self.source(play_item, duration);
        self.sink.play();
    }

    // 暂停
    pub fn pause(&mut self) {
        self.sink.pause();
    }

    // 恢复
    pub fn resume(&mut self) {
        self.sink.play();
    }

    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    // 清空播放
    pub fn clear(&self) {
        self.sink.clear();
    }
}
