use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("archiver")
        .required(false)
        .args(&["list", "log", "restore", "archive"]),
))]

struct Args {
    /// Show the list of archived objects
    #[arg(short, long, group = "archiver")]
    list: bool,

    /// Show the log of archiving operations
    #[arg(short = 'g', long, group = "archiver")]
    log: bool,

    /// Restore an archived object by its file/directory name or id
    #[arg(short, long, value_name = "name|id", group = "archiver")]
    restore: Option<String>,

    /// Archive a directory or file by its path.
    /// Will record file/directory name for future use
    #[arg(short, long, value_name = "path", group = "archiver")]
    archive: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.list {
        println!("现有列表：1,2,3");
    } else if args.log {
        println!("查看{}", "日志".green());
    } else if let Some(obj_path) = args.restore {
        println!("要复原obj_path：{}", obj_path.yellow());
    } else if let Some(path) = args.archive {
        println!("对目录 {} 进行移动操作", path.green());
        // 这里实现移动目录的逻辑
    }
}
