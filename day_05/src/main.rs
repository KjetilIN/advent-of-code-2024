use std::{collections::HashMap, fs, ops::Index, sync::atomic::{AtomicU16, Ordering}};

use rayon::iter::{IntoParallelIterator, ParallelIterator}; 

fn parse_input(file_path: &str) -> (HashMap<u16, Vec<u16>> , Vec<String>){
    let content = fs::read_to_string(file_path).expect("Could not read file");

    // Rules are stored in a hashmap 
    let mut rule_map: HashMap<u16, Vec<u16>> = HashMap::new();

    // Updates are stored as vector of str
    let mut updates_vec: Vec<String> = Vec::with_capacity(10);

    for line in content.lines(){
        // A Rule is being parsed 
        if line.contains("|"){
            // Expect there to be two numbers
            let numbers: Vec<u16> = line.split("|").map(|f| f.parse::<u16>().unwrap()).collect();
            assert!(numbers.len() == 2);

            // Add rule to hashmap 
            if let Some(current_list) = rule_map.get_mut(&numbers[0]){
                current_list.push(numbers[1]);
            }else{
                rule_map.insert(numbers[0], vec![numbers[1]]);
            }

        }else if !line.is_empty(){
            // An update is being parsed 
            updates_vec.push(String::from(line));
        }
    }

    return (rule_map, updates_vec);

}


fn process_rules_part_1(rules_map: &HashMap<u16, Vec<u16>>, updates_list: &Vec<String>) -> u16{

    // Create new atomic u16 to keep track of the total sum
    let total_sum = AtomicU16::new(0);
    
    // Iterate over each rule, and create a thread 
    (0..updates_list.len()).into_par_iter().for_each(|i| {
        // Rule to currently check
        let update_numbs: Vec<u16> = updates_list[i].split(",").map(|f| f.parse::<u16>().unwrap()).collect();

        let mut has_broken_rule = false; 
        let mut has_rule = false; 

        for (i, key) in update_numbs.iter().enumerate(){
            // Check only if number is in the vector 
            if update_numbs.contains(&key){
                // If the key is in the Rule Hashmap, we know we need to check and evaluate it 
                if let Some(values) = rules_map.get(&key){

                    // Check that there is no numbers that are before the index
                    // If there is, we have a rule break, and can exit early 
                    for j in 0..i{
                        if values.contains(&update_numbs[j]){
                            // Exit early due to rule break 
                            has_broken_rule = true; 
                            break; 
                        }
                    }
                    // Exit if rule break
                    if has_broken_rule { break; }

                    // No rule, break, i.e test all values 
                    (i..update_numbs.len()).into_iter().for_each(|value_index|{
                        let current_numb = &update_numbs[value_index];
                        if values.contains(current_numb){
                            has_rule = true; 
                        }
                    });
                }
            }   
        }

        // Only add if there is no rule break and if there is a rule that is valid
        if has_rule && !has_broken_rule{
            let mid_number = update_numbs[update_numbs.len()/2];
            total_sum.fetch_add(mid_number, Ordering::Relaxed);
        }


    });

    return total_sum.load(Ordering::Relaxed);
}

// Check rule method
fn is_following_rules(update: &Vec<u16>, rules_map: &HashMap<u16, Vec<u16>>) -> (bool, usize, usize){
    for (key_index, key) in update.iter().enumerate(){
        // Check only if number is in the vector 
        if update.contains(&key){
            // If the key is in the Rule Hashmap, we know we need to check and evaluate it 
            if let Some(values) = rules_map.get(&key){

                // Check that there is no numbers that are before the index
                // If there is, we have a rule break, and can exit early 
                for value_index in 0..key_index{
                    if values.contains(&update[value_index]){
                        // Exit early due to rule break 
                        return (false, key_index, value_index);
                    }
                }
            }
        }
    }

    return (true, 0, 0); 
}


// Use the hashmap rule to repair the vector 
fn repair_update(mut update: Vec<u16>, rules_map: &HashMap<u16, Vec<u16>>) -> u16{
    // While not fixed 
    let (mut ok, mut key_index, mut value_index) = is_following_rules(&update, rules_map);
    while !ok && rules_map.contains_key(&update[key_index]) {
        // Ensure the key exists in the rules map
        if let Some(_) = rules_map.get(&update[key_index]) {
            // Swap the value behind the key
            update.swap(key_index, value_index);
        } else {
            // If the key doesn't exist in the rules, return 0
            return 0;
        }

        // Recheck the rules
        (ok, key_index, value_index) = is_following_rules(&update, rules_map);
    }

    if !rules_map.contains_key(&update[key_index]){
        return 0; 
    }

    // Return the middle part 
    return update[update.len()/2];
}


fn process_rules_part_2(rules_map: &HashMap<u16, Vec<u16>>, updates_list: &Vec<String>) -> u16{

    // Create new atomic u16 to keep track of the total sum
    let total_sum = AtomicU16::new(0);
    
    // Iterate over each rule, and create a thread 
    (0..updates_list.len()).into_par_iter().for_each(|i| {
        // Rule to currently check
        let update_numbs: Vec<u16> = updates_list[i].split(",").map(|f| f.parse::<u16>().unwrap()).collect();
        let mut has_broken_rule = false; 
        for (i, key) in update_numbs.iter().enumerate(){
            // Check only if number is in the vector 
            if update_numbs.contains(&key){
                // If the key is in the Rule Hashmap, we know we need to check and evaluate it 
                if let Some(values) = rules_map.get(&key){

                    // Check that there is no numbers that are before the index
                    // If there is, we have a rule break, and can exit early 
                    for j in 0..i{
                        if values.contains(&update_numbs[j]){
                            // Exit early due to rule break 
                            has_broken_rule = true; 
                            break; 
                        }
                    }
                    // Exit if rule break
                    if has_broken_rule { break; }
                }
            }   
        }

        // If there is a broken rule, we repair it and return the middle number 
        // If neither is true, we return nothing 
        if has_broken_rule{
            let mid_number = repair_update(update_numbs, rules_map);
            total_sum.fetch_add(mid_number, Ordering::Relaxed);
        }
    });

    return total_sum.load(Ordering::Relaxed);
}

fn main() {
    let file_path = "./input.txt";

    let (rules, update_list) = parse_input(file_path);
    println!("Rules size: {}", rules.len());
    println!("Updates: {}\n", update_list.len());

    // Part 1 
    let total_sum_part_1 = process_rules_part_1(&rules, &update_list);
    println!("Part 1: {}", total_sum_part_1);

    // Part 2
    let total_sum_part_2 = process_rules_part_2(&rules, &update_list);
    println!("Part 2: {}", total_sum_part_2);

}

#[cfg(test)]
mod tests {
    use crate::{parse_input, process_rules_part_1, process_rules_part_2, repair_update};

    #[test]
    fn test_temp_input_file(){
        let file_path = "./test_1.txt";
        let (rules, update_list) = parse_input(file_path);
        assert_eq!(process_rules_part_1(&rules, &update_list), 143);
    }

    #[test]
    fn test_repair_func_1(){
        let file_path = "./test_1.txt";
        let (rules, _) = parse_input(file_path);

        let sequence = "75,97,47,61,53";
        let numbs: Vec<u16> = sequence.split(",").map(|f| f.parse::<u16>().unwrap()).collect();
        
        assert_eq!(repair_update(numbs, &rules), 47);
    }

    #[test]
    fn test_repair_func_2(){
        let file_path = "./test_1.txt";
        let (rules, _) = parse_input(file_path);

        let sequence = "61,13,29";
        let numbs: Vec<u16> = sequence.split(",").map(|f| f.parse::<u16>().unwrap()).collect();
        
        assert_eq!(repair_update(numbs, &rules), 29);
    }

    #[test]
    fn test_repair_func_3(){
        let file_path = "./test_1.txt";
        let (rules, _) = parse_input(file_path);

        let sequence = "97,13,75,29,47";
        let numbs: Vec<u16> = sequence.split(",").map(|f| f.parse::<u16>().unwrap()).collect();
        
        assert_eq!(repair_update(numbs, &rules), 47);
    }

    #[test]
    fn test_temp_input_file_on_repair(){
        let file_path = "./test_1.txt";
        let (rules, update_list) = parse_input(file_path);
        assert_eq!(process_rules_part_2(&rules, &update_list), 123);
    }
}

