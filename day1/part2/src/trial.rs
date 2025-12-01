use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_position: i32 = 50;
    let file_path = Path::new("input.txt");

    let mut amount_0: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let direction: char = line.chars().next().unwrap();
            let amount: i32 = line[1..line.len()].parse().expect("Couldn't parse");

            match direction {
                'L' => {
                    // Count how many times we pass through 0 during rotation
                    for _ in 0..amount {
                        current_position -= 1;
                        // Normalize to 0-99 range
                        if current_position < 0 {
                            current_position += 100;
                        }
                        if current_position == 0 {
                            amount_0 += 1;
                        }
                    }
                }
                'R' => {
                    // Count how many times we pass through 0 during rotation
                    for _ in 0..amount {
                        current_position += 1;
                        // Normalize to 0-99 range
                        if current_position >= 100 {
                            current_position -= 100;
                        }
                        if current_position == 0 {
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
