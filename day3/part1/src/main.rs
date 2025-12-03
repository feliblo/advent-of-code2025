use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = Path::new("input.txt");

    let mut total = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            total += max_joltage_in_bank(&line);
        }
    }
    println!("Total {total}");
}

fn max_joltage_in_bank(input: &str) -> u32 {
    let mut max_ten = 0;
    let mut max_i = 0;
    let dont_take_last = &input[0..input.len() - 1];
    for (i, character) in dont_take_last.chars().enumerate() {
        let tens = character
            .to_digit(10)
            .expect("Should never fail, since all are digits");
        if tens > max_ten {
            max_ten = tens;
            max_i = i;
        }
    }

    let digits_check = &input[max_i + 1..input.len()];
    let mut max_digit = 0;
    for (_, char) in digits_check.chars().enumerate() {
        let digit = char.to_digit(10).expect("Should never fail");
        if digit > max_digit {
            max_digit = digit;
        }
    }

    println!("max_ten={max_ten}, max_digit={max_digit}");
    (max_ten * 10) + max_digit
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

        let expected_joltage: Vec<u32> = vec![98, 89, 78, 92];

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
