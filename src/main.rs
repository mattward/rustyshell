use std::io;
use std::io::Write;

fn execute_line(line: &str) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        return;
    }
    let command = tokens[0];
    println!("Command: {}", command);

    if tokens.len() > 1 {
        let args: &[&str] = &tokens[1..];
        println!("Args: {}", args.join(", "));
    }
}

fn prompt() -> String {
    return String::from("rustyshell::matt> ");
}

fn main() {
    println!("rustyshell, 0.1 (18/06/2021)");

    loop {
        print!("{}", prompt());
        io::stdout().flush().expect("Unable to flush stdout");
        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line).expect("Unable to read from stdin");

        if command_line.trim().eq("exit") {
            break;
        }
        else {
            execute_line(&command_line);
        }
    }
}
