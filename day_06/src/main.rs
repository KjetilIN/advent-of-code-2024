use std::{collections::HashSet, fs}; 

enum Direction {
    North,
    West,
    East,
    South
}

impl Direction {
    fn right_turn(&self) -> Self{
        println!("Right turn!");
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

fn step(guard_index:&mut usize, cols:&usize, direction:&Direction){
    match direction{
        Direction::North => {
            *guard_index -= cols; 
        },
        Direction::West => {
            *guard_index -= 1; 
        },
        Direction::East => {
            *guard_index += 1; 
        },
        Direction::South => {
            *guard_index -= cols; 
        },
    }
}


fn main() {

    let file_path = "./test_1.txt";
    let content = fs::read_to_string(file_path).expect("could not read file");
    let rows = content.lines().into_iter().count(); 
    let cols = content
        .lines()
        .next()
        .map(|line| line.len())
        .expect("could not count columns");

    println!("Map Size: {}x{}", rows, cols);

    // Collect all chars to iterate over 
    let chars: Vec<char> = content.replace("\n", "").chars().collect();

    let mut positions:HashSet<u16> = HashSet::new();

    // First find the symbol 
    if let Some(mut guard_index) = &chars.clone().into_iter().position(|c| c == '^'){
        println!("Symbol index is: {}", guard_index);
        let guard_row = guard_index / rows; 
        let guard_col = guard_index % cols; 

        let mut current_direction = Direction::North;

        println!("Map position is: ({},{})", guard_row, guard_col);

        // While we can step in the given direction 
        while !next_index_out_of_bounds(&rows, &cols, &guard_index, &current_direction){
            println!("Step!");
            step(&mut guard_index, &cols, &current_direction);
            positions.insert(guard_index as u16);
            if chars[guard_index] == '#'{
                current_direction = current_direction.right_turn();
            }
        }

    }else{
        panic!("Puzzle input does not contain a starting guard!")
    }

    let part_1 = positions.len();
    println!("Part 1: {}", part_1);
}
