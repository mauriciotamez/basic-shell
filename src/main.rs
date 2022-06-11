use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        // use the '>' char as the prompt
        print!("> ");
        // need to explicitly flush this to ensure it prints before read_line
        let mut stdout = io::stdout();
        stdout.flush();

        let mut input = String::new();
        let stdin = io::stdin();

        // read the user input
        stdin.read_line(&mut input).unwrap();

        // reading lines leaves a trailing newline, which trim method removes, everything after the first whitespace
        // character s interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e)
                }
            }
            "exit" => return,
            command => {
                let mut child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
