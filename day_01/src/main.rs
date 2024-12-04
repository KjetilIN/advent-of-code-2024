use std::fs; 


fn main() {
    // Read input file 
    let file_path = "./input.txt";
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

    // Calculate the differences!
    let dif: i32 = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();
    println!("Part 1 sum: {dif}");



    // Part 2: 











}
