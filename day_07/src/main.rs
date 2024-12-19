use std::{fs};

#[derive(Clone, PartialEq, Eq)]
struct NumberSequence{
    numbers: Vec<usize>,
    sequence_sum: usize,
    op_bits: String
}

impl NumberSequence {
    fn new(numb1: usize, numb2: usize, start_op_is_multiply:bool) -> Self{
        let mut sum = 0; 
        let mut op_bits = String::new();
        if start_op_is_multiply{    
            sum = numb1 * numb2;
            op_bits.push_str("1");
        }else{
            sum = numb1 + numb2;
            op_bits.push_str("0");
        }
        Self { numbers: vec![numb1, numb2],op_bits, sequence_sum: sum}
    }

    fn append_new_numb(&mut self, new_numb:usize, is_multiply: bool){
        self.numbers.push(new_numb);
        if is_multiply{
            self.op_bits.push_str("1");
            self.sequence_sum *= new_numb
        }else{
            self.op_bits.push_str("0");
            self.sequence_sum += new_numb
        }
    }
}

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).expect("could not read file");
    let mut total_sum = 0; 

    for line in content.lines() {
        let numbers: Vec<usize> = line
            .replace(":", "")
            .split_ascii_whitespace()
            .map(|f| f.parse::<usize>().unwrap())
            .collect();


        let base_number = numbers[0];

        // Vector must be 3 or longer in length 
        assert!(numbers.len() >= 3, "Line in input file did not contain at least 3 numbers");

        let mut chains: Vec<NumberSequence> = Vec::new();
        let mut valid_chains: Vec<NumberSequence> = Vec::new();
        for i in 1..numbers.len(){
            if i == 1{
                // Create two new sequences if this is the first number
                let first_sequence = NumberSequence::new(numbers[1], numbers[2], true);
                if first_sequence.sequence_sum < base_number{
                    chains.push(first_sequence);
                }
                let second_sequence = NumberSequence::new(numbers[1], numbers[2], false);
                if second_sequence.sequence_sum < base_number{
                    chains.push(second_sequence);
                }
            }else{
                let mut new_sequences: Vec<NumberSequence> = Vec::new();
                for (i, sequence) in chains.clone().into_iter().enumerate(){
                    if i == chains.len() - 1{
                        continue;
                    }

                    // Try multiply 
                    let mut mult_seq: NumberSequence = sequence.clone();
                    mult_seq.append_new_numb(numbers[i], true);
                    if mult_seq.sequence_sum == base_number{
                        // Found a valid sequence 
                        valid_chains.push(mult_seq.clone());
                        new_sequences.push(mult_seq);
                    }else if mult_seq.sequence_sum < base_number{
                        // Valid sequence 
                        new_sequences.push(mult_seq);
                        
                    }

                    // Try plus 
                    let mut plus_seq: NumberSequence = sequence.clone();
                    plus_seq.append_new_numb(numbers[i], false);
                    if plus_seq.sequence_sum == base_number{
                        // Found a valid sequence 
                        valid_chains.push(plus_seq.clone());
                        new_sequences.push(plus_seq);
                    }else if plus_seq.sequence_sum < base_number{
                        // Valid sequence 
                        new_sequences.push(plus_seq);
                    }
                };

                // set the new sequences
                chains = new_sequences;  
            }

        }

        // Add all valid chains to that sequence 
        for seq in valid_chains{
            for numb in seq.numbers{
                total_sum += numb;
            }
        }
    }

    println!("Part 1: {}", total_sum)
}
