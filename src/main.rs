#![allow(unused)]
use std::{env::args, fs};

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() > 3 || args.len() < 2 {
        eprintln!("error: try 'rgrep [PATTERN] <PATH>");
    }

    let (pattern, path) = parser(args).unwrap();

    search(pattern, path);
}

fn search(pattern: String, file_path: String) {
    let file_content = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = file_content.lines().collect();
    
    for line in lines {
        if line.contains(&pattern) { // we might need coloring
            println!("{}", line);
        }
    }
}

fn parser(args: Vec<String>) -> Result<(String, String), std::io::Error> {
    let pattern = &args[1];
    let path = &args[2];

    // the main check should be around directories first then recursively go into the files
    if !fs::exists(path).unwrap() {
        eprintln!("error: path does not exist");
    }

    Ok((pattern.to_owned(), path.to_owned()))
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
