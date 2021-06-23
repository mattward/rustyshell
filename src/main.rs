mod builtins;

use std::io;
use std::io::Write;
use std::process::Command;


const PROG_NAME: &str = "rustyshell";
const PROG_VER: &str = "0.0.1";


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

fn execute_line(line: &str, history: &Vec<String>) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        return;
    }

    let command = tokens[0];
    let args: &[&str] = &tokens[1..];

    if builtins::is_builtin(&command) {
        builtins::execute_builtin(&command, args, &history);
    } else {
        execute_command(&command, args);
    }
}

fn make_prompt() -> String {
    PROG_NAME.to_owned() + "::> "
}

fn next_command(prompt: &str, history: &Vec<String>) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Unable to flush stdout");
    let mut command_line = String::new();
    io::stdin().read_line(&mut command_line).expect("Unable to read from stdin");

    let mut command_line = command_line.trim().to_string();

    if command_line.starts_with("!") && command_line.len() > 1 {
        let res: Result<usize, _> = command_line.split_at(1).1.parse();
        if res.is_ok() {
            let hist_idx: usize = res.unwrap() - 1;
            if hist_idx >= history.len() {
                eprintln!("No history entry #{}", hist_idx + 1);
                command_line.truncate(0);
            } else {
                print!("Execute '{}' [Y/n] ", history[hist_idx]);
                io::stdout().flush().expect("Unable to flush stdout");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Unable to read from stdin");
                if answer.trim().eq("") || answer.trim().eq("Y") || answer.trim().eq("y") {
                    command_line.truncate(0);
                    command_line.push_str(&history[hist_idx]);
                }  else {
                    command_line.truncate(0);
                }
            }
        }
        else {
            eprintln!("Expected a history item number, but got {}", res.err().unwrap())
        }
    }

    command_line
}

fn main() {
    let prompt = make_prompt();
    let mut history: Vec<String> = vec![];

    println!("** {}, v{} **", PROG_NAME, PROG_VER);

    loop {
        let command_line = next_command(&prompt, &history);

        if command_line.eq("exit") {
            break;
        } else {
            execute_line(&command_line, &history);
            if !command_line.is_empty() {
                history.push(String::from(command_line));
            }
        }
    }
}
