[![License](https://img.shields.io/crates/l/{{project-name}}.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})

<!-- cargo-sync-readme -->

## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything



### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[.pre-commit.sh]: https://github.com/{{github-user}}/{{project-name}}/blob/main/pre-commit.sh
