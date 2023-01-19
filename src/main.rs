use std::fmt::Debug;
use crate::mouse_checker::{Rival650, RivalError};
use clap::Parser;
use progressing::{Baring, mapping::Bar as MappingBar};

pub mod mouse_checker;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
struct Cli {
    /// Show the battery of Rival 650
    #[arg(short, long)]
    show_battery: bool,
}


fn get_width() -> u16 {
    termsize::get().map(|size| {
        size.cols
    }).unwrap()
}

fn main() -> Result<(), RivalError> {
    let args = Cli::parse();

    let mut rival_650 = Rival650::new();
    rival_650.connect()?;

    if args.show_battery {
        let level = rival_650.battery_level();

        let unicode = match rival_650.get_is_wired() {
            true => { "🔌" }
            false => { "🪫" }
        };

        let mut progress_bar = MappingBar::with_range(0, 100);
        progress_bar.set_len((get_width() - 26) as usize);
        progress_bar.set(level);
        let s = format!("Battery {}% {}: {}", level, unicode, progress_bar);
        println!("{}", s);
    }

    return Ok(())
}
