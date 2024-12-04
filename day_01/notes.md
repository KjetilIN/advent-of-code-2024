# Day 01 - Description

Help the Elvish Senior Historians locate the missing Chief Historian by solving puzzles that reconcile two inconsistent lists of historically significant location IDs. Part 1 involves calculating the total distance between paired numbers from the two lists, while Part 2 requires computing a similarity score based on the frequency of overlapping numbers. Each solved puzzle earns a star, with the goal of collecting 50 stars to save Christmas.

Link to complete description: https://adventofcode.com/2024/day/1

## Approach/Algorithm

### Part 1: 
- Read the file and create to vectors. 
- Sort the vectors
- Use method chaining in rust to calculate the final difference 

### Part 2: 
- Use the sorted vectors from before
- Iterate the left vector
- For each left vector, count how many instances there are of the item
- Add to the score

(I could have made it a lot better, but Rust does filter operations quite fast)

## Results 

Part 1: 
```text
real    0m0,083s
user    0m0,059s
sys     0m0,024s
```

Part 2: 
```text
real    0m0,472s
user    0m0,328s
sys     0m0,195s
```

