use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);
    let mut sum: i32 = 0;
    for line in lines { 
              
         let first = first_number(&line);         
         let last = last_number(&line);

         let two_digit:String = create_two_digit_string(first, last);            
         println!("{}", two_digit);
         
         let int_num = match two_digit.parse::<i32>(){
            Ok(i) => i,   
            Err(_) => 0,
         };

         sum += int_num;
    }
    
    println!("Answer = {}", sum);
}

fn first_number(line: &String) -> Option<char>
{
    let first_digit = first_digit(line);    
    let first_word: Option<(usize, &str)>  = first_spelled_number(line);
   
    // a spelled out word occurs first
    if let Some((digit_ix, c)) = first_digit{
        if let Some((word_ix, number)) = first_word{
            if digit_ix >= word_ix{
                return written_word_to_char(number);
            }              
        }    
        return Some(c);
    }

    else if let Some((_, number)) = first_word{
        return written_word_to_char(number);
    }
    None
}


fn last_number(line: &String) -> Option<char>
{    
    let last_digit = last_digit(line);
    let last_word: Option<(usize, &str)> = last_spelled_number(line);
    
    // a spelled out word occurs last
    if let Some((digit_ix, c)) = last_digit{
        if let Some((word_ix, number)) = last_word{
            if digit_ix < word_ix{
                return written_word_to_char(number);
            }              
        }    
        return Some(c);
    }

    else if let Some((_, number)) = last_word{
        return written_word_to_char(number);
    }

    None
}



fn written_word_to_char(number: &str) -> Option<char>{
    match number{
        "one" => return Some('1'),
        "two" => return Some('2'),
        "three" => return Some('3'),
        "four" => return Some('4'),
        "five" => return Some('5'),
        "six" => return Some('6'),
        "seven" => return Some('7'),
        "eight" => return Some('8'),
        "nine" => return Some('9'),
        _ => return None
    };    
}

fn first_spelled_number(line: &str) -> Option<(usize, &str)>
{
    let spelled_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut ret: (usize, &str)= (usize::MAX, "zero"); 
    for (_, &number) in spelled_numbers.iter().enumerate() {
        // Check if the current spelled-out number is present in the input line
        if let Some(position) = line.find(number) {
            if position < ret.0 {
                ret = (position, number);
            }
        }
    }

    if ret == (usize::MAX, "zero"){
        return None;
    }
    // If no match is found
    Some(ret)
}


fn last_spelled_number(line: &str) -> Option<(usize, &str)>{
    let spelled_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut ret: (usize, &str)= (0, "zero"); 
    for (_, &number) in spelled_numbers.iter().enumerate() {
        // Check if the current spelled-out number is present in the input line
        if let Some(position) = line.rfind(number) {
            if position > ret.0 {
                ret = (position, number);
            }
        }
    }

    if ret == (0, "zero"){
        return None;
    }
    // If no match is found
    Some(ret)
}



fn create_two_digit_string(d1: Option<char>, d2: Option<char>) -> String
{
    let mut s: String = String::from("");
    if let Some(c1) = d1{
        if let Some(c2) = d2{
            s.push(c1);
            s.push(c2);
        }
    }
    s
}


fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}

fn first_digit(line: &str) -> Option<(usize,char)>
{
    for (index, c) in 
    line.chars().enumerate(){
        if c.is_digit(10) {
            return Some((index,c));
        }        
    }
    None
}

fn last_digit(line: &str) -> Option<(usize, char)>
{
    let length = line.len();
    for (index, c) in 
    line.chars().rev().enumerate(){
        if c.is_digit(10) {
            return Some((length-(index+1),c));
        }   
    }
    None
}


// Should return Some(char) when given a spelled-out number from one to nine
#[test]
fn should_return_some_char_with_spelled_out_number_from_one_to_nine() {
    let number = "one";
    let result = written_word_to_char(number);
    assert_eq!(result, Some('1'));
}

// Should return None when given a non-spelled-out number or an invalid string
#[test]
fn should_return_none_with_non_spelled_out_number_or_invalid_string() {
    let number = "ten";
    let result = written_word_to_char(number);
    assert_eq!(result, None);
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
    assert_eq!(result, Some((0,'1')));
}

// Should return None when given a string that does not contain any digits
#[test]
fn should_return_none_with_no_digits() {
    let line = "abc";
    let result = first_digit(line);
    assert_eq!(result, None);
}


// Returns Some(char) when the input string starts with a digit.
#[test]
fn test_ends_with_digit() {
    let line = String::from("123abc");
    assert_eq!(last_number(&line), Some('3'));
}

// Returns Some(char) when the input string starts with a spelled out number.
#[test]
fn test_ends_with_spelled_number() {
    let line = String::from("one 2 three");
    assert_eq!(last_number(&line), Some('3'));
}

#[test]
fn should_return_7() {
    let line = String::from("tdszrfzspthree2ttzseven5seven");
    let result = last_number(&line);
    assert_eq!(result, Some('7'));
}


// Returns Some(char) when the input string starts with a digit.
#[test]
fn test_starts_with_digit() {
    let line = String::from("123abc");
    assert_eq!(first_number(&line), Some('1'));
}

// Returns Some(char) when the input string starts with a spelled out number.
#[test]
fn test_starts_with_spelled_number() {
    let line = String::from("one 2 three");
    assert_eq!(first_number(&line), Some('1'));
}


#[test]
fn should_return_the_second_spelled_number() {
    let line = "two one";
    let result = last_spelled_number(line);
    assert_eq!(result, Some((4, "one")));
}



#[test]
fn should_return_some_position_number_with_line_containing_spelled_out_number() {
    let line = "This is one example";
    let result = first_spelled_number(line);
    assert_eq!(result, Some((8, "one")));
}

#[test]
fn should_return_the_first_spelled_number() {
    let line = "two one";
    let result = first_spelled_number(line);
    assert_eq!(result, Some((0, "two")));
}

// should return None when given a line that does not contain any spelled-out numbers
#[test]
fn should_return_none_with_line_not_containing_spelled_out_numbers() {
    let line = "This is an example";
    let result = first_spelled_number(line);
    assert_eq!(result, None);
}