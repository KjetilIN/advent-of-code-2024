use std::{collections::HashMap, fs, ops::Index, sync::atomic::{AtomicU16, Ordering}};

use rayon::iter::{IntoParallelIterator, ParallelIterator}; 

fn parse_input(file_path: &str) -> (HashMap<u16, u16> , Vec<String>){
    let content = fs::read_to_string(file_path).expect("Could not read file");

    // Rules are stored in a hashmap 
    let mut rule_map: HashMap<u16, u16> = HashMap::new();

    // Updates are stored as vector of str
    let mut updates_vec: Vec<String> = Vec::with_capacity(10);

    for line in content.lines(){
        // A Rule is being parsed 
        if line.contains("|"){
            // Expect there to be two numbers
            let numbers: Vec<u16> = line.split("|").map(|f| f.parse::<u16>().unwrap()).collect();
            assert!(numbers.len() == 2);

            // Add rule to hashmap 
            rule_map.insert(numbers[0], numbers[1]);

        }else if !line.is_empty(){
            // An update is being parsed 
            updates_vec.push(String::from(line));
        }
    }

    return (rule_map, updates_vec);

}


fn process_rules_part_1(rules_map: &HashMap<u16, u16>, updates_list: &Vec<String>) -> u16{

    // Create new atomic u16 to keep track of the total sum
    let total_sum = AtomicU16::new(0);
    
    // Iterate over each rule, and create a thread 
    (0..updates_list.len()).into_par_iter().for_each(|i| {
        // Rule to currently check
        let update_numbs: Vec<u16> = updates_list[i].split(",").map(|f| f.parse::<u16>().unwrap()).collect();

        // Store the sum of all middle update numbers 
        let mut update_mid_numbers_sum: u16  = 0; 

        for key in &update_numbs{
            // Check only if number is in the vector 
            if update_numbs.contains(&key){
                // If the key is in the Rule Hashmap, we know we need to check and evaluate it 
                if let Some(value) = rules_map.get(&key){
                    if let Some(key_index) = update_numbs.iter().position(|n| n == key){
                        if let Some(value_index) = update_numbs.iter().position(|n| n == value){
                            // There is a key and a value in the rule, and increment the sum by that value
                            // Check the ordering of the rule
                            if key_index > value_index{
                                let mid_index = (key_index + value_index) / 2; 
                                update_mid_numbers_sum += update_numbs[mid_index];
                            }

                        }
                    }
                    
                }
            }   
        }

        // Add the total sum for that given rule 
        total_sum.fetch_add(update_mid_numbers_sum, Ordering::Relaxed);

    });

    return total_sum.load(Ordering::Relaxed);
}

fn main() {
    let file_path = "./input.txt";

    let (rules, update_list) = parse_input(file_path);
    println!("Rules size: {}", rules.len());
    println!("Updates: {}", update_list.len());

    // Part 1 
    let total_sum_part_1 = process_rules_part_1(&rules, &update_list);
    println!("Part 1: {}", total_sum_part_1);

}

