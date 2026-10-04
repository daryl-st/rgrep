#![allow(unused)]
use std::{env::args, fs, path::PathBuf};

use crate::RgrepError::{CommandError, DoNotExist, FailedToRead};

#[derive(Debug)]
enum RgrepError {
    DoNotExist,
    CommandError,
    FailedToRead(std::io::Error),
}

struct Rgrep {
    pattern: String,
    path: PathBuf,
    files: Vec<PathBuf>,
    case_sensetivity: bool,
}

fn main() {
    let args: Vec<String> = args().collect();

    let mut rgrep = Rgrep { 
        pattern: String::new(), 
        path: PathBuf::new(), 
        files: vec![], 
        case_sensetivity: false, 
    };

    if let Err(err) = parser(args, &mut rgrep) {
        handle_error(err);
    }

    if let Err(err) = search(&rgrep) {
        handle_error(err);
    }  
}

fn search(rgrep: &Rgrep) -> Result<(), RgrepError> {
    println!("");
    for file_path in &rgrep.files {
        let file_content = match fs::read_to_string(&file_path) {
            Ok(file_content) => file_content,
            Err(err) => {
                return Err(FailedToRead(err));
            },
        };

        let lines: Vec<&str> = file_content.lines().collect();

        for line in lines {
            if rgrep.case_sensetivity {
                let (line_case, pattern_case) = (line.to_lowercase(), rgrep.pattern.to_lowercase());
                
                if line_case.contains(&pattern_case) {
                    println!("{}", line);
                    continue;
                }
            }

            if line.contains(&rgrep.pattern) { // we might need coloring
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
            return Err(FailedToRead(err));
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

            if let Err(err) = traversal(path, files) {
                return Err(err);
            }
        } else if path.is_file() {
            files.push(path);
        } else {
            eprintln!("error: path can not be identified");
            continue;
        }
    }

    Ok(())
}

fn parser(args: Vec<String>, rgrep: &mut Rgrep) -> Result<(), RgrepError> {
    if args.len() > 4 || args.len() < 2 { 
        return Err(CommandError);
    }
    
    if args[1] == String::from("-h") || args[1] == String::from("--help") {
        helper();
        std::process::exit(0);
    } else if args[1] == String::from("-v") || args[1] == String::from("--version") {
        println!("rgrep 0.1");
        std::process::exit(0);
    } else if args[1] == String::from("-i") && args.len() == 4 {
        rgrep.case_sensetivity = true;
        rgrep.pattern = args[2].clone();
        rgrep.path = PathBuf::from(args[3].clone());
    } else if args.len() == 3 {
        rgrep.pattern = args[1].clone();
        rgrep.path = PathBuf::from(args[2].clone());
    } else {
        return Err(CommandError)
    }

    // if the path is file, no traversal is needed.
    let metadata = match fs::metadata(rgrep.path.clone()) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(FailedToRead(err));
        }
    };

    if metadata.is_dir() {
        // directory travesral 
        if let Err(err) = traversal(rgrep.path.to_path_buf(), &mut rgrep.files) {
            return Err(err);
        }
    } else if metadata.is_file() {
        rgrep.files.push(rgrep.path.to_path_buf());
    } else { // temporary
        eprintln!("error: failed to read the type of file");
        std::process::exit(1);        
    }

    Ok(())

    // Ok((pattern.to_owned(), files, case_insensetive))  
}

fn handle_error(err: RgrepError) {
    match err {
        RgrepError::DoNotExist => {
            eprintln!("error: path does not exist");
            std::process::exit(1);
        },
        RgrepError::CommandError => {
            eprintln!("error: try 'rgrep --help'");
            std::process::exit(1);
        },
        RgrepError::FailedToRead(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        },
    }
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
