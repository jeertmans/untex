use crate::chars::CharStream;
use crate::token::{TokenKind, TokenStream};
use regex::Regex;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
enum DependencyKind {
    TeX,
    Image,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
struct Dependency {
    filename: String,
    main: String,
    dependencies: Vec<Self>,
    kind: DependencyKind,
}

fn relative_path(parent: &str, child: &str, ext: &str) -> String {
    let mut child = PathBuf::from(child);

    if child.extension().is_none() {
        child.set_extension(ext);
    }

    let child = Path::new(parent).parent().unwrap().join(child);

    child.to_str().unwrap().to_string()
}

impl Dependency {
    pub fn new(filename: &str, main: &str) -> Self {
        let mut dependencies = Vec::<Dependency>::new();

        if !filename.ends_with(".tex") {
            return Self {
                filename: filename.to_string(),
                main: main.to_string(),
                dependencies,
                kind: DependencyKind::Other,
            };
        }

        let contents = read_to_string(filename).expect(&format!("Could not read {}", filename));

        let token_stream: TokenStream = CharStream::new(&contents).into();

        let re_input = Regex::new(r"\\input\{(.*)\}").unwrap();
        let re_image = Regex::new(r"\\includegraphics(?:\[.*\])\{([^\}]*)\}").unwrap();

        for token in token_stream {
            if token.kind == TokenKind::Command {
                if let Some(caps) = re_input.captures(token.slice) {
                    let dep_filename = relative_path(main, caps.get(1).unwrap().as_str(), "tex");
                    dependencies.push(Dependency::new(&dep_filename, main));
                } else if let Some(caps) = re_image.captures(token.slice) {
                    let dep_filename = relative_path(main, caps.get(1).unwrap().as_str(), "pdf");
                    dependencies.push(Dependency::new(&dep_filename, main));
                }
            }
        }

        Self {
            filename: filename.to_string(),
            main: main.to_string(),
            dependencies,
            kind: DependencyKind::TeX,
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
    let main_dep = Dependency::new(filename, filename);

    write_deps(&main_dep, 0);
}
