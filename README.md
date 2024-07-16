Crate template
==============

A [cargo-generate][] template for a library crate including:

* [cargo-readme-sync]
* [typos]
* github actions
* badges
* MIT license

Usage
-----
```bash
cargo install cargo-generate
read -p "Enter the crate name: " crate
cargo generate --git https://github.com/imbolc/crate-template.git --name $crate
cd ./$crate && ./.pre-commit.sh
```

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[cargo-readme-sync]: https://github.com/phaazon/cargo-sync-readme
[typos]: https://github.com/crate-ci/typos 
