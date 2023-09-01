use std::fs;
use std::io::Write;
use std::process::Command;

fn tokenize(){
    
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

    generate_assembly();
    assemble();
    run_executable();

    println!("Program succeeded.");

}
