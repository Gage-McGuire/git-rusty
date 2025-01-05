mod rusty_commands;
mod rusty_plumbing;
mod rusty_helper;

fn main() {
    let mut running = true;
    while running {
        print!("Enter a command: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input;
        match command.trim() {
            "init" => rusty_commands::init(),
            "cat-file" => rusty_plumbing::cat_file(),
            "hash-object" => rusty_helper::init_hash_object(),
            "ls-tree" => rusty_plumbing::ls_tree(),
            "write-tree" => rusty_plumbing::write_tree("testing"),
            "help" => rusty_commands::help(),
            "exit" => rusty_commands::exit(&mut running),
            _ => println!("Unknown command: {}Enter help to see available commands...", command),
        }
    }
}
