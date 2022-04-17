# UnTeX

[![Crates.io](https://img.shields.io/crates/v/untex)](https://crates.io/crates/untex)
[![docs.rs](https://img.shields.io/docsrs/untex)](https://docs.rs/untex)

UnTeX is both a library and an executable that allows you to manipulate and understand TeX files.

#### Executable

If you wish to use the executable, you can install it with Cargo:
```
cargo install untex
```

##### Usage

```
untex --help
untex deps <FILE>
untex explain <FILE>
```

#### Library

You can use UnTeX in your Rust project by adding to your `Cargo.toml`:
```toml
untex = "version"
```

##### Documentation

Automatically generated documentation can found found [here](https://docs.rs/untex). Many functions are missing docs, but it's on the TODO list!

### Disclaimers

As this project is under active development, expect non backward compatible changes from version to version. Before reaching **v1.x.x**, UnTeX will be considered as unstable.

### TODO List

- [ ] Define command regexes in a config files (and maybe some can be hardcoded)
- [ ] Recursively lex files that are given by \input commands
- [ ] Keep fileno reference inside lexers
- [ ] Document functions
- [ ] Better general error handling
- [ ] Handle BrokenPipe errors
- [ ] ...

## Contributing

Contributions are more than welcome!
