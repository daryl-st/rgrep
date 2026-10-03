#![allow(unused)]
use std::{env::args, fs, path::PathBuf};

use crate::RgrepError::{CommandError, DoNotExist, FailedToRead};

#[derive(Debug)]
enum RgrepError {
    DoNotExist,
    CommandError,
    FailedToRead(std::io::Error),
}

fn main() {
    let args: Vec<String> = args().collect();

    let (pattern, files, case_insensitive) = match parser(args) {
        Ok(result) => result,
        Err(err) => {
            handle_error(err);
            return;
        }
    };

    if let Err(err) = search(pattern, files, case_insensitive) {
        handle_error(err);
    }  
}

fn handle_error(err: RgrepError) {
    match err {
        RgrepError::DoNotExist => {
            eprintln!("error: path does not exist");
            std::process::exit(1);
        },
        RgrepError::CommandError => {
            eprintln!("error: try 'rgrep --help'");
            std::process::exit(127);
        },
        RgrepError::FailedToRead(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        },
    }
}

fn search(pattern: String, files: Vec<PathBuf>, case_insensitive: bool) -> Result<(), RgrepError> {
    println!("");
    for file_path in files {
        let file_content = match fs::read_to_string(&file_path) {
            Ok(file_content) => file_content,
            Err(err) => {
                return Err(FailedToRead(err));
            },
        };

        let lines: Vec<&str> = file_content.lines().collect();

        for line in lines {
            if case_insensitive {
                let (line_case, pattern_case) = (line.to_lowercase(), pattern.to_lowercase());
                
                if line_case.contains(&pattern_case) {
                    println!("{}", line);
                    continue;
                }
            }

            if line.contains(&pattern) { // we might need coloring
                println!("{}", line);
            }
        }   
    }
    println!("");

    Ok(())
}

fn traversal(path: PathBuf, files: &mut Vec<PathBuf>) -> Result<(), RgrepError> {
    let read_dir = match fs::read_dir(path) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            return Err(DoNotExist);
        }
    };

    for dir in read_dir {
        let path = match dir {
            Ok(dir) => dir.path(),
            Err(err) => {
                return Err(FailedToRead(err));
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

    Ok(())
}

fn parser(args: Vec<String>) -> Result<(String, Vec<PathBuf>, bool), RgrepError> {
    if args.len() > 4 || args.len() < 2 { 
        return Err(CommandError);
    }

    let mut case_insensetive = false;
    let mut files = vec![];
    let (pattern, path);

    if args[1] == String::from("-h") || args[1] == String::from("--help") {
        helper();
        std::process::exit(0);
    } else if args[1] == String::from("-v") || args[1] == String::from("--version") {
        println!("rgrep 0.1");
        std::process::exit(0);
    } else if args[1] == String::from("-i") && args.len() == 4 {
        case_insensetive = true;
        pattern = &args[2];
        path = &args[3];
    } else if args.len() == 3 {
        pattern = &args[1];
        path = &args[2];
    } else {
        return Err(CommandError)
    }

    // directory travesral 
    if let Err(err) = traversal(PathBuf::from(path), &mut files) {
        return Err(err);
    }

    Ok((pattern.to_owned(), files, case_insensetive))  
}

fn helper() {
    println!("rgrep - a better rust based grep");
    println!("\nUSAGE:");
    println!("  rgrep [OPTIONS] [PATTERN] <PATH>");
    println!("\nUseful Commands:");
    println!("  -h, --help      display help");
    println!("  -v, --version   show version");
}

// what i need to build r-grep
// somewhat similar to rg and better than find and something that use regex/globbing

// main finctionalities
// - search a file for a pattern - DONE
// - support case sensitivity option - DONE
// - print matching lines with context you choose - 
// - useful CLI help and errors - DONE
// - print using colors, line numbers, & file names - 
// - tests for at least normal match, no match and missing file - 
