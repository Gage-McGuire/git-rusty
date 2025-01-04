mod rusty_commands;
mod rusty_plumbing;

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
            "hash-object" => rusty_plumbing::hash_object(),
            "exit" => rusty_commands::exit(&mut running),
            _ => println!("Unknown command: {}", command),
        }
    }
}
