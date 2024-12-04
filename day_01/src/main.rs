use std::fs; 

fn get_input_vectors(file_path: &str) -> (Vec<i32>, Vec<i32>){
    // Read input file 
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Create to vectors to be sorted 
    let mut left: Vec<i32> = Vec::with_capacity(contents.len());
    let mut right: Vec<i32> = Vec::with_capacity(contents.len());

    // For each line split and put number in that vector
    for line in contents.lines(){
        // Read from line and parse value
        let inputs: Vec<&str> = line.split_ascii_whitespace().collect();
        let left_val = inputs[0].parse::<i32>().expect("left parse error");
        let right_val = inputs[1].parse::<i32>().expect("right parse error");

        // Push both values to the vector 
        left.push(left_val); right.push(right_val);
    }

    // Sort
    left.sort();
    right.sort();

    return (left, right);
}


fn get_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32{
    let mut score = 0; 

    for item in left.iter(){
        let count = right.iter().filter(|v| *v == item).count() as i32;
        score += item * count;
    }

    score
}

fn main() {
    // Create to vectors to be sorted 
    let (left, right )= get_input_vectors("./input.txt");

    // Part 1: 
    // Calculate the differences!
    let dif: i32 = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();
    println!("Part 1 sum: {dif}");


    // Part 2: 
    // Calculate similarity
    let similarity = get_similarity_score(&left, &right);
    println!("Part 2 similarity: {similarity}");
}
