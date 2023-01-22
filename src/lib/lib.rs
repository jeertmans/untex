//#![deny(missing_docs)]
//#![deny(missing_debug_implementations)]
#![warn(clippy::must_use_candidate)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! UnTeX, a library for manipulating (La)TeX files.
//!
//! This crate, with its submodules, tries to provide a
//! comprehensive set of tools to work with TeX and LaTeX documents.
//!
//! > **NOTE**: until `v1.0.0`, UnTeX will not be considered stable, and
//! breaking changes are expected to occur. The command line version of UnTex,
//! installable with `cargo install UnTeX --features cli`, will be more stable.
//!
//! (La)TeX documents are inherently hard to parse, and this for multiple reasons:
//!
//! * in opposition to other programming languages, there exists no reserved keyword;
//! * the same character can have different meanings, i.e., category codes (see
//! [`CategoryCode`](crate::tex::category_codes::CategoryCode)), depending on the context;
//! * commands can be re-redefined at any time, and optional and required arguments have position
//! that is not fixed;
//! * multiple compilations are often required before an output document is as expected, due to
//! auxilary files generation.
//!
//! All of this makes working with such documents a challenge. UnTeX strives for a *mostly*
//! correct implementation of all the syntax rules, while keeping in mind that performances
//! are also important.
//!
//! If you find a bug using UnTeX, please create an [issue on
//! GitHub](https://github.com/jeertmans/untex/issues), so we can continue
//! on improving this tool.
pub mod latex;
pub mod prelude;
pub mod tex;
