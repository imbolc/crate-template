#![forbid(
    unsafe_code,
    future_incompatible,
    keyword_idents,
    let_underscore,
    missing_docs,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rust_2024_compatibility
)]
#![warn(
    clippy::all,
    clippy::complexity,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used
)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
