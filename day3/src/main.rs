use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);
       
    let matrix = convert_to_matrix(lines);

    let mut symbol_coords = Vec::new();
    let mut gears = Vec::new();

    for (row_ix, row) in matrix.iter().enumerate(){
        for (col_ix, c) in row.iter().enumerate(){
            if !(c.is_numeric() || *c == '.') {
                //println!("{} is a symbol on coord ({},{})", c, row_ix, col_ix);
                symbol_coords.push((row_ix, col_ix));                
                if *c == '*'
                {
                    gears.push((row_ix, col_ix));
                }
            }
            
            else{
               // println!("{} is not a symbol", c);
            }
        }
    }
    // todo: iterate through symbols and look around the symbol for numbers

    let mut visited_coordinates: HashSet<(usize,usize)> = HashSet::new();
    let mut numbers: Vec<i32> = Vec::new();
   
    for (row,col) in symbol_coords{
        numbers.append(&mut iterate_surrounding_elements(&matrix, row, col, &mut visited_coordinates));
    }


    let mut visited_coordinates: HashSet<(usize,usize)> = HashSet::new();
    let mut sum_of_ratios: i32 = 0;
    for (row,col) in gears{
        let surrounding: Vec<i32> = iterate_surrounding_elements(&matrix, row, col, &mut visited_coordinates);
        if surrounding.len() == 2
        {
            let ratio: i32 = surrounding[0]*surrounding[1];
            sum_of_ratios += ratio;
        }
    }

    let sum: i32 = numbers.iter().sum();
    println!("The sum of all numbers bordering a symbol = {}", sum);
    println!("Ths sum of gear rations = {}", sum_of_ratios);
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

fn iterate_surrounding_elements(matrix: &Vec<Vec<char>>, row: usize, 
    col: usize, visited_coordinates: &mut HashSet<(usize,usize)>) -> Vec<i32>
{
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut numbers: Vec<i32> = Vec::new();
    // Iterate through surrounding elements
    for i in (row.saturating_sub(1))..(row + 2).min(rows) {
        for j in (col.saturating_sub(1))..(col + 2).min(cols) {
            // Skip the current element
          
            if (i == row && j == col) || visited_coordinates.contains(&(i,j)){
                continue;
            }

            // Access the surrounding element at position (i, j)
            let element = matrix[i][j];
           
            // Perform operations with the surrounding element
            if element.is_numeric(){
              //  println!("Element {} at ({}, {}) borders to the symbol {} at ({}, {})", element, i, j, symbol, row, col);              

                // search left and right of element for other numerics
                let nr = find_consecutive_numerics(&matrix, i, j, visited_coordinates);
                numbers.push(nr);
              //  println!("The whole number is {}", nr);

            }
            
        }
    }
    numbers
}

fn find_consecutive_numerics(matrix: &Vec<Vec<char>>, i: usize, j:usize, visited_coordinates: &mut HashSet<(usize, usize)>) -> i32 {
    let mut result = String::new();
    let chars = &matrix[i];

    // Iterate to the left
    let mut left = j;
    while left > 0 && chars[left - 1].is_numeric() {
        left -= 1;
    }

    // Iterate to the right
    let mut right = j;
    while right < chars.len() - 1 && chars[right + 1].is_numeric() {
        right += 1;
    }

    // Construct the result string
    for k in left..=right {
        visited_coordinates.insert((i,k));   
        result.push(chars[k]);
    }
    
    let int_num = result.parse::<i32>().expect("error while converting &str to u32");
    int_num
}


fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}


