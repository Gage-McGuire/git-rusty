use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::os::unix::fs::PermissionsExt;

pub fn cat_file() {
    // get file from user
    print!("Enter file name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // format file location
    let obj_dir = input.trim()[0..2].to_string();
    let obj_file = input.trim()[2..].to_string();
    let file = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();

    // decode file
    let mut d = ZlibDecoder::new(&file[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();

    // print file contents
    println!("{}", s);
}

pub fn hash_object(object_type: &str, content: String) -> String {
    // Create an object with the file content
    let length = content.len();
    let object_content = format!("{} {}\0{}", object_type, length, content);

    // Create a new hasher and hash the blob content
    // print the hash value of the blob content
    let mut hasher = Sha1::new();
    hasher.update(&object_content);
    let sha = format!("{:x}", hasher.finalize());

    // Create a new zlib encoder and compress the blob content
    // Create a new directory for the object 
    // and write the compressed content to the object file
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(object_content.as_bytes()).unwrap();
    let compressed = e.finish().unwrap();
    let obj_dir = &sha[0..2];
    let obj_file = &sha[2..];
    std::fs::create_dir_all(format!(".git-rusty/objects/{}", obj_dir)).unwrap();
    std::fs::write(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file), compressed).unwrap();

    // return the hash value of the blob content
    return sha;
}

pub fn ls_tree() {
    println!("Enter tree hash: ");
}

pub fn write_tree(dir: &str) {
    // get all files in the directory
    let paths = std::fs::read_dir(dir).unwrap();

    // create a vector to store the files
    let mut files: Vec<(String, String)> = Vec::new();
    let mut tree_content = Vec::new();

    // iterate over the files and store the file path and mode
    for path in paths {
        let path = path.unwrap().path();
        let metadata = std::fs::metadata(&path).unwrap();
        let permissions = metadata.permissions();
        let mode = format!("{:0>6o}", permissions.mode());
        files.push((path.display().to_string(), mode.trim().to_string()));
    }
    
    for file in files {
        if file.1 == "040755" {
            let hash = hash_object("tree", file.0.clone());
            tree_content.push(format!("040000 tree {} {}\n", hash, file.0));
        } else {
            let hash = hash_object("blob", file.0.clone());
            tree_content.push(format!("100644 blob {} {}\n", hash, file.0));
        }
    }

    println!("{:?}", tree_content);
}   