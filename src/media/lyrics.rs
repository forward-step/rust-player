use std::{cmp, fs::File, io::Read, path::PathBuf, time::Duration};

use regex::Regex;

#[derive(Clone)]
pub struct Lyric {
    pub time: Duration,  // 播放时刻
    pub content: String, // 歌词内容
}

#[derive(Clone)]
pub struct Lyrics {
    pub list: Vec<Lyric>,
}

impl Lyrics {
    const LRC_REGEX: &'static str = r"\[(?P<min>\d+):(?P<sec>\d+).(?P<ms>\d+)](?P<content>[^\[\]]*)";

    // 读取歌词文件
    pub fn from_music_path(s: &PathBuf) -> Self {
        let mut p = PathBuf::from(s);
        p.set_extension("lrc");
        if let Ok(ref mut file) = File::open(p) {
            Self::from_read(file)
        } else {
            Self { list: vec![] }
        }
    }

    // 解析歌词文件
    pub fn from_read(f: &mut File) -> Self {
        let mut buffer = vec![];
        let _ = f.read_to_end(&mut buffer);
        let m = String::from_utf8(buffer).unwrap_or(String::default());

        Self::from_string(m)
    }

    pub fn from_string(strs: String) -> Self {
        let regex = Regex::new(Lyrics::LRC_REGEX).unwrap();

        let list = regex
            .captures_iter(strs.as_str())
            .map(|cap| {
                let min = cap["min"].parse::<u64>().unwrap_or(0);
                let sec = cap["sec"].parse::<u64>().unwrap_or(0);
                let sec = cmp::min(cmp::max(sec, 0), 59); // 0s-59s
                let ms = cap["ms"].parse::<u64>().unwrap_or(0);
                let ms = cmp::min(cmp::max(ms, 0), 999); // 0ms-999ms
                let dur = Duration::from_millis(ms + sec * 1000 + min * 1000 * 60);

                Lyric {
                    time: dur,
                    content: String::from(&cap["content"]),
                }
            })
            .collect();

        Self { list }
    }
}
