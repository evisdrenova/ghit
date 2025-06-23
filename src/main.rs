use clap::Parser;

mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = true)]
    dry_run: bool,
    #[arg(short, long)]
    branch: bool,
}

fn main() {
    let args = Args::parse();

    let cfg = config::Config::load();

    println!("the config {:?}", cfg);
}
