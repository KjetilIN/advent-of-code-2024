use std::{
    fs,
    sync::atomic::{AtomicU32, Ordering},
};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn part_1(content: String) -> u32 {
    // Get the number of rows and columns
    let rows = content.lines().count();
    let cols = content
        .lines()
        .next()
        .map(|line| line.len())
        .expect("could not count columns");

    // Print size
    println!("Map size: ({}x{})", rows, cols);

    // Replace newline characters and convert to Vec<char> for indexed access
    let content: Vec<char> = content.replace('\n', "").chars().collect();

    // Atomic counter for thread-safe counting
    let xmas_count = AtomicU32::new(0);

    // Process characters in parallel
    (0..content.len()).into_par_iter().for_each(|i| {
        let ch = content[i];

        // Only start a search if we find an 'X'
        if ch == 'X' {
            let mut count = 0;

            // Look right
            if i % cols < cols - 3
                && content.get(i + 1) == Some(&'M')
                && content.get(i + 2) == Some(&'A')
                && content.get(i + 3) == Some(&'S')
            {
                count += 1;
            }

            // Look left
            if i % cols >= 3
                && content.get(i - 1) == Some(&'M')
                && content.get(i - 2) == Some(&'A')
                && content.get(i - 3) == Some(&'S')
            {
                count += 1;
            }

            // Look up
            if i >= cols * 3
                && content.get(i - cols) == Some(&'M')
                && content.get(i - cols * 2) == Some(&'A')
                && content.get(i - cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Look down
            if i + cols * 3 < content.len()
                && content.get(i + cols) == Some(&'M')
                && content.get(i + cols * 2) == Some(&'A')
                && content.get(i + cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Look diagonal right down
            if i % cols < cols - 3
                && i + cols * 3 + 3 < content.len()
                && content.get(i + 1 + cols) == Some(&'M')
                && content.get(i + 2 + cols * 2) == Some(&'A')
                && content.get(i + 3 + cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Look diagonal left down
            if i % cols >= 3
                && i + cols * 3 >= cols * 3
                && content.get(i - 1 + cols) == Some(&'M')
                && content.get(i - 2 + cols * 2) == Some(&'A')
                && content.get(i - 3 + cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Look diagonal right up
            if i % cols < cols - 3
                && i >= cols * 3
                && content.get(i + 1 - cols) == Some(&'M')
                && content.get(i + 2 - cols * 2) == Some(&'A')
                && content.get(i + 3 - cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Look diagonal left up
            if i % cols >= 3
                && i >= cols * 3
                && content.get(i - 1 - cols) == Some(&'M')
                && content.get(i - 2 - cols * 2) == Some(&'A')
                && content.get(i - 3 - cols * 3) == Some(&'S')
            {
                count += 1;
            }

            // Increment the counter safely
            xmas_count.fetch_add(count, Ordering::Relaxed);
        }
    });

    // Return the total count
    xmas_count.load(Ordering::Relaxed)
}

fn part_2(content: String) -> u32 {
    // Get the number of rows and columns
    let rows = content.lines().count();
    let cols = content
        .lines()
        .next()
        .map(|line| line.len())
        .expect("could not count columns");

    // Print size
    println!("Map size: ({}x{})", rows, cols);

    // Replace newline characters and convert to Vec<char> for indexed access
    let content: Vec<char> = content.replace('\n', "").chars().collect();

    // Atomic counter for thread-safe counting
    let xmas_count = AtomicU32::new(0);

    // Process characters in parallel
    (0..content.len())
        .into_par_iter()
        .filter(|&i| {
            // Check if the character is 'A' and within boundaries
            content[i] == 'A'
                && i > cols
                && i % cols > 0
                && i % cols < cols - 1
                && i < (rows - 1) * cols
        })
        .for_each(|i| {
            let top_left = content[i - 1 - cols];
            let top_right = content[i + 1 - cols];
            let bottom_left = content[i - 1 + cols];
            let bottom_right = content[i + 1 + cols];

            // Check combinations that forms the valid MAS cross 
            if (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'M')
                || (top_left == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'M'
                    && bottom_left == 'S'
                    && bottom_right == 'M')
            {
                xmas_count.fetch_add(1, Ordering::SeqCst);
            }
        });

    // Return the total count
    xmas_count.load(Ordering::Relaxed)
}

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).expect("could not read content");
    // let xmas_count_part_1 = part_1(content.clone());

    let xmas_count_part_2 = part_2(content);

    //println!("Part 1: {}", xmas_count_part_1);
    println!("Part 2: {}", xmas_count_part_2);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::part_1;

    // Right
    #[test]
    fn test_count_part_right() {
        let content = String::from(
            "....XM
AS..XM
......
XMAS..
......",
        );
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    // backwards left
    #[test]
    fn test_count_part_left_backwards() {
        let content = String::from(
            "....SA
MX..MX
......
SAMX..
......",
        );
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    // Down
    #[test]
    fn test_count_part_down() {
        let content = String::from(".....X\n.....M\n.....A\n.....S\n......");
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    // UP
    #[test]
    fn test_count_up() {
        let content = String::from(".....S\n.....A\n.....M\n.....X\n......");
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    // Left down
    #[test]
    fn test_count_part_4() {
        let content = String::from(".....X\n....M.\n...A..\n..S...\n......");
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    // Right down
    #[test]
    fn test_count_part_5() {
        let content = String::from("X.....\n.M....\n..A...\n...S..\n......");
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 1);
    }

    #[test]
    fn test_count_final() {
        let content = fs::read_to_string("./test_1.txt").expect("could not read file");
        let xmas_count = part_1(content);
        assert_eq!(xmas_count, 18);
    }
}
