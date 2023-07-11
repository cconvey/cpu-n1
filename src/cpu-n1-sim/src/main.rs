use std::io;

fn main() {
    println!("Greetings, program.");

    let mut cmd_string = String::new();

    io::stdin().read_line(&mut cmd_string).expect("Failed to read line.");

    println!("Command entered: {cmd_string} {:#p}", &cmd_string);
}
