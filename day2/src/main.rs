use std::env;
use std::fs;


fn main() {
    

    // read file
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  
    let lines: Vec<String> = open(filepath);

    let mut games: Vec<Game> = Vec::new();
    // parse lines
    for line in lines{
        // extract game id
        let id = extract_id(&line);
        let mut game = Game{id:id, runs:Vec::new()};
        
        let colon_split: Vec<&str> = line.split(':').collect();

        if colon_split.len() < 2{
            return;
        }

        let runs: Vec<&str> = colon_split[1].split(';').collect();
        for run in runs
        {
            let colors: Vec<&str> = run.split(',').collect();
            let mut parsed_run = Run {red: 0, green:0, blue: 0};
            
            for color in colors{
                let number_color_pair: Vec<&str> = color[1..].split(' ').collect();
                assert_eq!(number_color_pair.len(), 2);

                let number: u32 = number_color_pair[0].parse::<u32>().expect("error while converting &str to u32");
                
                match number_color_pair[1]{
                    "red" => parsed_run.red = number,
                    "green" => parsed_run.green = number,
                    "blue" => parsed_run.blue = number,
                    _ => (),
                }
            }

            game.runs.push(parsed_run);
        } 
        games.push(game);       
    }

    let mut sum:i32 = 0;
    for game in games{
        if game_possible(&game){
            sum += game.id;
        }        
    }    

    println!("Sum of game IDs of possible games: {}", sum);

        
    // game id
}

struct Run{
    red: u32,
    green: u32,
    blue: u32
}

struct Game{
    id: i32,
    runs: Vec<Run>
}

fn game_possible(game: &Game) -> bool{
    for run in &game.runs{
        if run.red > 12 || run.green > 13 || run.blue > 14
        {
            return false;
        }
    }    
    true
}

fn extract_id(line: &String) -> i32{
    let colon_index = line.find(':').expect("syntax error in input");
    let ix: &str = &line[5..colon_index];
    let index = ix.parse::<i32>().expect("error parsing input");
    index
}

#[test]
fn test_extract_id_with_colon() {
    let line = String::from("Game 123: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let result = extract_id(&line);
    assert_eq!(result, 123);
}







fn open(filepath: &str) -> Vec<String>
{
    println!("{}",filepath);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = contents.lines().map(|s| 
    s.to_string()).collect();
    lines
}
