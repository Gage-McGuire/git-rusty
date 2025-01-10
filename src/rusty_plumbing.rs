use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::os::unix::fs::PermissionsExt;

pub fn cat_file() {
    // get file from user
    print!("Enter blob sha: ");
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

    let blob_info: Vec<&str> = contents.split("\0").collect();
    println!("{}", blob_info[0]);
    println!("{}", blob_info[1]);
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

    // split the contents for printing
    match contents.split_once("\0") {
        Some((tree_info, content)) => {
            println!("{}", tree_info);
            let lines: Vec<&str> = content.split("\n").collect();
            for line in lines {
                if line != "" {
                    let split_line: Vec<&str> = line.split(" ").collect();
                    let split_file: Vec<&str> = split_line[1].split("\0").collect();
                    let mode = split_line[0];
                    let file = split_file[0];
                    let sha = split_file[1];
                    println!("Mode: {} Name: {} Sha: {}", mode, file, sha);
                }
            }
        },
        None => {
            println!("Invalid tree object");
        }
    };
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

    // iterate over the files and check the mode
    // if the mode is a tree (040755), call write_tree recursively
    // if the mode is a blob, hash the object
    for file in files {
        if &file.1[..2] == "04" {
            let tree_sha = write_tree(&file.0);
            tree_content.push_str(&format!("{} {}\0{}\n", &file.1, &file.0, tree_sha));
        } else {
            let contents = std::fs::read_to_string(&file.0).unwrap();
            let blob_sha = hash_object("blob", &contents);
            tree_content.push_str(&format!("{} {}\0{}\n", &file.1, &file.0, blob_sha));
        }
    }

    // return final tree sha
    let tree_sha = hash_object("tree", &tree_content);
    return tree_sha;
}

pub fn commit_tree() {
    print!("Enter tree sha: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut tree_sha = String::new();
    std::io::stdin().read_line(&mut tree_sha).unwrap();

    print!("Enter parent commit sha (if any, otherwise press enter): ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut parent_sha = String::new();
    std::io::stdin().read_line(&mut parent_sha).unwrap();

    print!("Enter author: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();

    print!("Enter author email: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut author_email = String::new();
    std::io::stdin().read_line(&mut author_email).unwrap();

    print!("Enter committer: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut committer = String::new();
    std::io::stdin().read_line(&mut committer).unwrap();

    print!("Enter committer email: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut committer_email = String::new();
    std::io::stdin().read_line(&mut committer_email).unwrap();

    print!("Enter commit message: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut commit_message = String::new();
    std::io::stdin().read_line(&mut commit_message).unwrap();

    let mut commit_content = String::new();
    commit_content.push_str(&format!("tree {}\n", tree_sha.trim()));
    commit_content.push_str(&format!("parent {}\n", parent_sha.trim()));
    commit_content.push_str(&format!("author {} <{}>\n", author.trim(), author_email.trim()));
    commit_content.push_str(&format!("committer {} <{}>\n", committer.trim(), committer_email.trim()));
    commit_content.push_str(&format!("\n{}", commit_message.trim()));

    let commit_sha = hash_object("commit", &commit_content);
    let ref_location = std::fs::read_to_string(".git-rusty/HEAD").unwrap();
    let ref_location = ref_location.split(": ").collect::<Vec<&str>>()[1].trim();
    let branch = ref_location.split("/").collect::<Vec<&str>>()[2];
    std::fs::write(format!(".git-rusty/{}", ref_location), commit_sha).unwrap();

    println!("Changes commited to {} branch", branch);
}

pub fn checkout() {
    print!("Enter branch name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut branch_name = String::new();
    std::io::stdin().read_line(&mut branch_name).unwrap();

    let refs_file = format!(".git-rusty/refs/heads/{}", branch_name.trim());
    let refs_obj = std::fs::read_to_string(refs_file).unwrap();
    let obj_dir = refs_obj[0..2].trim().to_string();
    let obj_file = refs_obj.trim()[2..].to_string();
    let file = std::fs::read(format!(".git-rusty/objects/{}/{}", obj_dir, obj_file)).unwrap();
    
    let mut decoder = ZlibDecoder::new(&file[..]);
    let mut commit_info = String::new();
    decoder.read_to_string(&mut commit_info).unwrap();

    println!("{}", commit_info);

    let _ = std::fs::write(".git-rusty/HEAD", format!("ref: refs/heads/{}", branch_name.trim()));
}

pub fn clone() {
    print!("Enter commit sha: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut commit_sha = String::new();
    std::io::stdin().read_line(&mut commit_sha).unwrap();


}

pub fn branch() {
    print!("Enter branch name: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut branch = String::new();
    std::io::stdin().read_line(&mut branch).unwrap();

    let head = std::fs::read_to_string(".git-rusty/HEAD").unwrap();
    let head = head.split(": ").collect::<Vec<&str>>()[1].trim();
    let current_branch = format!(".git-rusty/{}", head.trim());
    let current_commit = std::fs::read_to_string(current_branch).unwrap();
    let refs_file = format!(".git-rusty/refs/heads/{}", branch.trim());
    std::fs::write(refs_file, current_commit).unwrap();
    println!("Branch {} created", branch.trim());
}