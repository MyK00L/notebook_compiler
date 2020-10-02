use std::path::Path;

enum LineType {
	Section(String),
	SubSection(String),
	File(String),
}

const ALLOWED_EXTENSIONS: [&str; 8] = ["cpp","rs","c","h","hpp","py","java","txt"];

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
		for entry in std::fs::read_dir(dir).unwrap() {
			let path = entry.unwrap().path();
			res.extend(dfs(&path,depth+1));
		}
		if !res.is_empty() {
			if let Some(filename) = dir.to_str() {
				if depth == 1 {
					res.push(LineType::Section(filename.to_string()));
				} else if depth>1 {
					res.push(LineType::SubSection(filename.to_string()));
				}
			}
		}
	} else if dir.is_file() {
		if let Some(ext) = dir.extension(){
			if let Some(ext) = ext.to_str() {
				if ALLOWED_EXTENSIONS.contains(&ext){
					if let Some(filename) = dir.to_str() {
						res.push(LineType::File(filename.to_string()));
					}
				}
			}
		}
	}
	res
}

fn main(){
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		eprintln!("Usage: {} directory", args[0]);
		return;
	}
	if !std::path::Path::exists(Path::new(&args[1])){
		eprintln!("directory {} does not exist", args[1]);
		return;
	}
	let mut res = dfs(&Path::new(&args[1]),0);
	res.reverse();
	for i in res {
		match i {
			LineType::Section(x) => println!("{}",x),
			LineType::SubSection(x) => println!("\t{}",x),
			LineType::File(x) => println!("\t\t{}",x),
		};
	}
}
