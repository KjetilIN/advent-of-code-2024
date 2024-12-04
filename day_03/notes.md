# Day 03 - Description
Analyze corrupted memory to extract valid `mul(X,Y)` instructions and calculate the sum of their results.  

### Part 1  
Scan the input for valid `mul(X,Y)` commands. A valid command follows the exact pattern `mul(number,number)` where numbers are 1-3 digits. Ignore all other sequences. Parse the commands, compute their results, and return the total sum.  

### Part 2  
Add support for conditional instructions:  
- `do()` enables subsequent `mul` instructions.  
- `don't()` disables subsequent `mul` instructions.  

Process the input sequentially, tracking the current enable/disable state. Only execute `mul(X,Y)` if enabled by the most recent `do()` or `don't()` instruction. Sum up the results of all enabled `mul` commands for the final output.  

This challenge requires regex parsing for valid instructions and state management to handle conditional execution logic effectively.

Link to complete description: https://adventofcode.com/2024/day/3

## Approach/Algorithm

Instead of doing regular regex for solving these issues, we create a replicated state machine for checking the next char. 
It work the same way as compilers; first we specify the tokens we are interested in, then the order, and then what operations should happen if an illegal order and a legal order is give. 

### Part 1
- Created the state machine for handling input
- Validated each condition by creating test cases that test state machine 

### Part 2
- Added extra state for enabled
- Check if enabled before doing multiplication 
- If unknown character => check if it's '`do()` or `don't()`, and toggle enabled state appropriately 

## Result

### Part 1

```text
real    0m0,185s
user    0m0,168s
sys     0m0,017s
```

### Part 2 

```text
real    0m0,185s
user    0m0,163s
sys     0m0,022s
```