use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::os::unix::fs::PermissionsExt;

pub fn cat_file() {
    // get file from user
    print!("Enter file sha: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // format file location
    let obj_dir = input.trim()[0..2].to_string();
    let obj_file = input.trim()[2..].to_string();
    let file = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();

    // decode file
    let mut decoder = ZlibDecoder::new(&file[..]);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents).unwrap();

    // print file contents
    println!("{}", contents);
}

pub fn hash_object(object_type: &str, content: &String) -> String {
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
    // get tree sha from user
    print!("Enter tree sha: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // format tree location
    let obj_dir = input.trim()[0..2].to_string();
    let obj_file = input.trim()[2..].to_string();
    let file = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();

    // decode file
    let mut decoder = ZlibDecoder::new(&file[..]);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents).unwrap();

    // print file contents
    println!("{}", contents);
}

pub fn write_tree(dir: &str) -> String {
    // get all files in the directory
    let paths = std::fs::read_dir(dir).unwrap();

    // create a vector to store the files
    let mut files: Vec<(String, String)> = Vec::new();
    let mut tree_content = String::new();

    // iterate over the files and store the file path and mode
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let permissions = metadata.permissions();
        let mode = format!("{:0>6o}", permissions.mode());
        if !file_name.starts_with("."){
            files.push((path.display().to_string(), mode.trim().to_string()));
        }
    }

    // TODO: properly hash contents of files and directories
    for file in files {
        if &file.1[..2] == "04" {
            let tree_sha = write_tree(&file.0);
            tree_content.push_str(&format!("{} {}\0{}", &file.1, &file.0, tree_sha));
        } else {
            let contents = std::fs::read_to_string(&file.0).unwrap();
            let blob_sha = hash_object("blob", &contents);
            tree_content.push_str(&format!("{} {}\0{}", &file.1, &file.0, blob_sha));
        }
    }

    let tree_sha = hash_object("tree", &tree_content);
    return tree_sha;
}   