mod archiver_error;
pub mod macros;

pub use archiver_error::{ArchiverError, StackFrame};

#[test]
fn test_error_display() {
    let e = crate::err_fatal_from_str!("asdf");
    e.display();
    let e = crate::err_warn_from_str!("asdf");
    e.display();
    let e = crate::err_info_from_str!("asdf");
    e.display();
}
