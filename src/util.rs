//! Utilities shared across the crate.

/// Convenience trait that allows wrapping a `syn::Error` with another error.
pub trait SynErrorContext {
    fn wrap_err(self, new: syn::Error) -> Self;

    fn wrap_err_with<F>(self, f: F) -> Self
    where
        F: FnOnce() -> syn::Error;
}

impl<T> SynErrorContext for syn::Result<T> {
    fn wrap_err(self, mut new: syn::Error) -> Self {
        self.map_err(|e| {
            new.combine(e);
            new
        })
    }

    fn wrap_err_with<F>(self, f: F) -> Self
    where
        F: FnOnce() -> syn::Error,
    {
        self.map_err(|e| {
            let mut new = f();
            new.combine(e);
            new
        })
    }
}

impl SynErrorContext for syn::Error {
    fn wrap_err(self, mut new: syn::Error) -> Self {
        new.combine(self);
        new
    }

    fn wrap_err_with<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        self.wrap_err(f())
    }
}

/// Print a debug trace message when the `debug-trace` feature is enabled.
/// This is a no-op when the `debug-trace` feature is disabled.
#[cfg(feature = "debug-trace")]
macro_rules! debug_trace {
    ($($tt:tt)*) => {
        {
            let message = format!($($tt)*);
            eprintln!("delegate_match debug @ {}:{}\t| {}", file!(), line!(), message);
        }
    };
}

/// Print a debug trace message when the `debug-trace` feature is enabled.
/// This is a no-op when the `debug-trace` feature is disabled.
#[cfg(not(feature = "debug-trace"))]
macro_rules! debug_trace {
    ($($tt:tt)*) => {
        if false {
            let _ = format_args!($($tt)*);
        }
    };
}

pub(crate) use debug_trace;
