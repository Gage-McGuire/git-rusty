use std::io;

fn main() {
    let mut input = String::new();
    print!("Enter a command: ");
    let _ = io::Write::flush(&mut io::stdout());
    let _ = io::stdin().read_line(&mut input);
    println!("You entered: {}", input);
}
