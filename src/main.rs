use std::{fs::{self, DirEntry, ReadDir}};
use std::cmp;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("no path given");
    iterate_dir(fs::read_dir(path.as_str()).unwrap(), 0);
}

fn print_dir(entry: &DirEntry, depth: i32) {
    println!("{}{}{:?}", "  ".repeat(cmp::max(depth, 0) as usize), "+", entry.file_name());
}

fn print_fil(entry: &DirEntry, depth: i32) {
    println!("{}{}{:?}", "  ".repeat(cmp::max(depth, 0) as usize), "L", entry.file_name());
}

fn iterate_dir(path: ReadDir, depth: i32) {
    for items in path{
        if let Ok(entry) = items{
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    print_dir(&entry, depth);
                    iterate_dir(fs::read_dir(entry.path()).unwrap(), depth+1);
                }
                else if metadata.is_file() {
                    print_fil(&entry, depth);
                }
            }
        }
    }
}
