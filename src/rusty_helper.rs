use std::path::Path;
use crate::rusty_plumbing;

pub fn read_file(path: &str) -> String {
    // Read the file content
    let path = Path::new(path.trim());
    let file_content = std::fs::read_to_string(Path::new(path)).unwrap();
    return file_content;
}

pub fn init_hash_object() {
    print!("Enter file name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let content = read_file(input.trim());
    let sha = rusty_plumbing::hash_object("blob",  content);
    println!("{}", sha)
}