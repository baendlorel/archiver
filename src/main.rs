use clap::Parser;
use owo_colors::OwoColorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 列出归档内容
    #[arg(short, long)]
    list: bool,

    /// 归档器操作日志
    #[arg(short, long)]
    log: bool,

    /// 从归档中恢复指定文件或目录
    #[arg(short, long)]
    restore: Option<String>,

    /// 要处理的文件目录地址，如果未指定其他命令，则对该目录进行移动操作
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    if args.list {
        println!("现有列表：1,2,3");
    } else if let Some(obj_path) = args.restore {
        println!("要复原obj_path：{}", obj_path.yellow());
    } else if let Some(path) = args.path {
        println!("对目录 {} 进行移动操作", path.display().to_string().green());
        // 这里实现移动目录的逻辑
    }
}
