use std::backtrace::Backtrace;

#[derive(Debug)]
pub struct WithBacktrace<E> {
    pub error: E,
    pub backtrace: Backtrace,
}

impl<E> WithBacktrace<E> {
    pub fn new(error: E) -> Self {
        Self {
            error,
            backtrace: Backtrace::capture(),
        }
    }
}

impl<E: std::fmt::Display> std::fmt::Display for WithBacktrace<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nBacktrace:\n{}", self.error, self.backtrace)
    }
}

impl<E: std::fmt::Debug + std::fmt::Display + std::error::Error + 'static> std::error::Error
    for WithBacktrace<E>
{
}

#[macro_export]
macro_rules! impl_from_with_backtrace {
    ($err_ty:ty, $enum_ty:ident :: $variant:ident) => {
        impl From<$err_ty> for $enum_ty {
            fn from(e: $err_ty) -> Self {
                $enum_ty::$variant(WithBacktrace::new(e))
            }
        }
    };
}
