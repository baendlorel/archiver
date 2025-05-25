mod archiver_error;
pub mod macros;

pub use archiver_error::{ArchiverError, StackFrame};

#[test]
fn test_error_display() {
    let e = crate::err_fatal!("asdf");
    e.display();
    let e = crate::err_warn!("asdf");
    e.display();
    let e = crate::err_info!("asdf");
    e.display();
}
