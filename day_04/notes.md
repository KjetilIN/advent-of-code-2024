# Day 04 - Description

Analyze a grid-based word search puzzle to identify specific patterns involving the word **XMAS**.  

### Part 1  
Find all occurrences of the word **XMAS** in the grid.  
- Search horizontally, vertically, and diagonally in all directions (forward, backward, up, down, and both diagonal orientations).  
- Words can overlap, so track each independent occurrence.  

Efficient pattern matching across the 2D grid is key, leveraging algorithms to handle all search directions simultaneously.  

### Part 2  
Identify instances of **X-MAS**, where two **MAS** substrings form the shape of an **X**:  
- Each **MAS** can appear forward or backward.  
- Check all valid positions for the X-pattern within the grid, matching rows, columns, and diagonal constraints to ensure correct alignment.  

This part requires nested grid traversal and positional validation to detect overlapping and rotated patterns effectively. Parsing both conditions demands careful handling of boundaries and relative positions in the grid.

Link to complete description: https://adventofcode.com/2024/day/04

## Approach/Algorithm

### Part 1 
- Look for `X` instead of the whole xmas word in each direction
- Only when found we search in the possible directions, if feasible
- Use `rayon` to parallelize the process

### Part 2
- Look for `A` character before checking the cross
- Check if only `A`, if it is within the valid boundaries (must not be on the edge of the map)
- Use `rayon` to check only the valid indexes where there is an `A`
- Check if any of the valid combinations of a cross is present. If it is, increment the count

## Result

### Part 1

```text
real    0m0,091s
user    0m0,092s
sys     0m0,019s
```

### Part 2

```text
real    0m0,088s
user    0m0,072s
sys     0m0,028s
```