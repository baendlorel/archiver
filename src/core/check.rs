use owo_colors::OwoColorize;
use std::collections::HashSet;
use std::ops::Deref;

use crate::core::{archive, auto_incr, config};
use crate::misc::{jsonl, mark, paths};
use crate::models::types::{ListStatus, LogEntry, Vault, VaultStatus};
use crate::traits::CustomColors;

macro_rules! must {
    ($e:expr) => {
        $e.unwrap_or_else(|error| panic!("{}", error.to_string()))
    };
}

/// 捕获panic并自定义错误信息插入errors
pub trait OnPanic<T> {
    fn on_panic(self, msg: impl AsRef<str>);
}

// todo 可能需要增加check完成后的统计
static mut INDEX: u32 = 0;
static mut VERBOSE: bool = false;

impl<T> OnPanic<T> for std::thread::Result<T> {
    fn on_panic(self, msg: impl AsRef<str>) {
        unsafe {
            INDEX += 1;
        }
        match self {
            Ok(_) => unsafe {
                if VERBOSE {
                    let idx = INDEX;
                    println!(
                        "{} {}. {}",
                        mark::succ(),
                        idx.to_string().green(),
                        msg.as_ref()
                    );
                }
            },
            Err(e) => {
                let panic_msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };

                unsafe {
                    let idx = INDEX;
                    // 错误的输出要开头结尾各多空出一行
                    println!(
                        "\n{} {}. {}\n    {}\n",
                        mark::fatal(),
                        idx.to_string().red(),
                        msg.as_ref(),
                        panic_msg.replace("\n", "\n    ")
                    );
                };
            }
        }
    }
}

fn it<F, R>(m: impl AsRef<str>, f: F)
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).on_panic(m);
}

/// 检查Archiver所有文件内容是否符合逻辑
pub fn check(verbose: bool) {
    unsafe {
        VERBOSE = verbose;
    };

    // 防止panic输出到控制台，我要统一捕获
    std::panic::set_hook(Box::new(|_| {}));

    // 各个目录
    it("Home directory is valid", || paths::HOME_DIR.exists());
    it("Root directory is valid", || paths::ROOT_DIR.exists());
    it("Current working directory is valid", || paths::CWD.exists());
    it("Logs directory is valid", || paths::LOGS_DIR.exists());
    it("Core directory is valid", || paths::CORE_DIR.exists());
    it("Vaults directory is valid", || paths::VAULTS_DIR.exists());

    // 配置文件
    it("Configuration can be loaded", || config::CONFIG.deref());

    // archive list的文件都存在，位置都对
    it("Archived items exist and are correct", || {
        let list = must!(archive::list::find_all());
        let mut errs = vec![];
        for entry in list {
            let p = paths::get_archived_path(entry.id, entry.vault_id);
            let head = format!(
                "id: {}({})",
                entry.id.styled_id(),
                entry.item.styled_comment()
            );

            macro_rules! add {
                ($($arg:tt)*) => {{
                    let msg = format!($($arg)*);
                    errs.push(format!("{} '{}' {}", head, p.display(), msg));
                }};
            }

            // 在库状态的记录，也要有对应的文件才行
            if !p.exists() && matches!(entry.status, ListStatus::Archived) {
                add!("does not exist.");
            }
            if p.exists() && matches!(entry.status, ListStatus::Restored) {
                add!("is marked restored but is still there.");
            }

            // 路径实际上是否为目录，必须和列表记录一致
            if p.is_dir() == entry.is_dir {
                let t = if p.is_dir() { "directory" } else { "file" };
                add!("is actually a {} but records not", t);
            }
        }
    });

    // vault list都存在，位置都对
    it("Vaults exist and are correct", || {
        let vaults = must!(jsonl::load::<Vault>(&paths::VAULTS_FILE_PATH));
        let mut errs = vec![];
        for vault in vaults {
            let p = paths::get_vault_path(vault.id);
            let head = format!("{}({})", vault.name.styled_vault(), vault.id.styled_vault());

            macro_rules! add {
                ($($arg:tt)*) => {{
                    let msg = format!($($arg)*);
                    errs.push(format!("{} '{}' {}", head, p.display(), msg));
                }};
            }

            if !p.exists() && !matches!(vault.status, VaultStatus::Removed) {
                add!("does not exist.");
            }

            if p.exists() && !p.is_dir() {
                add!("is not a directory.");
            }
        }
    });

    // 自增数据是合理的
    // 不存在重复id
    it("Auto Increment is correct. No duplicated ids.", || {
        let mut local_idx: u32 = 0;
        let mut errs: Vec<String> = vec![];
        macro_rules! add {
          ($($arg:tt)*) => {{
              local_idx+=1;
              let msg = format!($($arg)*);
              errs.push(format!("{} - {}", local_idx.styled_const(), msg));
          }};
        }

        let mut max_aid = 0;
        let mut aid_set: HashSet<u32> = HashSet::new();
        let arr = must!(archive::list::find_all());
        arr.iter().for_each(|entry| {
            max_aid = max_aid.max(entry.id);
            if !aid_set.insert(entry.id) {
                add!("Duplicate archive id found: {}", entry.id.styled_id());
            }
        });

        let mut max_vid = 0;
        let mut vid_set: HashSet<u32> = HashSet::new();
        let arr = must!(jsonl::load::<Vault>(&paths::VAULTS_FILE_PATH));
        arr.iter().for_each(|entry| {
            max_vid = max_vid.max(entry.id);
            if !vid_set.insert(entry.id) {
                add!("Duplicate vault id found: {}", entry.id.styled_id());
            }
        });

        let mut max_lid = 0;
        let mut lid_set: HashSet<u32> = HashSet::new();
        let years = paths::get_years_desc();
        for y in years {
            let p = paths::get_log_path(y);
            let arr = must!(jsonl::load::<LogEntry>(&p));
            arr.iter().for_each(|entry| {
                max_lid = max_lid.max(entry.id);
                if !lid_set.insert(entry.id) {
                    add!("Duplicate log id found: {}", entry.id.styled_id());
                }
            });
        }

        // 检查是否能和auto-incr对得上
        let aid = auto_incr::peek_next("archive_id");
        let vid = auto_incr::peek_next("vault_id");
        let lid = auto_incr::peek_next("log_id");
        if aid <= max_aid {
            add!(
                "Next archive id {} is not greater than the maximum id {}.",
                aid.styled_id(),
                max_aid.styled_id()
            );
        }
        if vid <= max_vid {
            add!(
                "Next vault id {} is not greater than the maximum id {}.",
                vid.styled_vault(),
                max_vid.styled_vault()
            );
        }
        if lid <= max_lid {
            add!(
                "Next log id {} is not greater than the maximum id {}.",
                lid.styled_id(),
                max_lid.styled_id()
            );
        }

        if errs.len() > 0 {
            panic!("{}", errs.join("\n"));
        }
    });
}
