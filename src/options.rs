use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    optimize_level: u8,

    #[arg(short, long)]
    disables: Vec<String>,
}
