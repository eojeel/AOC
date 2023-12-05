use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_1(line: &str) -> u32 {
    return clean_results(line);
}


fn part_2(line: &str) -> u32 {
    let mut new_line = line.to_string();

    println!("line: {}", new_line);

    for (word, digit) in WORDS_AND_DIGITS {
        new_line = new_line.replace(word, &format!("{}{}{}", word, digit, word));
    }

    println!("new_line: {}", new_line);

    return clean_results(&new_line);
}

fn clean_results(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));

    println!("{:?}", digits);

    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    println!("first {} last {}", first_digit, last_digit);

    return 10 * first_digit + last_digit;
}

fn main() -> io::Result<()> {
    let path = Path::new("./data/day_01.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part_1_total = 0;
    let mut part_2_total = 0;

    for line in reader.lines() {
        let line = line?;

        part_1_total += part_1(&line);
        part_2_total += part_2(&line);
    }

    println!("Total sum: {} {}", part_1_total, part_2_total);
    Ok(())
}

const WORDS_AND_DIGITS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];
