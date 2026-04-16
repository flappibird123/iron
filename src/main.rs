use std::{env, fs, io, process};
use owo_colors::OwoColorize;
use iron::runtime::vm;
use iron::frontend::chunk::Chunk;

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

    let bytecode = vec![5, 0, 5, 1, 2, 0, 6];

    let constants = vec![5, 3];

    let module = Chunk::new(bytecode, constants);
    let mut vm = vm::VM::new(&module);
    let res = vm.run();
    if res != 0 {
        process::exit(res);
    }
    
    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
