use std::fs;

pub fn init() {
    println!("Initializing a new git-rusty repository...");
    let git_rusty = fs::create_dir(".git-rusty");
    let objects = fs::create_dir(".git-rusty/objects");
    let refs = fs::create_dir(".git-rusty/refs");
    let head = fs::write(".git-rusty/HEAD", "ref: refs/heads/main\n");
    match git_rusty {
        Ok(_) => println!("Created .git-rusty"),
        Err(e) => println!("Error creating .git-rusty: {}", e),
    }
    match objects {
        Ok(_) => println!("Created .git-rusty/objects"),
        Err(e) => println!("Error creating .git-rusty/objects: {}", e),
    }
    match refs {
        Ok(_) => println!("Created .git-rusty/refs"),
        Err(e) => println!("Error creating .git-rusty/refs: {}", e),
    }
    match head {
        Ok(_) => println!("Created .git-rusty/HEAD"),
        Err(e) => println!("Error creating .git-rusty/HEAD: {}", e),
    }

    println!("Initialized git-rusty repository...");
}

pub fn cat_file() {
    println!("cat-file command");
}