# Crate template

A [cargo-generate][] template for a library crate including:

- github actions and pre-commit lints
- crate badges
- MIT license

## Configured lints

- Clippy warnings
- Tests
- Nightly `rustfmt` with wrapped comments and grouped imports
- Unused Cargo dependencies with [cargo-shear][]
- [Typos][typos]
- `Cargo.toml` sorting with [cargo-sort][]

## Usage

```bash
cargo install cargo-generate
read -p "Enter the crate name: " crate
cargo generate --git https://github.com/imbolc/crate-template.git --name $crate
cd ./$crate && ./.pre-commit.sh
git add . && git commit -m'Init'
```

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[cargo-shear]: https://github.com/Boshen/cargo-shear
[cargo-sort]: https://github.com/DevinR528/cargo-sort
[typos]: https://github.com/crate-ci/typos
