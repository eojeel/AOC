use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./data/day_03.txt") {
        // Consumes the iterator, returns an (Optional) String
        let lines: Vec<String> = lines.filter_map(Result::ok).collect();

        println!("{:?}", lines[0]);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
