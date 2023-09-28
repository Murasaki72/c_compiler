use std::fs;
use std::io::Write;
use std::process::Command;
enum TokenKind {
    NUMBER,
    SYMBOL,
    IDENTIFIER,
    NONE
}
pub struct Token {
    content: String,
    kind: TokenKind,
}

fn load_source(string_source: &mut String){
    let content = fs::read_to_string("input/input.c").expect("Cannot read a file");
    string_source.clear();
    string_source.push_str(&content);
}

// In this function, the assembly file will be generated.
fn generate_assembly(){
    
    let mut assembly_file = fs::File::create("output/assembly.s").expect("Cannot create a file.");

    let mut content = String::from(".intel_syntax noprefix\n.global main\nmain:\n");

    content += "\tmov rax, 21\n";
    content += "\tret\n";

    assembly_file.write(content.as_bytes()).expect("Cannot write.");

}

fn main(){

    let mut string_source = String::from("");
    load_source(&mut string_source);

    generate_assembly();

    println!("Program succeeded.");

}
