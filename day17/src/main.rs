#![allow(unused)]

use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;

// Utility functions

// echo: repeats input
fn echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

// cat: concatenates files
fn cat(file_paths: Vec<String>) {
    for path in file_paths {
        if let Ok(contents) = fs::read_to_string(&path) {
            println!("{}", contents);
        } else {
            eprintln!("Error reading file: {}", path);
        }
    }
}

// ls: lists directories
fn ls(directory_path: &str) {
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    } else {
        eprintln!("Error reading directory: {}", directory_path);
    }
}

// find: locates files or directories
fn find(directory_path: &str, target_name: &str) {
    let walker = fs::read_dir(directory_path).unwrap();
    for entry in walker {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.file_name().map_or(false, |name| name == target_name) {
                println!("{}", path.display());
            }
        }
    }
}

// grep: matches text in files
fn grep(file_path: &str, pattern: &str) {
    if let Ok(file) = fs::File::open(file_path) {
        let reader = io::BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if line.contains(pattern) {
                    println!("{}: {}", index + 1, line);
                }
            }
        }
    } else {
        eprintln!("Error reading file: {}", file_path);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("echo") => echo(args[2..].to_vec()),
        Some("cat") => cat(args[2..].to_vec()),
        Some("ls") => ls(args.get(2).map(|s| s.as_str()).unwrap_or(".")),
        Some("find") => find(
            args.get(2).map(|s| s.as_str()).unwrap_or("."),
            args.get(3).map(|s| s.as_str()).unwrap_or(""),
        ),
        Some("grep") => grep(
            args.get(2).map(|s| s.as_str()).unwrap_or(""),
            args.get(3).map(|s| s.as_str()).unwrap_or(""),
        ),
        _ => eprintln!("Unknown command."),
    }
}
