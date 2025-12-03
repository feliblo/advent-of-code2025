use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result;

fn main() {
    let file_path = Path::new("input.txt");

    let mut total = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            total += max_joltage_in_bank_bytes(&line);
        }
    }
    println!("Total {total}");
}

fn max_joltage_in_bank_bytes(input: &str) -> u64 {
    const BATTERIES_ON: usize = 12;

    let bytes = input.as_bytes();
    let mut search_space_start = 0;
    let mut remaining = BATTERIES_ON;
    let mut result_vec = Vec::with_capacity(BATTERIES_ON);

    while remaining > 0 {
        let search_space_end = bytes.len() - (remaining - 1);

        let mut best_digit = b'0';
        let mut best_index = search_space_start;

        for i in search_space_start..search_space_end {
            if bytes[i] > best_digit {
                best_digit = bytes[i];
                best_index = i;
                if best_digit == b'9' {
                    break;
                }
            }
        }

        result_vec.push((best_digit - b'0') as u8);
        search_space_start = best_index + 1;
        remaining -= 1;
    }

    let num: u64 = result_vec.iter().fold(0, |acc, &d| acc * 10 + d as u64);
    num
}

fn max_joltage_in_bank(input: &str) -> u64 {
    let mut start = 0;
    let mut remaining_digits = 12;
    let mut result_vec = Vec::with_capacity(remaining_digits);

    // If we still need to assign digits
    while remaining_digits > 0 {
        let current_search_space = &input[start..input.len() - (remaining_digits - 1)];
        let mut max_current_digit = 0;
        let mut max_i = 0;
        for (i, character) in current_search_space.chars().enumerate() {
            let current_digit = u64::from(
                character
                    .to_digit(10)
                    .expect("Should never fail, since all are digits"),
            );

            if current_digit > max_current_digit {
                max_current_digit = current_digit;
                max_i = i;
                println!("{}{}", max_current_digit, remaining_digits);
            }
            // insane optimization
            if current_digit == 9 {
                break;
            }
        }
        start = start + max_i + 1;
        result_vec.push(max_current_digit);
        remaining_digits -= 1;
    }

    let result = result_vec
        .into_iter()
        .map(|c| c.to_string())
        .collect::<String>()
        .parse()
        .expect("never happens");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let inputs = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        let expected_joltage: Vec<u64> =
            vec![987654321111, 811111111119, 434234234278, 888911112111];

        for (i, input) in inputs.iter().enumerate() {
            let outcome = max_joltage_in_bank(input);
            let expected_outcome = expected_joltage[i];
            println!(
                "Result: {outcome}, Expected: {expected_outcome}
            "
            );
            assert_eq!(outcome, expected_outcome);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
