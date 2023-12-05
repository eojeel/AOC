use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./data/day_02.txt") {
        // Consumes the iterator, returns an (Optional) String
        let lines: Vec<String> = lines.filter_map(Result::ok).collect();
        println!("Part1: {}", part1(&lines));
        println!("Part2: {}", part2(&lines));
    }
}

fn part2(line: &[String]) -> usize {
    line.iter().map(|s| count2(s)).sum()
}

fn part1(line: &[String]) -> usize {
    line.iter().map(|s| count(s)).sum()
}

fn count(s: &str) -> usize {
    let (id, rows) = line_formatting(s);

    const RED_LIMIT: usize = 12;
    const GREEN_LIMIT: usize = 13;
    const BLUE_LIMIT: usize = 14;

    for event in rows {
        if calculate(&event, "red") > RED_LIMIT {
            return 0;
        }
        if calculate(&event, "green") > GREEN_LIMIT {
            return 0;
        }
        if calculate(&event, "blue") > BLUE_LIMIT {
            return 0;
        }
    }
    id
}

fn count2(s: &str) -> usize {
    let (id, rows) = line_formatting(s);

    let mut mred = 0;
    let mut mgreen = 0;
    let mut mblue = 0;

    for event in rows {
        mred = mred.max(calculate2(&event, "red"));
        mgreen = mgreen.max(calculate2(&event, "green"));
        mblue = mblue.max(calculate2(&event, "blue"));
    }
    mblue * mgreen * mred
}

fn calculate2(v: &[(i32, String)], color: &str) -> usize {
    v.iter()
        .filter(|(_, c)| c == color)
        .map(|(n, _)| *n as usize)
        .max()
        .unwrap_or(0)
}

fn calculate(v: &[(i32, String)], color: &str) -> usize {
    v.iter()
        .filter(|(_, c)| c == color)
        .map(|(n, _)| *n as usize)
        .sum()
}

fn line_formatting(s: &str) -> (usize, Vec<Vec<(i32, String)>>) {
    let tab: Vec<&str> = s.split(": ").collect();
    let left: Vec<&str> = tab[0].split(' ').collect();
    let games: Vec<&str> = tab[1].split("; ").collect();

    let id = left[1].parse::<usize>().unwrap();
    let res = games
        .iter()
        .map(|e| {
            e.split(", ")
                .map(|s| {
                    let event: Vec<&str> = s.split(' ').collect();
                    let n = event[0].parse::<i32>().unwrap();
                    let color = event[1].to_string();
                    (n, color)
                })
                .collect()
        })
        .collect();

    (id, res)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
