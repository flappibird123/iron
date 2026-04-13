use std::{env, fs, io, process};
use owo_colors::OwoColorize;
use iron::frontend;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}{}", "error:".bright_red().bold(), " no input files".bold());
        process::exit(1);
    }
    let source = read_file(&args[1].as_str())?;
    if !source.is_ascii() {
        eprintln!("File contains non-ascii characters");
        process::exit(1);
    }

    let mut lexer = frontend::lexer::Lexer::new(&source);
    if let Err(e) = lexer.run() {
        eprintln!("{}", e);
        process::exit(1);
    }
    let tokens = lexer.tokens;
    println!("{:?}", tokens); 

    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
