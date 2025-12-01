use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let mut current_position: i32 = 50;
    let file_path = Path::new("input.txt");

    let mut amount_0: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let direction: &char = &line.chars().next().unwrap();
            let amount: i32 = line[1..line.len()].parse().expect("Couldn't parse");
            match direction {
                'L' => {
                    let previous_position = current_position.clone();
                    current_position = current_position - amount;
                    for i in 0..amount {
                        let pos = (previous_position - i - 1).rem_euclid(100);
                        if pos == 0 {
                            amount_0 += 1
                        }
                    }
                }
                'R' => {
                    let previous_position = current_position.clone();
                    current_position = current_position + amount;
                    for i in 0..amount {
                        let pos = (previous_position + i + 1).rem_euclid(100);
                        if pos == 0 {
                            amount_0 += 1;
                        }
                    }
                }
                _ => panic!("Something else found"),
            };
        }
    }
    println!("Final result: {}", amount_0)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
