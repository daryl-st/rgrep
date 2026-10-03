#![allow(unused)]
use std::{env::args, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() > 3 || args.len() < 2 {
        eprintln!("error: try 'rgrep [PATTERN] <PATH>");
    }

    let (pattern, files) = parser(args).unwrap();

    search(pattern, files);
}

fn search(pattern: String, files: Vec<PathBuf>) {
    println!("");
    for file_path in files {
        let file_content = fs::read_to_string(file_path).unwrap();
        let lines: Vec<&str> = file_content.lines().collect();

        for line in lines {
            if line.contains(&pattern) { // we might need coloring
                println!("{}", line);
            }
        }   
    }
    println!("");
}

fn traversal(path: PathBuf, files: &mut Vec<PathBuf>) {
    let read_dir = match fs::read_dir(path) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            eprintln!("error: path does not exist");
            return;
        }
    };

    for dir in read_dir {
        let path = match dir {
            Ok(dir) => dir.path(),
            Err(err) => {
                eprintln!("error: something failed {}", err);
                return;
            }
        };

        if path.is_dir() {
            // quick fix for now
            if path.starts_with("./.git") || path.starts_with("./target") { // i need a better way of excluding files
                continue;
            }

            traversal(path, files);
        } else if path.is_file() {
            files.push(path);
        } else {
            eprintln!("error: path can not be identified");
            continue;
        }
    }
}

fn parser(args: Vec<String>) -> Result<(String, Vec<PathBuf>), std::io::Error> {
    let pattern = &args[1];
    let path = &args[2];
    let mut files = vec![];

    // directory travesral 
    traversal(PathBuf::from(path), &mut files);

    dbg!(&files);

    Ok((pattern.to_owned(), files))
}


// what i need to build r-grep
// somewhat similar to rg and better than find and something that use regex/globbing

// main finctionalities
// - search a file for a pattern
// - support case sensitivity option
// - print matching lines with context you choose
// - useful CLI help and errors
// - tests for at least normal match, no match and missing file
// - only use idiomatic rust
