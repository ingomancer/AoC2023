use std::iter::zip;

pub fn run(input: String) -> (String, String) {
    let mut sum = 0;
    for line in input.lines() {
        let (grid, griddle) = line.split_once(' ').unwrap();
        let griddle: Vec<&str> = griddle.split(',').collect();
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
        sum += count_possible_arangements(&parts, &griddle);
    }
    (format!("{sum}"), format!(""))
}

fn count_possible_arangements(parts: &Vec<String>, griddle: &Vec<&str>) -> usize {
    let mut arrangements = 0;
    for (i, part) in parts.iter().enumerate() {
        for (j, char) in part.chars().enumerate() {
            if char == '?' {
                if part.len() == 1 {
                    arrangements += count_possible_arangements(
                        &[&parts[..i], &parts[i + 1..]].concat(),
                        griddle,
                    );
                    arrangements += count_possible_arangements(
                        &[&parts[..i], &["#".to_owned()], &parts[i + 1..]].concat(),
                        griddle,
                    );
                } else {
                    let left = [part[..j].to_string(), part[j + 1..].to_string()];
                    let right = format!("{}#{}", part[..j].to_string(), part[j + 1..].to_string());

                    arrangements += count_possible_arangements(
                        &[&parts[..i], &left[..], &parts[i + 1..]].concat(),
                        griddle,
                    );
                    arrangements += count_possible_arangements(
                        &[&parts[..i], &[right][..], &parts[i + 1..]].concat(),
                        griddle,
                    );
                }
            }
        }
    }
    if parts.len() == griddle.len() {
        for (part, riddle) in zip(parts, griddle) {
            if !(part.len() == riddle.parse().unwrap()) {
                break;
            }
            return 1 + arrangements;
        }
    }
    arrangements
}
