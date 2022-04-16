use crate::chars::CharStream;
use crate::token::{TokenKind, TokenStream};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref RE_INPUT: Regex = Regex::new(r"\\input\{(.*)\}").unwrap();
    static ref RE_IMAGE: Regex = Regex::new(r"\\includegraphics(?:\[.*\])\{([^\}]*)\}").unwrap();
    static ref RE_BIBLI: Regex = Regex::new(r"\\bibliography\{([^\}]*)\}").unwrap();
    static ref RE_TABLE: Regex = Regex::new(r"\{([^\}]*)\}").unwrap();
}

trait PathUtils {
    fn with_default_extension(self, ext: &str) -> PathBuf;
    fn with_main_dir(&self, main_dir: &PathBuf) -> PathBuf;
}

impl PathUtils for PathBuf {
    fn with_default_extension(mut self, ext: &str) -> PathBuf {
        if self.extension().is_none() {
            self.set_extension(ext);
        }
        self
    }
    fn with_main_dir(&self, main_dir: &PathBuf) -> PathBuf {
        if self.is_relative() && !self.starts_with(main_dir) {
            main_dir.join(self)
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DependencyKind {
    TeX,
    Image,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
struct Dependency<'source> {
    filename: PathBuf,
    main_dir: &'source Path,
    dependencies: Vec<Self>,
    kind: DependencyKind,
}

impl<'source> Dependency<'source> {
    pub fn new(filename: PathBuf, main_dir: &'source PathBuf) -> Self {
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

fn write_deps(dep: &Dependency, depth: usize) {
    for sub_dep in dep.dependencies.iter() {
        print!("{:<1$}|->", "", depth);
        println!("{:?}", sub_dep.filename);
        write_deps(sub_dep, depth + 2);
    }
}

pub fn file_deps(filename: &str) {
    let filename = PathBuf::from(filename);
    let main_dir = filename.parent().unwrap().into();
    let main_dep = Dependency::new(filename.clone(), &main_dir);

    if main_dep.dependencies.len() == 0 {
        println!("Depedencies of {}: none.", filename.to_string_lossy());
    } else {
        println!("Depedencies of {}:", filename.to_string_lossy());
        write_deps(&main_dep, 0);
    }
}
