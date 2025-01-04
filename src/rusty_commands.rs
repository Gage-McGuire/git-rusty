use std::fs;

pub fn init() {
    println!("Initializing a new git-rusty repository...");
    let mut init_ok = true;
    let git_rusty = fs::create_dir(".git-rusty");
    let objects = fs::create_dir(".git-rusty/objects");
    let refs = fs::create_dir(".git-rusty/refs");
    let head = fs::write(".git-rusty/HEAD", "ref: refs/heads/main\n");
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
    match head {
        Ok(_) => println!("Created .git-rusty/HEAD"),
        Err(e) => {
            println!("Error creating .git-rusty/HEAD: {}", e);
            init_ok = false;
        },
    }
    if !init_ok {
        println!("An error ocurred while initializing the git-rusty repository...");
        return;
    }
    println!("Initialized git-rusty repository successfully...");
}

pub fn exit(running: &mut bool) {
    println!("Exiting...");
    *running = false;
}