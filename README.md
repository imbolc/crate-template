Crate template
==============

A [cargo-generate][] template for a library crate including:

* [cargo-readme-sync]
* [rusty-hook]
* [typos]
* github actions
* badges
* MIT license

Usage
-----
```bash
cargo install cargo-generate cargo-readme rusty-hook typos-cli
read -p "Enter the crate name: " crate
cargo generate --git https://github.com/imbolc/crate-template.git --name crate
cd ./crate
rusty-hook init
```

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[cargo-readme-sync]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
[typos]: https://github.com/crate-ci/typos 
