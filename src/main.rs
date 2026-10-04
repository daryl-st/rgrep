#![allow(unused)]
use std::{env::args, fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};

use crate::RgrepError::{CommandError, DoNotExist, FailedToRead, UnknownFlag};

#[derive(Debug)]
enum RgrepError {
    DoNotExist,
    CommandError,
    UnknownFlag,
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
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                return  Err(FailedToRead(err));
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    return Err(FailedToRead(err));
                }
            };

            if rgrep.case_sensetivity {
                let (line_case, pattern_case) = (line.to_lowercase(), rgrep.pattern.to_lowercase());
                    
                if line_case.contains(&pattern_case) {
                    println!("{}", line);
                    continue;
                }
            } else {
                if line.contains(&rgrep.pattern) { // extract the print function for better styling 
                    println!("{}", line);
                }
            }            
        }
    }
    println!("");

    Ok(())
}

fn parser(args: Vec<String>, rgrep: &mut Rgrep) -> Result<(), RgrepError> {
    let len = args.len();

    if len > 4 || len < 2 { 
        return Err(CommandError);
    }

    if args[1].starts_with("-") { // we are in the flags teritory
        let flag = args[1].as_str();

        if flag == "-h" || flag == "--help" {
            helper();
            std::process::exit(0);
        } else if flag == "-v" || flag == "--version" {
            println!("rgrep 0.1");
            std::process::exit(0);
        } else if flag == "-i" && len == 3 { // default path
            rgrep.case_sensetivity = true;
            rgrep.pattern = args[2].clone();
            rgrep.path = PathBuf::from(".");
        } else if flag == "-i" && len == 4 {
            rgrep.case_sensetivity = true;
            rgrep.pattern = args[2].clone();
            rgrep.path = PathBuf::from(args[3].clone());
        } else {
            return Err(UnknownFlag);
        }
    } else if len == 2 { // default path
        rgrep.path = PathBuf::from(".");
        rgrep.pattern = args[1].clone();
    } else if len == 3 { // no flag supplied
        rgrep.pattern = args[1].clone();
        rgrep.path = PathBuf::from(args[2].clone());
    } else {
        return Err(CommandError)
    }

    let metadata = match fs::metadata(rgrep.path.clone()) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(FailedToRead(err));
        }
    };

    if metadata.is_dir() {
        if let Err(err) = traversal(rgrep.path.to_path_buf(), &mut rgrep.files) {
            return Err(err);
        }
    } else if metadata.is_file() {
        rgrep.files.push(rgrep.path.to_path_buf()); // no traversal needed
    } else { // temporary
        eprintln!("error: failed to read the type of file");
        std::process::exit(1);        
    }

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
        RgrepError::UnknownFlag => {
            eprintln!("error: unrecognized option: try rgrep --help");
            std::process::exit(1);
        }
    }
}

fn helper() {
    println!("rgrep - a better rust based grep");
    println!("\nUSAGE:");
    println!("  rgrep [OPTIONS] [PATTERN] <PATH>");
    println!("\nUseful Commands:");
    println!("  -h, --help      display help");
    println!("  -v, --version   show version");
    println!("  -i,             case insensitive");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_work() {
        let args: Vec<String> = vec![String::from("-i"), String::from("files")];

        let mut rgrep = Rgrep { 
            pattern: String::new(), 
            path: PathBuf::new(), 
            files: vec![], 
            case_sensetivity: false, 
        };

        if let Err(err) = parser(args, &mut rgrep) {
            handle_error(err);
        }
        
        assert_eq!(rgrep.pattern, "files");
        assert!(!rgrep.files.is_empty());
    }

    #[test]
    fn traversal_works() {
        let mut rgrep = Rgrep { 
            pattern: String::new(), 
            path: PathBuf::new(), 
            files: vec![], 
            case_sensetivity: false, 
        };

        rgrep.path = PathBuf::from(".");

        let a = traversal(rgrep.path, &mut rgrep.files);

        assert!(!rgrep.files.is_empty());
        assert!(a.is_ok());

    }
}

// improve functionality
// - print matching lines with context you choose - 
// - print using colors, line numbers, & file names - 
// - improve tests for at least normal match, no match and missing file - 
// - add regex features - 
// - add features that enables it for pipe functions - 

// - it should only print the values if everything goes okay other wise not print at all
// - separating the printing function into module might help when decorating
