use std::fs;
use std::io::Write;
use std::process::Command;

enum TokenKind {
    NUMBER,
    OPERATOR,
}
pub struct Token {
    content: String,
    kind: TokenKind,
}

fn load_source(string_source: &mut String){
    let content = fs::read_to_string("input/input.c").expect("Cannot read a file");
    string_source.push_str(&content);
}

fn tokenize(string_source: &String, tokens:&mut Vec<Token>){
    
    let mut is_in_number = false;
    let mut tmp_string = String::from("");

    // Brute force the given string.
    for c in string_source.chars() {
        if c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9' {
            if !is_in_number {
                is_in_number = true;
            }
            tmp_string.push(c);
        } else if is_in_number {
            is_in_number = false;
            tokens.push(Token{content: tmp_string, kind: TokenKind::NUMBER});
        }
    }

}

// In this function, the assembly file will be generated.
fn generate_assembly(){
    
    let mut assembly_file = fs::File::create("output/assembly.s").expect("Cannot create a file.");

    let mut content = String::from(".intel_syntax noprefix\n.global main\nmain:\n");

    content += "\tmov rax, 42\n";
    content += "\tret\n";

    assembly_file.write(content.as_bytes()).expect("Cannot write.");

}

//In this function, the assembly file will be converted to a executable file format.
fn assemble(){
    let _program = Command::new("wsl").args(&["cc", "-o", "output/result", "output/assembly.s"]).output().expect("Failed to assemble.");
}

// In this function, the executable file will be executed.
fn run_executable(){
    let program = Command::new("wsl").arg("./output/result").output().expect("Failed to assemble.");
    println!("{:?}", program);
}

fn main(){

    let mut string_source = String::from("");
    load_source(&mut string_source);

    let mut tokens: Vec<Token> = Vec::new();
    tokenize(&string_source, &mut tokens);

    //println!("{}", string_source);

    generate_assembly();
    assemble();
    run_executable();

    println!("Program succeeded.");

}
