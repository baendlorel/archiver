mod archiver_error;
pub mod macros;

pub use archiver_error::{ArchiverError, StackFrame};

#[test]
fn test_error_display() {
    let e = err_fatal_from_str!("asdf");
    e.display();
    let e = err_warn_from_str!("asdf");
    e.display();
    let e = err_info_from_str!("asdf");
    e.display();
}
