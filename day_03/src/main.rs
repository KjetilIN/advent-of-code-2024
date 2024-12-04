use std::fs;

#[derive(PartialEq, Debug)]
enum Token {
    M,
    U,
    L,
    LeftParentheses,
    RightParentheses,
    Digit,
}

fn find_and_mul(line: &str, is_part_2: bool) -> i32 {
    // Keep track of the next statement
    let mut next_token = Token::M;

    // Keep track of first and second digit to be
    let mut comma_found = false;
    let mut first_digit_str = String::new();
    let mut second_digit_str = String::new();

    // Keep track if enabled (for part 2)
    let mut enabled = true;

    // Keep track of the sum
    let mut multiplied_sum = 0;

    for (i, char) in line.chars().enumerate() {
        match char {
            'm' => {
                if next_token == Token::M {
                    next_token = Token::U;
                } else {
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
            'u' => {
                if next_token == Token::U {
                    next_token = Token::L;
                } else {
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
            'l' => {
                if next_token == Token::L {
                    next_token = Token::LeftParentheses;
                } else {
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
            '(' => {
                if next_token == Token::LeftParentheses {
                    next_token = Token::Digit;
                } else {
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
            ',' => {
                // Next token is either a digit and a comma
                if next_token == Token::Digit && !comma_found {
                    next_token = Token::Digit;
                    comma_found = true;
                } else {
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
            ')' => {
                if (next_token == Token::RightParentheses || next_token == Token::Digit)
                    && !second_digit_str.is_empty()
                {
                    // Parse left and right numbers
                    let left_number = first_digit_str
                        .parse::<i32>()
                        .expect("could not parse left number");
                    let right_number = second_digit_str
                        .parse::<i32>()
                        .expect("could not parse left number");

                    // Add the multiplied sum
                    if is_part_2 {
                        // For part 2: only multiply and add if enabled
                        if enabled {
                            multiplied_sum += left_number * right_number;
                        }
                    } else {
                        // For part 1: do it anyways
                        multiplied_sum += left_number * right_number;
                    }
                }

                // Reset
                next_token = Token::M;
                first_digit_str = String::new();
                second_digit_str = String::new();
                comma_found = false;
            }
            v => {
                // Next token is set to be a digit
                // We also allow the next digit to be , or )
                if next_token == Token::Digit {
                    // If the char is digit add it to one of the numbers
                    if v.is_ascii_digit() {
                        if comma_found {
                            second_digit_str.push(v);
                        } else {
                            first_digit_str.push(v);
                        }
                    } else {
                        // There is some sort of unknown char
                        // Reset
                        next_token = Token::M;
                        first_digit_str = String::new();
                        second_digit_str = String::new();
                        comma_found = false;
                    }
                } else if v == 'd' {
                    // Check if is do()
                    if line.chars().nth(i + 1) == Some('o')
                        && line.chars().nth(i + 2) == Some('(')
                        && line.chars().nth(i + 3) == Some(')')
                    {
                        enabled = true;
                    }

                    // Check if is don't()
                    if line.chars().nth(i + 1) == Some('o')
                        && line.chars().nth(i + 2) == Some('n')
                        && line.chars().nth(i + 3) == Some('\'')
                        && line.chars().nth(i + 4) == Some('t')
                        && line.chars().nth(i + 5) == Some('(')
                        && line.chars().nth(i + 6) == Some(')')
                    {
                        enabled = false;
                    }
                } else {
                    // There is some sort of unknown char
                    // Reset
                    next_token = Token::M;
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;
                }
            }
        }
    }

    return multiplied_sum;
}

fn part_1(file_path: &str) -> i32 {
    // Read input file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return find_and_mul(&contents, false);
}

fn part_2(file_path: &str) -> i32 {
    // Read input file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return find_and_mul(&contents, true);
}

fn main() {
    let file_path = "./input.txt";

    let mul_part_1 = part_1(&file_path);
    println!("Part 1: {mul_part_1}");

    let mul_part_2 = part_2(file_path);
    println!("Part 2: {mul_part_2}");
}

#[cfg(test)]
mod tests {
    use crate::find_and_mul;

    #[test]
    fn test_find_and_mul_1() {
        let program = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = 161;
        assert_eq!(find_and_mul(program, false), result);
    }

    #[test]
    fn test_find_and_mul_2() {
        let program = "xmul(2 , 4)%&mul(3!@^do_not_mul(5,5)+mul(3*,64)then(mul?(11,8)mul(8,5!";
        let result = 25;
        assert_eq!(find_and_mul(program, false), result);
    }

    #[test]
    fn test_find_and_mul_part_2() {
        let program = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = 48;
        assert_eq!(find_and_mul(program, true), result);
    }
}
