use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let input = Path::new("input.txt");
    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let parts = line
                .split(",")
                .filter_map(|product_id| {
                    product_id.split_once("-").and_then(|(l, r)| {
                        let left = l.parse::<i64>().ok()?;
                        let right = r.parse::<i64>().ok()?;
                        Some((left, right))
                    })
                })
                .collect::<Vec<(i64, i64)>>();
            let mut complete_total: i64 = 0;
            let mut complete_total_invalid: i64 = 0;
            for part in parts {
                let (total, total_invalid) = find_invalid_ids_range(part);
                complete_total += total;
                complete_total_invalid += total_invalid
            }
            println!("Found {complete_total_invalid} ids, their total = {complete_total}");
        }
    }
}

#[derive(Debug)]
struct SingleResult {
    result: i32,
    number: i64,
}

fn find_invalid_id(num: i64) -> SingleResult {
    let nr_str = num.to_string();
    let str_len = nr_str.len();

    for i in 1..=str_len / 2 {
        if str_len % i != 0 {
            continue;
        }

        let pattern = &nr_str[0..i];
        let repeats = str_len / i;

        let compare = pattern.repeat(repeats);

        if nr_str == compare {
            println!(
                "Found match while comparing pattern {pattern} which is repeated like {compare} in {nr_str}"
            );
            return SingleResult {
                result: 1,
                number: num,
            };
        }
    }

    SingleResult {
        result: 0,
        number: num,
    }
}

fn find_invalid_ids_range(input: (i64, i64)) -> (i64, i64) {
    let (start, end) = input;
    let mut total = 0;
    let mut total_invalid = 0;
    for i in start..=end {
        let result = find_invalid_id(i);
        if result.result == 1 {
            total += result.number;
            total_invalid += 1;
        }
    }
    (total, total_invalid)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_given_single_num() {
        let inputs: Vec<i64> = vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859];

        let expected_results = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let mut total: i64 = 0;
        let expected_total = 1227775554;
        for (i, input) in inputs.iter().enumerate() {
            let result: SingleResult = find_invalid_id(*input);
            if result.result == 1 {
                total += result.number
            }

            println!(
                "Result {:?}. Expected: {}",
                result.result, expected_results[i]
            );
            assert!(result.result == expected_results[i]);
        }

        assert!(total == expected_total)
    }

    #[test]
    fn test_example_given() {
        let inputs: Vec<(i64, i64)> = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (824824821, 824824827),
            (38593856, 38593862),
        ];

        let expected_results = vec![2, 2, 2, 1, 1, 0, 1, 1, 1];
        for (i, input) in inputs.iter().enumerate() {
            let (_, total_invalid) = find_invalid_ids_range(*input);
            let expected_result = expected_results[i];
            println!("Result {total_invalid:?}. Expected: {expected_result}");

            assert!(total_invalid == expected_result);
        }
    }
}
