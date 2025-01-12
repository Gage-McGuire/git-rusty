use std::path::Path;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use crate::rusty_plumbing;

pub fn read_file(path: &str) -> String {
    // Read the file content
    let path = Path::new(path.trim());
    let file_content = std::fs::read_to_string(path).unwrap();
    return file_content;
}

pub fn hash_object() {
    print!("Enter file name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let content = read_file(input.trim());
    let sha = rusty_plumbing::hash_object("blob",  &content);
    println!("{}", sha)
}

pub fn write_tree() {
    print!("Enter directory: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let tree_sha = rusty_plumbing::write_tree(input.trim());
    println!("{}", tree_sha);
}

pub fn get_current_head() -> String {
    let head = std::fs::read_to_string(".git-rusty/HEAD").unwrap();
    let head = head.split(": ").collect::<Vec<&str>>()[1].trim();
    return head.to_string();
}

pub fn get_tree_vec(tree_sha: &str) -> Vec<(String, String, String)> {
    // format tree location
    let obj_dir = tree_sha.trim()[0..2].to_string();
    let obj_file = tree_sha.trim()[2..].to_string();
    let file = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();

    // decode file
    let mut decoder = ZlibDecoder::new(&file[..]);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents).unwrap();

    // split the contents for printing
    let mut tree_vec: Vec<(String, String, String)> = Vec::new();
    match contents.split_once("\0") {
        Some((_, content)) => {
            let lines: Vec<&str> = content.split("\n").collect();
            for line in lines {
                if line != "" {
                    let split_line: Vec<&str> = line.split(" ").collect();
                    let split_file: Vec<&str> = split_line[1].split("\0").collect();
                    let mode = split_line[0];
                    let file = split_file[0];
                    let sha = split_file[1];
                    tree_vec.push((mode.to_string(), file.to_string(), sha.to_string()));
                }
            }
        },
        None => {
            println!("Invalid tree object");
        }
    };
    return tree_vec;
}

pub fn write_working_dir(tree_sha: &str) {
    let tree_vec = get_tree_vec(tree_sha);
    for (mode, file, sha) in tree_vec {
        if &mode[..2] == "04" {
            write_working_dir(&sha);
        } else {
            let file_path = Path::new(&file);
            if !file_path.exists() {
                let obj_dir = sha[0..2].to_string();
                let obj_file = sha[2..].to_string();
                let file_vec = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();
                let mut decoder = ZlibDecoder::new(&file_vec[..]);
                let mut contents = String::new();
                decoder.read_to_string(&mut contents).unwrap();
                std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
                match contents.split_once("\0") {
                    Some((_, content)) => {
                        std::fs::write(file_path, content).unwrap();
                    },
                    None => {
                        println!("Invalid tree object");
                    }
                }
            }
        }
    }
}

pub fn remove_working_dir(tree_sha: &str) {
    let tree_vec = get_tree_vec(tree_sha);
    let working_dir = tree_vec[0].1.split("/").collect::<Vec<&str>>()[0];
    let dir = Path::new(working_dir);
    if dir.exists() {
        std::fs::remove_dir_all(dir).unwrap();
    }
}