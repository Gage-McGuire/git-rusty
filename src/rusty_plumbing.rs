use flate2::read::ZlibDecoder;
use std::{io::prelude::*, path::Path};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

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

pub fn hash_object() {
    // get file from user
    print!("Enter file name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Create a new path where the file is located and read the file content
    // Then create a blob object with the file content
    let path = Path::new(input.trim());
    let file_content = std::fs::read_to_string(Path::new(path)).unwrap();
    let length = file_content.len();
    let blob_content = format!("blob {}\0{}", length, file_content);

    // Create a new hasher and hash the blob content
    // print the hash value of the blob content
    let mut hasher = Sha1::new();
    hasher.update(&blob_content);
    let sha = format!("{:x}", hasher.finalize());
    println!("{}", sha);

    // Create a new zlib encoder and compress the blob content
    // Create a new directory for the object 
    // and write the compressed content to the object file
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(blob_content.as_bytes()).unwrap();
    let compressed = e.finish().unwrap();
    let obj_dir = &sha[0..2];
    let obj_file = &sha[2..];
    std::fs::create_dir_all(format!(".git-rusty/objects/{}", obj_dir)).unwrap();
    std::fs::write(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file), compressed).unwrap();
}