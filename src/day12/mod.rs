use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{collections::HashMap, iter::zip};
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
            let mut memo: HashMap<(String, Vec<String>), usize> = HashMap::new();
            let mut memo2: HashMap<(String, Vec<String>), usize> = HashMap::new();
            (
                count_possible_arangements(grid.to_string(), &griddle, &mut memo),
                count_possible_arangements(grid2, &griddle2, &mut memo2),
            )
        })
        .progress_count(lines)
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    (format!("{}", sum), format!("{}", sum2))
}

fn count_possible_arangements(
    grid: String,
    griddle: &Vec<&str>,
    memo: &mut HashMap<(String, Vec<String>), usize>,
) -> usize {
    let mut arrangements = 0;
    let mut wildcards = false;
    if let Some(memcount) = memo.get(&(
        grid.clone(),
        griddle.iter().map(|x| x.to_string()).collect(),
    )) {
        return *memcount;
    }
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
            let prev_groups: Vec<&str> =
                grid[..i + 1].split('.').filter(|x| !x.is_empty()).collect();
            let group_count = prev_groups.len() - 1;
            if group_count > griddle.len() {
                return 0;
            }
            let left = format!(
                "{}{}",
                prev_groups[group_count].replacen('?', "#", 1),
                &grid[i + 1..]
            );
            let right = format!(
                "{}{}",
                prev_groups[group_count].replacen('?', ".", 1),
                &grid[i + 1..]
            );
            arrangements +=
                count_possible_arangements(left, &griddle[group_count..].to_vec(), memo);
            arrangements +=
                count_possible_arangements(right, &griddle[group_count..].to_vec(), memo);
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
                arrangements = 1;
            }
        }
    }
    memo.insert(
        (grid, griddle.iter().map(|x| x.to_string()).collect()),
        arrangements,
    );
    arrangements
}
