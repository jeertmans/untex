# UnTeX

[![Crates.io](https://img.shields.io/crates/v/untex)](https://crates.io/crates/untex)
[![docs.rs](https://img.shields.io/docsrs/untex)](https://docs.rs/untex)

UnTeX is both a library and an executable that allows you to manipulate and understand TeX files.

### Usage

#### Executable

If you wish to use the executable, you can install it with Cargo:
```
cargo install untex --version 0.1.1-alha
```
*Warning: as UnTeX is still in alpha version, you must specify the version to download with Cargo.*

#### Library

You can use UnTeX in your Rust project by adding to your `Cargo.toml`:
```toml
untex = "0.2.0-alpha"
```

### Disclaimers

As this project is under active development, expect non backward compatible changes from version to version.

### TODO List

- [ ] Define command regexes in a config files (and maybe some can be hardcoded)
- [ ] Recursively lex files that are given by \input commands
- [ ] Keep fileno reference inside lexers
- [ ] Construct a list of file depedencies (\input, \includegraphics, ...)
- [ ] Document functions
- [ ] ...

## Contributing

Contributions are more than welcome!
