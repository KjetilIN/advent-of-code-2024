# Day 02 - Description

Assist engineers at the Red-Nosed Reindeer reactor by analyzing safety reports. Part 1 involves identifying safe reports where levels consistently increase or decrease within a difference of 1 to 3. In Part 2, the Problem Dampener allows tolerating one bad level per report, making otherwise unsafe reports count as safe. Your goal is to determine the total number of safe reports under these rules. 

Link to complete description: https://adventofcode.com/2024/day/2

## Approach/Algorithm

### Part 1
- Check line by line 
- If the first number is greater than the next then we now the sequence should be increasing 
- Else the sequence should be decreasing

(Could have improved the solution by doing it in parallel)

## Result

### Part 1

```text
real    0m0,085s
user    0m0,064s
sys     0m0,022s
```


