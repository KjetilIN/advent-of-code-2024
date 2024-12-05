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

#[derive(Debug)]
struct StateMachine {
    // Keep track of the next token that is expected 
    pub next_token: Token,
    pub comma_found: bool,
    pub first_digit_str: String,
    pub second_digit_str: String,
    enabled: bool,
    pub multiplied_sum: i32,
    pub is_part_2: bool
}

impl StateMachine {
    fn new(is_part_2: bool) -> Self {
        Self {
            next_token: Token::M,
            comma_found: false,
            first_digit_str: String::new(),
            second_digit_str: String::new(),
            enabled: true,
            multiplied_sum: 0,
            is_part_2: is_part_2,
        }
    }

    fn set_comma_found(&mut self){
        self.next_token = Token::Digit;
        self.comma_found = true; 
    }

    fn enable(&mut self){
        self.enabled = true;
    }

    fn disable(&mut self){
        self.enabled = false; 
    }

    fn is_ready_to_mul(&self) -> bool{
        return (self.next_token == Token::RightParentheses || self.next_token == Token::Digit) && !self.second_digit_str.is_empty()
    }

    fn multiply(&mut self){
        // Parse left and right numbers
        let left_number = self.first_digit_str
            .parse::<i32>()
            .expect("could not parse left number");
        let right_number = self.second_digit_str
            .parse::<i32>()
            .expect("could not parse left number");


        // Add the multiplied sum
        if self.is_part_2 {
            // For part 2: only multiply and add if enabled
            if self.enabled {
                self.multiplied_sum += left_number * right_number;
            }else{
            }

        } else {
            // For part 1: do it anyways
            self.multiplied_sum += left_number * right_number;
        }
    }

    fn append_digit(&mut self, v:char){
        if self.comma_found {
            self.second_digit_str.push(v);
        } else {
            self.first_digit_str.push(v);
        }
    }

    fn reset(&mut self){
        self.next_token = Token::M;
        self.comma_found = false;
        self.first_digit_str = String::new();
        self.second_digit_str = String::new();
    }


}

fn find_and_mul(line: &str, is_part_2: bool) -> i32 {
    // Create state machine 
    let mut state_machine: StateMachine = StateMachine::new(is_part_2);

    for (i, char) in line.chars().enumerate() {
        match char {
            'm' => {
                if state_machine.next_token == Token::M {
                    state_machine.next_token = Token::U;
                } else {
                    // Reset
                    state_machine.reset();
                }
            }
            'u' => {
                if state_machine.next_token == Token::U {
                    state_machine.next_token = Token::L;
                } else {
                    // Reset
                    state_machine.reset();
                }
            }
            'l' => {
                if state_machine.next_token == Token::L {
                    state_machine.next_token = Token::LeftParentheses;
                } else {
                    // Reset
                    state_machine.reset();
                }
            }
            '(' => {
                if state_machine.next_token == Token::LeftParentheses {
                    state_machine.next_token = Token::Digit;
                } else {
                    // Reset
                    state_machine.reset();
                }
            }
            ',' => {
                // Next token is either a digit and a comma
                if state_machine.next_token == Token::Digit && !state_machine.comma_found {
                    state_machine.set_comma_found();
                } else {
                    // Reset
                    state_machine.reset();
                }
            }
            ')' => {
                if state_machine.is_ready_to_mul(){
                    state_machine.multiply();
                }
                // Reset
                state_machine.reset();
            }
            v => {
                // Check if next token is a 
                if v == 'd' {
                    // Check if is do()
                    if line.chars().nth(i + 1) == Some('o')
                        && line.chars().nth(i + 2) == Some('(')
                        && line.chars().nth(i + 3) == Some(')')
                    {
                        state_machine.enable();
                        state_machine.reset();
                        continue;
                    }

                    // Check if is don't()
                    if line.chars().nth(i + 1) == Some('o')
                        && line.chars().nth(i + 2) == Some('n')
                        && line.chars().nth(i + 3) == Some('\'')
                        && line.chars().nth(i + 4) == Some('t')
                        && line.chars().nth(i + 5) == Some('(')
                        && line.chars().nth(i + 6) == Some(')')
                    {
                        state_machine.disable();
                        state_machine.reset();
                        continue;
                    }
                    
                }
                
                // Check if the next item is a digit that we can append 
                if v.is_ascii_digit() && state_machine.next_token == Token::Digit{
                    state_machine.append_digit(v);
                    continue;
                }

                // Reset
                state_machine.reset();
            }
        }
    }

    return state_machine.multiplied_sum;
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
