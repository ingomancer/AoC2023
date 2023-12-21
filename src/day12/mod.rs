use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::iter::zip;
pub fn run(input: String) -> (String, String) {
    let lines = input.lines().count() as u64;
    let (sum, sum2): (usize, usize) = input
        .par_lines()
        .map(|line| {
            let (grid, griddle) = line.split_once(' ').unwrap();
            let grid2 = format!("{}?{}?{}?{}?{}", grid, grid, grid, grid, grid);
            let griddle2 = format!(
                "{},{},{},{},{}",
                griddle, griddle, griddle, griddle, griddle
            );
            let griddle: Vec<&str> = griddle.split(',').collect();
            let griddle2: Vec<&str> = griddle2.split(',').collect();

            (
                count_possible_arangements(grid.to_string(), &griddle),
                count_possible_arangements(grid2, &griddle2),
            )
        })
        .progress_count(lines)
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    (format!("{}", sum), format!("{}", sum2))
}

fn count_possible_arangements(grid: String, griddle: &Vec<&str>) -> usize {
    let mut arrangements = 0;
    let mut wildcards = false;
    let parts: Vec<&str> = grid.split('.').filter(|x| !x.is_empty()).collect();
    for (part, riddle) in zip(parts.iter(), griddle) {
        if !part.contains('?') {
            if part.len() != riddle.parse().unwrap() {
                return 0;
            }
        } else {
            break;
        }
    }
    for (i, char) in grid.chars().enumerate() {
        if char == '?' {
            wildcards = true;
            let prev_groups: Vec<&str> = grid[..i + 1].split('.').filter(|x| *x != "").collect();

            println!("{prev_groups:?}");

            let left = format!("{}#{}", &grid[..i], &grid[i + 1..]);
            let right = format!("{}.{}", &grid[..i], &grid[i + 1..]);

            arrangements += count_possible_arangements(left, griddle);
            arrangements += count_possible_arangements(right, griddle);
            break;
        }
    }
    if !wildcards {
        let mut parts = vec![];
        let mut section = String::new();
        for char in grid.chars() {
            if char == '.' {
                if !section.is_empty() {
                    parts.push(section);
                    section = String::new();
                }
            } else {
                section.push(char);
            }
        }
        if !section.is_empty() {
            parts.push(section);
        }
        if parts.len() == griddle.len() {
            let mut all_match = true;
            for (part, riddle) in zip(parts, griddle) {
                if part.len() != riddle.parse().unwrap() {
                    all_match = false;
                }
            }
            if all_match && arrangements == 0 {
                return 1;
            }
        }
    }
    arrangements
}
