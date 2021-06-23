use std::env::set_current_dir;

pub fn is_builtin(command: &str) -> bool {
    command.eq("history") || command.eq("cd") || command.starts_with('!')
}

pub fn execute_builtin(command: &str, args: &[&str], history: &Vec<String>) {
    match command {
        "history" => list_history(&history),
        "cd" => change_working_dir(&args),
        _ => println!("Unknown command!")
    }
}

fn list_history(history: &Vec<String>) {
    let mut i = 1;
    for line in history.iter() {
        println!("{}. {}", i, line);
        i += 1;
    }
}

fn change_working_dir(args: &[&str]) {
    if args.len() != 1 {
        eprintln!("cd requires exactly one argument: the directory required")
    } else {
        match set_current_dir(args[0]) {
            Ok(_) => (),
            Err(_) => eprintln!("Couldn't change directory!")
        }
    }
}
