#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::complexity,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    future_incompatible,
    keyword_idents,
    let_underscore,
    missing_docs,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unreachable_pub,
    unused
)]
#![warn(clippy::nursery)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
