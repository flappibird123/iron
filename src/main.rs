use std::{env, fs, io, process};
use owo_colors::OwoColorize;
use iron::frontend::{parser};

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

    let res = parser::Parser::new(&source);
    let mut parser;
    match res {
        Ok(p) => parser = p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let ast = parser.run();
    match ast {
        Ok(program) => {
            println!("{:?}", program);
        },
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }
    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
