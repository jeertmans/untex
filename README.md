> [!WARNING]
> As of June 10, 2024, I'm officially archiving this project,
> as I haven't been able to put much time on this project ever since its start.
> Moreover, I think that lexing TeX code is a task that is way too hard, and I planning to phase
> to [Typst](https://github.com/typst/typst), a much more modern and fast alternative to LaTeX.
>
> I am sorry for all the people who put trust in this project, and I'd be happy to let anyone
> continue this project.

# UnTeX

[![Crates.io](https://img.shields.io/crates/v/untex)](https://crates.io/crates/untex)
[![docs.rs](https://img.shields.io/docsrs/untex)](https://docs.rs/untex/0.4.0-beta/untex/index.html)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/3bbc6f75856b4c4f85b6a7e509e0ccbf)](https://www.codacy.com/gh/jeertmans/untex/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=jeertmans/untex&amp;utm_campaign=Badge_Grade)
[![codecov](https://codecov.io/gh/jeertmans/untex/branch/main/graph/badge.svg?token=76STP1K1U1)](https://codecov.io/gh/jeertmans/untex)

UnTeX is both a library and an executable that allows you to manipulate and
understand TeX files.

> **NOTE**: even though TeX and LaTeX are not the same,
> UnTeX assumes that TeX documents are written such that
> they will be parsed with some LaTeX engine.
> For "*pure*" TeX content, see the `src/lib/tex` module.

## Executable

The most convenient way of using UnTeX is through its command-line interface (CLI).

Currently, you can install it with Cargo:

```bash
cargo install untex --all-features --version 0.4.0-beta
```

> **NOTE**: while using `--features cli` is sufficient to install UnTeX's CLI,
> using all features is recommend to take most benefits out of it!

### Usage

UnTeX has multiple commands, each one with a specific application:

* `check` for checking that a document will compile without error[*](#disclaimers);

* `dependendies`, or `deps`, for extracting dependencies from a TeX project;

* `expand` for expanding macros (e.g., `\input{...}` or `\include{...}`);

* `highlight`, or `hl`, for highlighting parts (e.g., comments) of TeX documents;

* `format` for pretty formatting your TeX files;

* `parse` for parsing and validating TeX documents[*](#disclaimers).

* `completions` to generate completions scripts for your shell
(needs `"cli-complete"` feature).

```bash
untex <COMMAND> [OPTIONS] [FILENAMES]...
```

A complete usage help can be obtained with `untex [-h|--help]` or with
`untex <COMMAND> [-h|--help]` for a given command.

### Examples

#### Highlighting text

```bash
untex hl -p math main.tex
echo "% this is a comment\nthis is not a comment" | untex hl -t comment
```

## Library

You can use UnTeX in your Rust project by adding to your `Cargo.toml`:

```toml
untex = "0.4.0-beta"
```

### Documentation

Automatically generated documentation can be found found [here](https://docs.rs/untex).

### Feature Flags

#### Default Features

* **color**: Adds support for output colorized text in the terminal with `termcolor`.

* **strum**: Uses `strum_macros`'s capabilities to enhance `Enum`s all across
the library.

#### Optional Features

* **cli**: Adds command-line related methods for multiple structures.
This feature is required to install UnTeX's CLI.

* **annotate**: (Soon) Adds method(s) to annotate results from check request.
If **cli** feature is also enabled, the CLI will by default print an annotated
output.

* **cli-complete**: Adds commands to generate completion files for various
shells. This feature also activates the **cli** feature.
Enter `untex completions --help` for get help with installing completion files.

* **json**: (Soon) Adds the `json` output option.

### Disclaimers

As this project is under active development, expect non backward compatible
changes from version to version.
Before reaching **v1.x.x**, UnTeX will be considered as unstable.

### What is a valid (La)TeX document

Parsing La(TeX) documents is very complicated, and the main reasons are
detailed in the header of the documentation. Because of this, UnTex does
not aim to be an exact parser, but a relatively good parser.

If you are in a situation where you think UnTeX produces a wrong result,
please reach out to me, preferably via a
[GitHub issue](https://github.com/jeertmans/untex/issues),
and explain to me what you expected!

## Contributing

Contributions are more than welcome!
Making UnTeX a good and reliable TeX tool is a matter of time and work,
so any kind of help is a step towards a better UnTeX!
