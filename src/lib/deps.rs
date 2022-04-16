use crate::chars::CharStream;
use crate::token::{TokenKind, TokenStream};
use itertools::Itertools;
use lazy_static::lazy_static;
use ptree::{Style, TreeItem};
use regex::Regex;
use std::borrow::Cow;
use std::fmt;
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref RE_INPUT: Regex = Regex::new(r"\\input\{(.*)\}").unwrap();
    static ref RE_IMAGE: Regex = Regex::new(r"\\includegraphics(?:\[.*\])\{([^\}]*)\}").unwrap();
    static ref RE_BIBLI: Regex = Regex::new(r"\\bibliography\{([^\}]*)\}").unwrap();
    static ref RE_TABLE: Regex = Regex::new(r"\{([^\}]*\.txt)\}").unwrap();
    static ref RE_LISTI: Regex = Regex::new(r"\\lstinputlisting(?:\[.*\])\{([^\}]*)\}").unwrap();
    static ref RE_MINTD: Regex = Regex::new(r"\\inputminted(?:\{.*\})\{([^\}]*)\}").unwrap();
}

trait PathUtils {
    fn with_default_extension(self, ext: &str) -> PathBuf;
    fn with_main_dir(&self, main_dir: &Path) -> PathBuf;
}

impl PathUtils for PathBuf {
    fn with_default_extension(mut self, ext: &str) -> PathBuf {
        if self.extension().is_none() {
            self.set_extension(ext);
        }
        self
    }
    fn with_main_dir(&self, main_dir: &Path) -> PathBuf {
        if self.is_relative() && !self.starts_with(main_dir) {
            main_dir.join(self)
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum DependencyKind {
    TeX = 1,
    Image = 2,
    Other = 3,
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TeX => write!(f, "TeX"),
            Self::Image => write!(f, "Image"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dependency<'source> {
    filename: PathBuf,
    main_dir: &'source Path,
    dependencies: Vec<Self>,
    kind: DependencyKind,
}

impl<'source> Dependency<'source> {
    pub fn new(filename: PathBuf, main_dir: &'source Path) -> Self {
        let mut dependencies = Vec::<Dependency>::new();

        let kind = match filename
            .extension()
            .expect(&format!("filename `{:?}` has no extension", filename))
            .to_str()
            .unwrap()
        {
            "tex" => {
                let filepath = filename.with_main_dir(main_dir);
                let contents =
                    read_to_string(&filepath).expect(&format!("Could not read {:?}", filepath));

                let token_stream: TokenStream = CharStream::new(&contents).into();

                for token in token_stream {
                    if token.kind == TokenKind::Command {
                        if let Some(caps) = RE_INPUT.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("tex");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        } else if let Some(caps) = RE_IMAGE.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("pdf");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        } else if let Some(caps) = RE_BIBLI.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("bib");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        } else if let Some(caps) = RE_LISTI.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("txt");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        } else if let Some(caps) = RE_MINTD.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("txt");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        }
                    } else if token.kind == TokenKind::Text {
                        if let Some(caps) = RE_TABLE.captures(token.slice) {
                            let dep_filename =
                                PathBuf::from(&caps[1]).with_default_extension("txt");
                            dependencies.push(Dependency::new(dep_filename, main_dir));
                        }
                    }
                }
                DependencyKind::TeX
            }
            "jpeg" | "jpg" | "png" | "pdf" | "svg" => DependencyKind::Image,
            _ => DependencyKind::Other,
        };

        Self {
            filename,
            main_dir,
            dependencies,
            kind,
        }
    }
}

impl<'source> TreeItem for Dependency<'source> {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", style.paint(self.filename.to_string_lossy()))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.dependencies.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct GroupedDependency<'source> {
    filename: Option<PathBuf>,
    main_dir: Option<&'source Path>,
    dependencies: Vec<Self>,
    kind: DependencyKind,
    prefix: Option<String>,
}

impl<'source> From<Dependency<'source>> for GroupedDependency<'source> {
    fn from(dependency: Dependency<'source>) -> Self {
        Self {
            filename: Some(dependency.filename),
            main_dir: Some(dependency.main_dir),
            dependencies: dependency
                .dependencies
                .into_iter()
                .map_into::<Self>()
                .collect(),
            kind: dependency.kind,
            prefix: None,
        }
    }
}

impl<'source> TreeItem for GroupedDependency<'source> {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        match &self.prefix {
            None => write!(
                f,
                "{}",
                style.paint(self.filename.as_ref().unwrap().to_string_lossy())
            ),
            Some(s) => write!(
                f,
                "{}",
                Style {
                    bold: true,
                    ..style.clone()
                }
                .paint(&s)
            ),
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        match &self.prefix {
            Some(_) => Cow::from(self.dependencies.clone()),
            None => self
                .dependencies
                .clone()
                .into_iter()
                .sorted_by_key(|dep| dep.kind.clone())
                .group_by(|dep| dep.kind.clone())
                .into_iter()
                .map(|(kind, group)| Self {
                    filename: None,
                    main_dir: None,
                    dependencies: group.collect(),
                    kind: kind.clone(),
                    prefix: Some(kind.to_string()),
                })
                .collect_vec()
                .into(),
        }
    }
}

pub fn file_deps(filename: &str) {
    let filename = PathBuf::from(filename);
    let main_dir: PathBuf = filename.parent().unwrap().into();
    let main_dep = Dependency::new(filename, &main_dir);
    let main_dep: GroupedDependency = main_dep.into();

    ptree::print_tree(&main_dep).expect("Unable to print dependencies tree");
}
