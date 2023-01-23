# UnTeX

[![Crates.io](https://img.shields.io/crates/v/untex)](https://crates.io/crates/untex)
[![docs.rs](https://img.shields.io/docsrs/untex)](https://docs.rs/untex)

UnTeX is both a library and an executable that allows you to manipulate and understand TeX files.

> **NOTE**: even though TeX and LaTeX are not the same, UnTeX assumes that TeX documents are written such that they will be parsed with some LaTeX engine. For "*pure*" TeX content, see the `src/lib/tex` module.

## Executable

The most convenient way of using UnTeX is through its command-line interface (CLI).

Currently, you can install it with Cargo:

```bash
cargo install untex --all-features
```

> **NOTE**: while using `--features cli` is sufficient to install UnTeX's CLI, using all features is recommend to take most benefits out of it!

### Usage

UnTeX has multiple commands, each one with a specific application:

* `check` for checking that a document will compile without error[*](#disclaimers);
* `dependendies`, or `deps`, for extracting dependencies from a TeX project;
* `expand` for expanding macros (e.g., `\input{...}` or `\include{...}`);
* `highlight`, or `hl`, for highlighting parts (e.g., comments) of TeX documents;
* `format` for pretty formatting your TeX files;
* `parse` for parsing and validating TeX documents[*](#disclaimers).


```bash
untex <COMMAND> [OPTIONS] [REQUIRED ARGS]...
```

## Library

You can use UnTeX in your Rust project by adding to your `Cargo.toml`:
```toml
untex = "^0.4.0"
```

### Documentation

Automatically generated documentation can found found [here](https://docs.rs/untex).

### Feature Flags

#### Default Features

- **color**: Adds support for output colorized text in the terminal with `termcolor`.
- **strum**: Uses `strum_macros`'s capabilities to enhance `Enum`s all across the library.

#### Optional Features

- **cli**: Adds command-line related methods for multiple structures. This feature is required to install UnTeX's CLI.
- **annotate**: (Soon) Adds method(s) to annotate results from check request. If **cli** feature is also enabled, the CLI will by default print an annotated output.
- **cli-complete**: (Soon) Adds commands to generate completion files for various shells. This feature also activates the **cli** feature. Enter `untex completions --help` for get help with installing completion files.
- **json**: (Soon) Adds the `json` output option.

### Disclaimers

As this project is under active development, expect non backward compatible changes from version to version. Before reaching **v1.x.x**, UnTeX will be considered as unstable.

### What is a valid (La)TeX document?

Parsing La(TeX) documents is very complicated, and the main reasons are detailes in the header of the documentation. Because of this, UnTex does not aim to be an exact parser, but a relatively good parser.

If you are in a situation where you think UnTeX produces a wrong result, please reach out to me, preferably via a [GitHub issue](https://github.com/jeertmans/untex/issues), and explain to me what you expected!

## Contributing

Contributions are more than welcome! Making UnTeX a good and reliable TeX tool is a matter of time and work, so any kind of help is a step towards a better UnTex!
