use std::path::PathBuf;

use clap::Parser;

use script::Script;
use ui::UI;

use crate::config::Config;

/// a tui application for audio player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// open dir
    path: Option<PathBuf>,

    /// search for file names and add them to the playlist
    name: Option<String>,

    /// full screen
    #[arg(short, long)]
    full: bool,

    /// show info
    #[arg(short, long)]
    info: bool,
}

mod config;
mod media;
mod script;
mod ui;
mod utils;

fn main() -> Result<(), std::io::Error> {
    let args: Args = Args::parse();
    let current_dir = std::env::current_dir().unwrap();

    let path = match args.path {
        Some(path) => {
            if path.is_absolute() {
                path
            } else {
                current_dir.join(path)
            }
        }
        None => PathBuf::from(&current_dir),
    };

    if !path.exists() {
        println!("{}: {:?}", Config::TIP_DIR_IS_NOT_FOUND, path);
        return Ok(());
    }

    let full_screen = args.full;
    let show_info = args.info;
    let mut script = Script::new()?;

    if path.is_file() {
        script.add_local_file_to_play_list(&path);

        let path = path.parent().unwrap_or(&path).to_path_buf();
        script.set_current_dir(&path);
    } else if path.is_dir() {
        script.set_current_dir(&path);
        if let Some(filename) = args.name {
            let filename = filename.replace(".", r"\.").replace("*", r".");
            let items = utils::search_directory(&path, &filename);
            for it in items {
                script.add_local_file_to_play_list(&it)
            }
        }
    }

    UI::new(script, full_screen, show_info)?;

    Ok(())
}
