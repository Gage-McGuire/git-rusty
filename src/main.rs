mod git_rusty;

fn main() {
    print!("Enter a command: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let command = input;
    match command.trim() {
        "init" => git_rusty::init(),
        "cat-file" => git_rusty::cat_file(),
        _ => println!("Unknown command: {}", command),
    }
}
