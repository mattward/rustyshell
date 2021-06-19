use std::io;
use std::io::Write;
use std::process::Command;

const PROG_NAME: &str = "rustyshell";
const PROG_VER: &str = "0.0.1";

fn is_builtin(command: &str) -> bool {
    return command.eq("!!");
}

fn execute_command(command: &str, args: &[&str]) {
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("Unable to start command");

    child.wait().expect("Command failed");
}

fn execute_line(line: &str) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        return;
    }
    let command = tokens[0];
    println!("Command: {}", command);

    let args: &[&str] = &tokens[1..];
    if tokens.len() > 1 {
        println!("Args: {}", args.join(", "));
    }

    if is_builtin(&command) {
        println!("Unimplemented, but would execute the built-in '{}'", &command);
    }
    else {
        execute_command(&command, args);
    }
}

fn make_prompt() -> String {
    return PROG_NAME.to_owned() + "::> ";
}

fn main() {
    println!("{}, v{}", PROG_NAME, PROG_VER);

    let prompt = make_prompt();

    loop {
        print!("{}", prompt);
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
