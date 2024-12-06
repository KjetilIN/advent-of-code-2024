use std::{char, collections::HashSet, fs}; 

enum Direction {
    North,
    West,
    East,
    South
}

impl Direction {
    fn right_turn(&self) -> Self{
        match &self {
            Direction::North => return Direction::East,
            Direction::West => return Direction::North,
            Direction::East => return Direction::South,
            Direction::South => return Direction::West,
        }
    }    
}

fn next_index_out_of_bounds(rows:&usize, cols:&usize, current_guard_index: &usize, direction:&Direction) -> bool{
    match direction{
        Direction::North => {
            // Check if we move one step to the north would lead to moving outside of the map
            // This would only work if the guard is on the first row, meaning that it is only possible if it is larger than a row
            return current_guard_index < cols; 
        },
        Direction::West => {
            // Check if we can take a step west 
            return current_guard_index % cols == 0;
        },
        Direction::East => {
            // Check if we can step east 
            return current_guard_index % cols == cols - 1; 
        },
        Direction::South => {
            // Check if we can step south 
            return current_guard_index + cols > rows*cols;
        },
    }
}

fn step(guard_index:&usize, cols:&usize, direction:&Direction) -> usize{
    match direction{
        Direction::North => {
            return guard_index - cols; 
        },
        Direction::West => {
            return guard_index - 1; 
        },
        Direction::East => {
            return guard_index + 1; 
        },
        Direction::South => {
            return guard_index + cols; 
        },
    }
}

fn get_map(file_path: &str) -> (usize, usize, Vec<char>){
    let content = fs::read_to_string(file_path).expect("could not read file");
    let rows = content.lines().into_iter().count(); 
    let cols = content
        .lines()
        .next()
        .map(|line| line.len())
        .expect("could not count columns");

    // Collect all chars to iterate over 
    let chars: Vec<char> = content.replace("\n", "").chars().collect();

    return (rows, cols, chars);
}

fn find_path(rows:&usize, cols:&usize, chars:&Vec<char>) -> usize{
    let mut positions:HashSet<u16> = HashSet::new();

    // First find the symbol 
    if let Some(mut guard_index) = &chars.clone().into_iter().position(|c| c == '^'){
        // Include the starting position 
        positions.insert(guard_index as u16);

        // Direction during the start is always north
        let mut current_direction = Direction::North;        

        // While we can step in the given direction 
        while !next_index_out_of_bounds(&rows, &cols, &guard_index, &current_direction){
            // Check next step
            let next_step: usize = step(&guard_index, &cols, &current_direction);
            if chars[next_step] == '#'{
                current_direction = current_direction.right_turn();
            }else{
                // Take a step
                //println!("Step!");
                guard_index = next_step;
                positions.insert(next_step as u16);
            }
        }

    }else{
        panic!("Puzzle input does not contain a starting guard!")
    }

    // Return the positions visited  
    positions.len()
}




fn main() {
    let file_path = "./input.txt";
    let (rows, cols, chars) = get_map(file_path);
    let path_len = find_path(&rows, &cols, &chars);

    // Part 1: 4988
    println!("Part 1: {}", path_len);
}


#[cfg(test)]
mod tests {
    use crate::{find_path, get_map};

    #[test]
    fn test_part_1(){
        let file_path = "./test_1.txt";
        let (rows, cols, chars) = get_map(file_path);
        let path_len = find_path(&rows, &cols, &chars);
        assert_eq!(path_len, 41);
    }

}