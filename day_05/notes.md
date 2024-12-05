# Day 05 - Description

The task is to validate and, if necessary, reorder print job updates based on a set of page ordering rules. Each update consists of a sequence of pages, and the ordering rules dictate the relative order between specific pairs of pages (`X|Y`, meaning page `X` must appear before page `Y`). The challenge is divided into two parts:

1. **Part 1: Validate Updates**  
   - Identify which updates adhere to the ordering rules.
   - For valid updates, calculate the "middle page" (median page number) and sum these values.

2. **Part 2: Correct Invalid Updates**  
   - For updates that violate the ordering rules, reorder the pages to conform.
   - Calculate the middle page of the corrected updates and sum these values.

Link to complete description: https://adventofcode.com/2024/day/5

## Approach/Algorithm

### Part 1
- We only care about if there are rule breaks and if there is a single valid rule. 
- Create a hashmap `HashMap<u16, Vec<u16>>` to keep track of rules, and use a `Vec<String>` for keeping track of updates
- For each update, we do a rule check (which I do in multiple threads)
- If there is a rule break, we stop checking the update
  - Only one rule break is required and then we would exit the loop
- If there is a valid rule that can apply, then we add the middle number of the update string to the total sum
  - Only one valid rule is required to be found. We check all rule breaks before, so either the rule applies, or it doesn't. 

## Result

### Part 1

```text
real    0m0,062s
user    0m0,056s
sys     0m0,022s
```

