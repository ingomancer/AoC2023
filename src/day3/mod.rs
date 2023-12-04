use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
struct PartNum {
    num: u32,
    i: usize,
    start_j: usize,
    end_j: usize,
}

pub fn run(input: String) -> (String, String) {
    let mut partNums = vec![];
    let mut symbols = vec![];
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let mut num = String::new();
        for (j, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    if !num.is_empty() {
                        partNums.push(PartNum {
                            num: num.parse().unwrap(),
                            i,
                            start_j: j - num.len(),
                            end_j: j - 1,
                        });
                        num = String::new();
                    }
                }
                '0'..='9' => num.push(char),
                _ => {
                    if !num.is_empty() {
                        partNums.push(PartNum {
                            num: num.parse().unwrap(),
                            i,
                            start_j: j - num.len(),
                            end_j: j - 1,
                        });
                        num = String::new();
                    }
                    symbols.push((i, j, char));
                }
            }
        }
        if !num.is_empty() {
            partNums.push(PartNum {
                num: num.parse().unwrap(),
                i,
                start_j: line.len() - num.len(),
                end_j: line.len() - 1,
            });
            num = String::new();
        }
    }
    let mut gears = HashMap::new();

    for part in partNums {
        let mut done = false;
        for (i, j, char) in symbols.as_slice() {
            if let -1..=1 = part.i as isize - *i as isize {
                let start = part.start_j as isize - 1;
                let end = part.end_j + 1;
                if j <= &end && *j as isize >= start {
                    if '*' == *char {
                        gears.entry((i, j)).or_insert(vec![]).push(part.num);
                    }
                    if !done {
                        sum += part.num;
                        done = true;
                    }
                }
            }
        }
    }
    let mut gearsum = 0;
    for gear in gears.values() {
        if gear.len() == 2 {
            gearsum += gear[0] * gear[1];
        }
    }
    (format!("{sum}"), format!("{gearsum}"))
}
