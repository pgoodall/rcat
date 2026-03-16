use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_contents(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path);
    match file {
        Ok(file) => BufReader::new(file),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", file_path)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", file_path)
                }
                _ => panic!("Error opening file: {}", e),
            }
        }
    }
}

fn print_file_contents(content: BufReader<File>) {
    for line in content.lines() {
        println!("{}", line.expect("Could not read line"));
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let file_contents = read_file_contents(&input_file);
    print_file_contents(file_contents);
//     for line in file_contents.lines() {
//         println!("{}", line.expect("Could not read line"));
//     }
}