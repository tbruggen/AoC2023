use std::env;
use std::fs;
use std::fmt;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);

    println!("number of lines: {}", lines.len());
    println!("length of a line: {}", lines[0].len());

    let mut symbolCoords = Vec::new();
    for (line_ix, line) in lines.iter().enumerate(){
        for (c_ix,c) in line.char_indices(){
            if !(c.is_numeric() || c == '.') {
                println!("{} is a symbol on coord ({},{})", c, line_ix, c_ix);
                symbolCoords.push((line_ix, c_ix));
                
            }
            else{
                println!("{} is not a symbol", c);
            }
        }
    }


    // todo: iterate through symbols and look around the symbol for numbers


}



fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}
