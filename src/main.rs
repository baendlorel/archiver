use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    restore: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.list {
        println!("现有列表：1,2,3");
    }
    if let Some(obj_path) = args.restore {
        println!("要复原obj_path：{}", obj_path.yellow());
    }
}
