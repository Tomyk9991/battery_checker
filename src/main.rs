use crate::mouse_checker::{Rival650, RivalError};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

pub mod mouse_checker;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
struct Cli {
    /// Show the battery of Rival 650
    #[arg(short, long)]
    show_battery: bool,
}

fn main() -> Result<(), RivalError> {
    let args = Cli::parse();

    let mut rival_650 = Rival650::new();
    rival_650.connect()?;

    if args.show_battery {
        let level = rival_650.battery_level();
        let bar = ProgressBar::new(100);
        let unicode = match rival_650.get_is_wired() {
            true => { "🔌" }
            false => { "🪫" }
        };
        let s = format!("Battery {}% {}:", level, unicode);

        bar.set_style(ProgressStyle::default_bar()
            .template(&(s + "[{wide_bar:.cyan/blue}]"))
            .unwrap()
            .progress_chars("#>-"));

        bar.inc(level as u64);
        bar.finish();
    }


    return Ok(())
}
