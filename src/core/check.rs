use std::ops::Deref;

use crate::{
    core::config,
    misc::{mark, paths},
    traits::CustomColors,
};

/// 捕获panic并自定义错误信息插入errors
pub trait OnPanic<T> {
    fn on_panic(self, msg: impl AsRef<str>);
}

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
                    println!("{} {}. {}\n", mark::succ(), idx.styled_id(), msg.as_ref());
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
                    println!(
                        "{} {}. {}\n  {}\n",
                        mark::fatal(),
                        idx.styled_id(),
                        msg.as_ref(),
                        panic_msg
                    );
                };
            }
        }
    }
}

fn it<F>(m: impl AsRef<str>, f: F)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).on_panic(m);
}

/// 检查Archiver所有文件内容是否符合逻辑
pub fn check(verbose: bool) {
    unsafe {
        VERBOSE = verbose;
    };

    // 检查配置文件
    it("Configuration can be loaded", || {
        let _ = config::CONFIG.deref();
    });

    it("Home directory is valid", || {
        let _ = *paths::HOME_DIR;
    });

    it("Root directory is valid", || {
        let _ = *paths::ROOT_DIR;
    });

    it("Current working directory is valid", || {
        let _ = *paths::CWD;
    });
}
