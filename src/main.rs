use std::io;
use std::io::Write;
use std::process::Command;

const PROG_NAME: &str = "rustyshell";
const PROG_VER: &str = "0.0.1";

fn is_builtin(command: &str) -> bool {
    command.eq("!!") || command.starts_with('*')
}

fn execute_command(command: &str, args: &[&str]) {
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("Unable to start command");

    child.wait().expect("Command failed");
}

fn execute_builtin(command: &str, args: &[&str], history: &Vec<String>) {
    if (command.eq("*.")) {
        list_history(&history);
    }
}

fn execute_line(line: &str, history: &Vec<String>) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        return;
    }

    let command = tokens[0];
    let args: &[&str] = &tokens[1..];

    if is_builtin(&command) {
        execute_builtin(&command, args, &history);
    } else {
        execute_command(&command, args);
    }
}

fn make_prompt() -> String {
    PROG_NAME.to_owned() + "::> "
}

fn list_history(history: &Vec<String>) {
    let mut i = 1;
    for line in history.iter() {
        println!("{}. {}", i, line);
        i += 1;
    }
}

fn main() {
    println!("** {}, v{} **", PROG_NAME, PROG_VER);

    let prompt = make_prompt();
    let mut history: Vec<String> = vec![];

    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Unable to flush stdout");
        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line).expect("Unable to read from stdin");

        let command_line = command_line.trim().to_string();

        if command_line.eq("exit") {
            break;
        } else {
            execute_line(&command_line, &history);
        }

        history.push(command_line);
    }
}
