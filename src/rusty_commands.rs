use std::fs;

pub fn init() {
    println!("Initializing a new git-rusty repository...");
    let mut init_ok = true;
    let git_rusty = fs::create_dir(".git-rusty");
    let objects = fs::create_dir(".git-rusty/objects");
    let refs = fs::create_dir(".git-rusty/refs");
    let refs_heads = fs::create_dir(".git-rusty/refs/heads");
    let refs_head_main = fs::write(".git-rusty/refs/heads/main", "0000000000000000000000000000000000000000");
    let head = fs::write(".git-rusty/HEAD", "ref: refs/heads/main\n");
    let index = fs::write(".git-rusty/index", "");
    match git_rusty {
        Ok(_) => println!("Created .git-rusty"),
        Err(e) => {
            println!("Error creating .git-rusty: {}", e);
            init_ok = false;
        },
    }
    match objects {
        Ok(_) => println!("Created .git-rusty/objects"),
        Err(e) => {
            println!("Error creating .git-rusty/objects: {}", e);
            init_ok = false;
        },
    }
    match refs {
        Ok(_) => println!("Created .git-rusty/refs"),
        Err(e) => {
            println!("Error creating .git-rusty/refs: {}", e);
            init_ok = false;
        },
    }
    match refs_heads {
        Ok(_) => println!("Created .git-rusty/refs/heads"),
        Err(e) => {
            println!("Error creating .git-rusty/refs/heads: {}", e);
            init_ok = false;
        },
    }
    match refs_head_main {
        Ok(_) => println!("Created .git-rusty/refs/heads/main"),
        Err(e) => {
            println!("Error creating .git-rusty/refs/heads/main: {}", e);
            init_ok = false;
        },
    }
    match head {
        Ok(_) => println!("Created .git-rusty/HEAD"),
        Err(e) => {
            println!("Error creating .git-rusty/HEAD: {}", e);
            init_ok = false;
        },
    }
    match index {
        Ok(_) => println!("Created .git-rusty/index"),
        Err(e) => {
            println!("Error creating .git-rusty/index: {}", e);
            init_ok = false;
        },
    }
    if !init_ok {
        println!("An error ocurred while initializing the git-rusty repository...");
        return;
    }
    println!("Initialized git-rusty repository successfully...");
}

pub fn help() {
    println!("Available commands:\n");
    println!("init - Initialize a new git-rusty repository");
    println!("cat-file - Display the contents of a file");
    println!("hash-object - Create a new object");
    println!("ls-tree - List the contents of a tree object");
    println!("help - Display this help message");
    println!("exit - Exit the program\n");
}

pub fn exit(running: &mut bool) {
    println!("Exiting...");
    *running = false;
}