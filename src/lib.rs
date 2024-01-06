//! Re-exports the crates for rustdoc.
//!
//! This crate itself is not intended to be used directly.

// With `custom-build` and `syn` crate, we can expand crate-level rustdocs.

macro_rules! re_export(($($name:ident),* $(,)?) => ($(pub mod $name { pub use ::$name::*; })*));

pub mod math {
    re_export!(divisors);
}

pub mod util {
    re_export!(io);
}
