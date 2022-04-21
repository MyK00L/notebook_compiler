use std::path::Path;

fn to_title_case(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    let mut last_space = true;
    for c in s.chars() {
        if c == ' ' || c == '-' || c == '_' {
            if !last_space {
                r.push(' ');
            }
            last_space = true;
        } else if last_space {
            r.extend(c.to_uppercase().collect::<String>().chars());
            last_space = false;
        } else {
            r.push(c);
            last_space = false;
        }
    }
    r
}

enum LineType {
    Section(String),
    SubSection(String),
    File(String),
}

const ALLOWED_EXTENSIONS: [&str; 8] = ["cpp", "rs", "c", "h", "hpp", "py", "java", "txt"];

fn dfs(dir: &Path, depth: usize) -> Vec<LineType> {
    if depth > 3 {
        return vec![];
    }
    if let Some(x) = dir.file_name() {
        if let Some(y) = x.to_str() {
            if let Some('.') = y.chars().next() {
                return vec![];
            }
        }
    }
    let mut res = Vec::<LineType>::new();
    if dir.is_dir() {
        let mut subs = vec![];
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            subs.extend(dfs(&path, depth + 1));
        }
        if !subs.is_empty() {
            if let Some(filename) = dir.file_stem() {
                let name = to_title_case(filename.to_string_lossy().as_ref());
                if depth == 1 {
                    res.push(LineType::Section(name));
                } else if depth > 1 {
                    res.push(LineType::SubSection(name));
                }
            }
            res.extend(subs);
        }
    } else if dir.is_file() {
        if let Some(ext) = dir.extension() {
            if let Some(ext) = ext.to_str() {
                if ALLOWED_EXTENSIONS.contains(&ext) {
                    if let Some(filename) = dir.file_stem() {
                        let name = to_title_case(filename.to_string_lossy().as_ref());
                        if depth == 1 {
                            res.push(LineType::Section(name));
                        } else if depth == 2 {
                            res.push(LineType::SubSection(name));
                        }
                    }
                    if let Some(filename) = dir.to_str() {
                        res.push(LineType::File(filename.to_string()));
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} directory", args[0]);
        return;
    }
    if !std::path::Path::exists(Path::new(&args[1])) {
        eprintln!("directory {} does not exist", args[1]);
        return;
    }
    let res = dfs(&Path::new(&args[1]), 0);
    for i in res {
        match i {
            LineType::Section(x) => println!("{}", x),
            LineType::SubSection(x) => println!("\t{}", x),
            LineType::File(x) => println!("\t\t{}", x),
        };
    }
}
