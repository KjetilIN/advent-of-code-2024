#!/bin/bash

# Check if the number argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <day_number>"
  exit 1
fi

# Extract the day number from the first argument
day_number="$1"
day_folder="day_$day_number"

# Create a new cargo project
cargo new $day_folder

# Move into the newly created project folder
cd $day_folder

# Create a notes.md file with Description, Input, Output, Approach, Learnings, Code Snippets, and Challenges Faced headings
echo -e "# Day $day_number - Description\n" >> notes.md
echo -e "Link to complete description: https://adventofcode.com/2024/day/$day_number" >> notes.md
echo -e "## Approach/Algorithm\n\n" >> notes.md
echo -e "## Result\n\n" >> notes.md

# Print a message
echo "New project for Day $day_number created in $day_folder. notes.md file created."
