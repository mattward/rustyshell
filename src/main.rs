use std::io;
use std::io::Write;
use std::process::Command;

const PROG_NAME: &str = "rustyshell";
const PROG_VER: &str = "0.0.1";

fn is_builtin(command: &str) -> bool {
    command.eq("history") || command.starts_with('!')
}

fn execute_command(command: &str, args: &[&str]) {
    let res = Command::new(command)
        .args(args)
        .spawn();

    if res.is_ok() {
        let mut child = res.unwrap();
        child.wait().expect("Command failed while shell waiting for it to finish");
    } else {
        eprintln!("Unable to start command: {}", command);
    }
}

fn execute_builtin(command: &str, _args: &[&str], history: &Vec<String>) {
    if command.eq("history") {
        list_history(&history);
    }
    else if command.starts_with("!") && command.len() > 1 {
        let res: Result<usize, _> = command.split_at(1).1.parse();
        if res.is_ok() {
            let hist_idx: usize = res.unwrap() - 1;
            if hist_idx >= history.len() {
                eprintln!("No history entry #{}", hist_idx);
            } else {
                print!("Execute '{}' [Y/n] ", history[hist_idx]);
                io::stdout().flush().expect("Unable to flush stdout");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Unable to read from stdin");
                if answer.trim().eq("") || answer.trim().eq("Y") || answer.trim().eq("y") {
                    execute_line(&history[hist_idx], &history);
                }
            }
        }
        else {
            eprintln!("Expected a history item number, but got {}", res.err().unwrap())
        }
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
