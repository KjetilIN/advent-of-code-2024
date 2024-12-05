use std::fs;

fn is_safe(numbers: &Vec<i16>) -> bool {
    assert!(numbers.len() > 2);

    // Check if it should be increasing or decreasing
    if numbers[0] > numbers[1] {
        // The numbers are decreasing
        for (i, numb) in numbers.iter().enumerate() {
            // If not last number
            if i + 1 != numbers.len() {
                if numb <= &numbers[i + 1] {
                    return false;
                }

                // Check the difference
                let dif = numb - numbers[i + 1];
                if dif > 3 || dif <= 0 {
                    return false;
                }
            }
        }
    } else if numbers[0] < numbers[1] {
        // The numbers are increasing
        for (i, numb) in numbers.iter().enumerate() {
            // If not last number
            if i + 1 != numbers.len() {
                if numb >= &numbers[i + 1] {
                    return false;
                }

                // Check the difference
                let dif = numbers[i + 1] - numb;
                if dif > 3 || dif <= 0 {
                    return false;
                }
            }
        }
    } else {
        // There are some numbers that are equal, i.e return false
        return false;
    }
    // OK
    true
}

fn is_sequence_increasing(numbers: &Vec<i16>) -> bool {
    let mut increasing_count = 0;
    let mut decreasing_count = 0;

    for (i, numb) in numbers.iter().enumerate() {
        if i + 1 != numbers.len() {
            if numb > &numbers[i + 1] {
                decreasing_count += 1;
            } else if numb < &numbers[i + 1] {
                increasing_count += 1;
            }
        }

        // Check for early breaks
        if increasing_count >= 3 {
            return true;
        }

        if decreasing_count >= 3 {
            return false;
        }
    }

    // Compare at the end
    increasing_count > decreasing_count
}

fn is_safe_with_rm(numbers: &Vec<i16>) -> bool {
    // First find out if the sequence is increasing
    if is_sequence_increasing(numbers) {
        // The numbers are increasing
        for (i, numb) in numbers.iter().enumerate() {
            // If not last number
            if i + 1 != numbers.len() {
                if numb >= &numbers[i + 1] {
                    // RULE BREAK
                    let mut first_numbers = numbers.clone();
                    first_numbers.remove(i);

                    let mut second_numbers = numbers.clone();
                    second_numbers.remove(i + 1);

                    return is_safe(&first_numbers) || is_safe(&second_numbers);
                }

                // Check the difference
                let dif = numbers[i + 1] - numb;
                if dif > 3 || dif <= 0 {
                    // RULE BREAK

                    let mut first_numbers = numbers.clone();
                    first_numbers.remove(i);

                    let mut second_numbers = numbers.clone();
                    second_numbers.remove(i + 1);

                    return is_safe(&first_numbers) || is_safe(&second_numbers);
                }
            }
        }
    } else {
        // The numbers are decreasing
        for (i, numb) in numbers.iter().enumerate() {
            // If not last number
            if i + 1 != numbers.len() {
                if numb <= &numbers[i + 1] {
                    // RULE BREAK

                    let mut first_numbers = numbers.clone();
                    first_numbers.remove(i);

                    let mut second_numbers = numbers.clone();
                    second_numbers.remove(i + 1);

                    return is_safe(&first_numbers) || is_safe(&second_numbers);
                }

                // Check the difference
                let dif = numb - numbers[i + 1];
                if dif > 3 || dif <= 0 {
                    // RULE BREAK

                    let mut first_numbers = numbers.clone();
                    first_numbers.remove(i);

                    let mut second_numbers = numbers.clone();
                    second_numbers.remove(i + 1);

                    return is_safe(&first_numbers) || is_safe(&second_numbers);
                }
            }
        }
    }

    true
}

#[allow(dead_code)]
fn numbers_to_vec(line: &str) -> Vec<i16> {
    line.split_whitespace()
        .map(|v| v.parse::<i16>().expect("parse number in line error"))
        .collect()
}

fn safe_line_count(file_path: &str) -> i16 {
    let mut safe = 0;

    // Read input file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let numbers: Vec<i16> = line
            .split_whitespace()
            .map(|v| v.parse::<i16>().expect("parse number in line error"))
            .collect();
        if is_safe(&numbers) {
            safe += 1;
        }
    }

    safe
}

fn safe_line_count_with_rm_level(file_path: &str) -> i16 {
    let mut safe = 0;

    // Read input file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let numbers: Vec<i16> = line
            .split_whitespace()
            .map(|v| v.parse::<i16>().expect("parse number in line error"))
            .collect();

        // First use the regular check, without removing
        // Then use the remove to test if it works with remove
        if is_safe(&numbers) {
            safe += 1;
        } else if is_safe_with_rm(&numbers) {
            safe += 1;
        }
    }

    safe
}

fn main() {
    // Part 1
    let safe_lines = safe_line_count("./input.txt");
    println!("Part 1: {safe_lines}");

    // Part 2
    let safe_lines_with_rm = safe_line_count_with_rm_level("./input.txt");
    println!("Part 2: {safe_lines_with_rm}");
}

#[cfg(test)]
mod tests {
    use crate::{is_sequence_increasing, numbers_to_vec};

    #[test]
    fn test_is_sequence_increasing_1() {
        let numbers = "34 32 29 28 26 25";
        let numb_vec = numbers_to_vec(&numbers);
        assert!(
            is_sequence_increasing(&numb_vec) == false,
            "Expected false was: {:?}",
            is_sequence_increasing(&numb_vec)
        )
    }
}
