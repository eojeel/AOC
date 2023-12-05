use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let regex_game_id = regex::Regex::new(r"Game (\d+):").unwrap();
    let regex_game_content = regex::Regex::new(r"Game \d+: (.+)").unwrap();

    if let Ok(lines) = read_lines("./data/day_02.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {


                let game_id = regex_game_id.captures(&ip).unwrap().get(1).unwrap().as_str();

                let game_content = regex_game_content.captures(&ip).unwrap().get(1).unwrap().as_str();

                println!("Game ID: {} {}", game_id, game_content);

            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
