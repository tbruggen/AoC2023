use std::env;
use std::fs;


fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);

    let lines = add_padding(lines);

    

    let mut symbol_coords = Vec::new();
    for (line_ix, line) in lines.iter().enumerate(){
        for (c_ix,c) in line.char_indices(){
            if !(c.is_numeric() || c == '.') {
                println!("{} is a symbol on coord ({},{})", c, line_ix, c_ix);
                symbol_coords.push((line_ix, c_ix));
                
            }
            else{
                println!("{} is not a symbol", c);
            }
        }
    }



    // todo: iterate through symbols and look around the symbol for numbers
    for (x,y) in symbol_coords{
       
        
    }


}

fn add_padding(lines: Vec<String>) -> Vec<String>
{
    let mut padded: Vec<String> = Vec::new();
    let width = lines[0].len();
    let height = lines.len();
   
    let binding = std::iter::repeat('.').take(width+2).collect::<String>();
    let padding: &str = binding.as_str();;
    padded.push(String::from(padding));

    for line in lines{
        let padded_line = format!(".{}.", line);
        padded.push(padded_line);
    }

    padded.push(String::from(padding));

    padded
}

fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}




#[test]
fn it_works() {
    let line: &str = "xxxxx";
    let mut lines: Vec<String> = Vec::new();
    lines.push(String::from(line));
    lines.push(String::from(line));

    let padded = add_padding(lines);

    assert_eq!(padded[0], String::from("......."));
    assert_eq!(padded[1], String::from(".xxxxx."));
    assert_eq!(padded[1], padded[2]);
    assert_eq!(padded[0], padded[3]);
}
