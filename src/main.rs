extern crate gecko_parser;
use gecko_parser::ast::parse_gecko;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc < 2 {
        println!("Please specify source file path.");
        return;
    }

    let path = &args[1];
    if !path.ends_with(".gk") {
        println!("File specified was not a Gecko (.gk) source file.");
        return;
    }

    let source = fs::read_to_string(path)
        .expect("Unable to read source file.");
    let ast = parse_gecko(&source);
    match ast {
        Ok(_) => {
            println!("Successfully constructed AST.");
            ast.unwrap().display(0);
        },
        Err(e) => println!("Parsing Unsuccessful: \n {:?}", e)
    }
}