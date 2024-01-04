use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);
    let mut sum: i32 = 0;
    for line in lines { 
         let mut two_digit: String = String::from("");       
         let first = first_digit(&line);
         let reversed: String =  line.chars().rev().collect();

         let last = first_digit(&reversed);
         add_to_string(&mut two_digit, first);   
         add_to_string(&mut two_digit, last);     
         println!("{}", two_digit);

         if let int_num = two_digit.parse::<i32>().expect("Failed to parse string as integer") {
            sum += int_num;
         }
         
         
    }
    
    println!("Answer = {}", sum);
}


fn add_to_string(s: &mut String, d1: Option<char>)
{
    if let Some(d) = d1 {
        s.push(d);
    }
}


fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}

fn first_digit(line: &str) -> Option<char>
{
    for c in line.chars(){
        match c.is_digit(10) {
            true => {
                return Some(c);
            }
            false => (),
        }
    }
    None
}



// Should return a vector of strings when given a valid file path.
#[test]
fn should_return_vector_of_strings_with_valid_file_path() {
    let filepath = "valid_file.txt";
    let result = open(filepath);
    assert_eq!(result, vec!["line 1", "line 2", "line 3"]);
}

// Should return Some(char) when given a string containing a digit as the first character
#[test]
fn should_return_some_char_with_digit_as_first_character() {
    let line = "1abc";
    let result = first_digit(line);
    assert_eq!(result, Some('1'));
}

// Should return None when given a string that does not contain any digits
#[test]
fn should_return_none_with_no_digits() {
    let line = "abc";
    let result = first_digit(line);
    assert_eq!(result, None);
}


// Should append a char to the end of a string when given a mutable string and Some(char)
#[test]
fn should_append_char_to_end_of_string_with_mutable_string_and_some_char() {
    let mut s = String::from("hello");
    let c = Some('!');
    add_to_string(&mut s, c);
    assert_eq!(s, "hello!");
}

#[test]
fn should_not_append_anything_to_string_with_mutable_string_and_none() {
    let mut s = String::from("hello");
    let c = None;
    add_to_string(&mut s, c);
    assert_eq!(s, "hello");
}