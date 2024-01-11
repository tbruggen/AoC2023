use std::env;
use std::fs;


fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);

   //let lines = add_padding(lines); 
    let width = lines[0].len();
    let height = lines.len();
    
    let matrix = convert_to_matrix(lines);

    let mut symbol_coords = Vec::new();
    for (row_ix, row) in matrix.iter().enumerate(){
        for (col_ix, c) in row.iter().enumerate(){
            if !(c.is_numeric() || *c == '.') {
                println!("{} is a symbol on coord ({},{})", c, col_ix, row_ix);
                symbol_coords.push((col_ix, row_ix));                
            }
            else{
                println!("{} is not a symbol", c);
            }
        }
    }
    // todo: iterate through symbols and look around the symbol for numbers
    for (x,y) in symbol_coords{
        iterate_surrounding_elements(&matrix, x,y);
    }
}

fn convert_to_matrix(lines: Vec<String>) -> Vec<Vec<char>>
{
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in lines{
        let mut row: Vec<char> =  Vec::new();
        for c in line.chars(){
            row.push(c);
        }
        chars.push(row);
    }

    chars
}

fn iterate_surrounding_elements(matrix: &Vec<Vec<char>>, col: usize, row: usize) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Iterate through surrounding elements
    for i in (col.saturating_sub(1))..(col + 2).min(cols) {
        for j in (row.saturating_sub(1))..(row + 2).min(rows) {
            // Skip the current element
            if i == col && j == row {
                continue;
            }

            // Access the surrounding element at position (i, j)
            let element = matrix[j][i];
            let symbol = matrix[row][col];
            // Perform operations with the surrounding element
            if element.is_numeric(){
                println!("Element {} at ({}, {}) borders to the symbol {} at ({}, {})", element, i, j, symbol, row, col);              

                // search left and right of element for other numerics
                let nr = find_consecutive_numerics(&matrix[j], i);

                println!("The whole number is {}", nr);

            }
            
        }
    }
}

fn find_consecutive_numerics(chars: &Vec<char>, x: usize) -> String {
    let mut result = String::new();

    // Iterate to the left
    let mut left = x;
    while left > 0 && chars[left - 1].is_numeric() {
        left -= 1;
    }

    // Iterate to the right
    let mut right = x;
    while right < chars.len() - 1 && chars[right + 1].is_numeric() {
        right += 1;
    }

    // Construct the result string
    for i in left..=right {
        result.push(chars[i]);
    }

    result
}

fn add_padding(lines: Vec<String>) -> Vec<String>
{
    let mut padded: Vec<String> = Vec::new();
    let width = lines[0].len();
    let height = lines.len();
   
    let binding = std::iter::repeat('.').take(width+2).collect::<String>();
    let padding: &str = binding.as_str();
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
