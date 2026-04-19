use std::{env, fs, io, process};
use owo_colors::OwoColorize;
use iron::runtime::vm;
use iron::frontend::codegen::Compiler;


use iron::frontend::lexer::Lexer;

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

    let source = "int x = 5;";
    let mut lexer = Lexer::new(source);
    if let Err(e) = lexer.run() {
        eprintln!("{}", e);
        process::exit(1);
    }

    let tokens = lexer.tokens;
    println!("{:?}", tokens);

    // let compiler = Compiler::new(&source).unwrap_or_else(|e| {
    //     eprintln!("{}", e);
    //     process::exit(1);
    // });
    
    // let module = compiler.compile().unwrap_or_else(|e| {
    //     eprintln!("{}", e);
    //     process::exit(1);
    // });

    // let mut vm = vm::VM::new(&module);
    // let res = vm.run();
    // if res != 0 {
    //     process::exit(res);
    // }
    
    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
