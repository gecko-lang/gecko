extern crate gecko_parser;
use gecko_parser::{
    ast::parse_gecko,
    node::Node
};
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
    let file = parse_gecko(&source);
    match file {
        Ok(_) => {
            println!("Successfully constructed AST.");

            let mut indent: String = String::from("");
            println!("{}", file.unwrap().display_tree(&mut indent, true));
        },
        Err(e) => println!("Parsing Unsuccessful: \n {:?}", e)
    }
}