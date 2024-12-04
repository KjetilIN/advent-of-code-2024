use std::fs; 

#[derive(PartialEq, Debug)]
enum Token{
    M, 
    U,
    L, 
    LeftParentheses,
    RightParentheses, 
    Digit
}


fn find_and_mul(line: &str) -> i32{
    let mut next_token = Token::M; 


    // Keep track of first and second digit to be 
    let mut comma_found = false;
    let mut first_digit_str = String::new();
    let mut second_digit_str = String::new();

    // Keep track of the sum 
    let mut multiplied_sum = 0; 

    for char in line.chars(){
        match char {
            'm' => {
                if next_token == Token::M{
                    next_token = Token::U;
                }else{
                    // Reset
                    next_token = Token::M; 
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;

                }
            }, 
            'u' => {
                if next_token == Token::U{
                    next_token = Token::L;
                }else{
                    // Reset
                    next_token = Token::M; 
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;

                }
            }, 
            'l' => {
                if next_token == Token::L{
                    next_token = Token::LeftParentheses;
                }else{
                    // Reset
                    next_token = Token::M; 
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;

                }
            }, 
            '(' => {
                if next_token == Token::LeftParentheses{
                    next_token = Token::Digit;
                }else{
                    // Reset
                    next_token = Token::M; 
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;

                }
            }, 
            ',' => {
                if next_token == Token::Digit{
                    next_token = Token::Digit;
                    comma_found = true;
                }else{
                    // Reset
                    next_token = Token::M; 
                    first_digit_str = String::new();
                    second_digit_str = String::new();
                    comma_found = false;

                }
            }, 
            ')' => {
                if (next_token == Token::RightParentheses || next_token == Token::Digit) && !second_digit_str.is_empty() {
                    // Use the given information to calculate the multiplications
                    println!("Left number: '{}', right number: '{}'", first_digit_str, second_digit_str);

                    let left_number = first_digit_str.parse::<i32>().expect("could not parse left number");
                    let right_number = second_digit_str.parse::<i32>().expect("could not parse left number");

                    // Add the multiplied sum
                    multiplied_sum += left_number*right_number;

                }

                // Reset
                next_token = Token::M; 
                first_digit_str = String::new();
                second_digit_str = String::new();
                comma_found = false;
            }, 
            v => {
                // Next token is set to be a digit
                // We also allow the next digit to be , or )
                if next_token == Token::Digit{
                    // If the char is digit add it to one of the numbers
                    if v.is_ascii_digit(){
                        if comma_found{
                            second_digit_str.push(v); 
                        }else{
                            first_digit_str.push(v);
                        }
                    }else{
                        // There is some sort of unknown char 
                        // Reset 
                        next_token = Token::M; 
                        first_digit_str = String::new();
                        second_digit_str = String::new();
                        comma_found = false;
                    }

                }else{
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
    
    return multiplied_sum
}

fn part_1(file_path: &str) -> i32{
    // Read input file 
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return find_and_mul(&contents);
}

fn main() {
    let file_path = "./input.txt";

    let mul_part_1 = part_1(&file_path);
    println!("Part 1: {mul_part_1}");

}

#[cfg(test)]
mod tests {
    use crate::find_and_mul;

    #[test]
    fn test_find_and_mul_1(){
        let program = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = 161;
        assert_eq!(find_and_mul(program), result);
    }

    #[test]
    fn test_find_and_mul_2(){
        let program = "xmul(2 , 4)%&mul(3!@^do_not_mul(5,5)+mul(3*,64)then(mul?(11,8)mul(8,5!";
        let result = 25;
        assert_eq!(find_and_mul(program), result);
    }
}
