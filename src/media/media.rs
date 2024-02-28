use std::{path::PathBuf, time::Duration};

use id3::{Tag, TagLike};
use rodio::Source;

use super::lyrics::Lyrics;

#[derive(Clone)]
pub enum Media {
    LocalFile {
        path: PathBuf,
        name: String,
        duration: Duration,
        lyrics: Lyrics,
        tag: Option<Tag>,
    },
}

impl Media {
    pub fn new_local_file(path: PathBuf) -> Option<Self> {
        if let Some(file_name) = path.file_name() {
            let tag = Tag::read_from_path(&path).ok();
            let lyrics = if let Some(tag) = &tag {
                let mut string = String::default();
                for it in tag.lyrics() {
                    string = string + &it.text;
                }
                if string.len() > 0 {
                    Lyrics::from_string(string)
                } else {
                    Lyrics::from_music_path(&path)
                }
            } else {
                Lyrics::from_music_path(&path)
            };

            Some(Media::LocalFile {
                path: PathBuf::from(path.clone()),
                name: file_name.to_string_lossy().to_string(),
                duration: Self::duration(&path),
                lyrics,
                tag,
            })
        } else {
            None
        }
    }
}

impl Media {
    // 获取播放名
    pub fn get_name(&self) -> String {
        match self {
            Media::LocalFile { name, .. } => String::from(name),
        }
    }
    pub fn get_duration(&self) -> Duration {
        match self {
            Media::LocalFile { duration, .. } => *duration,
        }
    }
    // 获取本地文件播放时长
    fn duration(path: &PathBuf) -> Duration {
        if let Ok(dur) = mp3_duration::from_path(path) {
            return dur;
        } else if let Ok(file) = std::fs::File::open(path) {
            if let Ok(decoder) = rodio::Decoder::new(file) {
                if let Some(dur) = decoder.total_duration() {
                    return dur;
                }
            }
        }
        return Duration::default();
    }
    // 获取歌词
    pub fn get_lyrics(&self) -> &Lyrics {
        match self {
            Media::LocalFile { lyrics, .. } => lyrics,
        }
    }
    // 获取id3 tag
    pub fn get_id3_tag(&self) -> Vec<(String, String, String)> {
        match self {
            Media::LocalFile { tag, .. } => {
                let mut items = vec![];

                if let Some(tag) = tag {
                    // TIT2 ; 歌曲标题名字
                    if let Some(title) = tag.title() {
                        items.push(("TIT2".to_string(), "标题".to_string(), title.to_string()))
                    }
                    // TPE1 ; 主要表演者/独奏者
                    if let Some(artist) = tag.artist() {
                        items.push(("TPE1".to_string(), "表演者".to_string(), artist.to_string()))
                    }
                    // TALB ; 专辑/电影/节目名称
                    if let Some(album) = tag.album() {
                        items.push(("TALB".to_string(), "专辑".to_string(), album.to_string()))
                    }
                    // TPE2 ; 乐队/管弦乐团/伴奏
                    if let Some(album_artist) = tag.album_artist() {
                        items.push((
                            "TPE2".to_string(),
                            "专辑集艺术家".to_string(),
                            album_artist.to_string(),
                        ))
                    }
                    // TCOM ; 作曲家
                    if let Some(tcom) = tag.get("TCOM").and_then(|frame| frame.content().text()) {
                        items.push(("TCOM".to_string(), "作曲家".to_string(), tcom.to_string()))
                    }
                    // TPOS ; 集数
                    if let Some(d) = tag.disc() {
                        items.push(("TPOS".to_string(), "集数".to_string(), d.to_string()))
                    }
                    // TCON ; 内容类型, 如流派、风格等
                    if let Some(genre) = tag.genre() {
                        items.push(("TCON".to_string(), "流派".to_string(), genre.to_string()))
                    }
                    // // TRCK ; 曲目编号/集合中位置
                    // if let Some(track) = tag.track() {
                    //     println!("track: {}", track)
                    // }
                    // TYER ; 年份
                    if let Some(year) = tag.year() {
                        items.push(("TYER".to_string(), "年份".to_string(), year.to_string()))
                    }
                    // COMM ; 注释
                    let mut comment = String::default();
                    for it in tag.comments() {
                        comment = comment + &it.text
                    }
                    if comment.len() > 0 {
                        items.push(("COMM".to_string(), "注释".to_string(), comment))
                    }

                    // 歌词
                    // for it in tag.lyrics() {}

                    // APIC ; 图片
                    // for it in tag.pictures() {}
                }

                items
            }
        }
    }
}
