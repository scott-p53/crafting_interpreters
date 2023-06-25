use std::env;
use std::fs;
use std::io;
use std::process;

mod helper;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
}

fn run_file(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    let has_errors = run(contents);

    if has_errors {
        process::exit(65)
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line from stdin");
        if bytes == 0 {
            break;
        }

        run(line);
    }
}

fn run(source: String) -> bool {
    let mut scanner = scanner::scanner::Scanner::new(&source);
    let errors = scanner.scan_tokens();

    helper::helper::report_errors(&errors);
    return errors.len() != 0;
}
