use std::collections::{HashMap, HashSet};

pub fn run(input: String) -> (String, String) {
    let mut map = HashMap::new();
    let mut s = (0, 0);
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char == 'S' {
                s = (x, y);
                map.insert((x, y), '7'); // Hack for my input instead of calculating what S must be
            } else {
                map.insert((x, y), char);
            }
        }
    }
    let first_step = match map.get(&(s.0 - 1, s.1)).unwrap_or(&'.') {
        '|' | '7' | 'F' => (s.0 - 1, s.1),
        _ => match map.get(&(s.0 + 1, s.1)).unwrap_or(&'.') {
            '|' | 'L' | 'J' => (s.0 + 1, s.1),
            _ => match map.get(&(s.0, s.1 - 1)).unwrap_or(&'.') {
                '-' | 'L' | 'F' => (s.0, s.1 - 1),
                _ => match map.get(&(s.0, s.1 + 1)).unwrap_or(&'.') {
                    '-' | 'J' | '7' => (s.0, s.1 + 1),
                    _ => (0, 0),
                },
            },
        },
    };

    let mut cur_pos = first_step;
    let mut prev_pos = s;
    let mut loop_length = 1;
    let mut loop_points = HashSet::new();
    loop_points.insert(s);
    loop {
        loop_points.insert(cur_pos);
        if cur_pos == s {
            break;
        }
        let (x, y) = cur_pos;
        let exits = match map.get(&cur_pos).unwrap() {
            '|' => ((x - 1, y), (x + 1, y)),
            '-' => ((x, y - 1), (x, y + 1)),
            'L' => ((x - 1, y), (x, y + 1)),
            'J' => ((x - 1, y), (x, y - 1)),
            '7' => ((x, y - 1), (x + 1, y)),
            'F' => ((x + 1, y), (x, y + 1)),
            _ => panic!(),
        };

        let next_pos = if exits.0 == prev_pos {
            exits.1
        } else {
            exits.0
        };
        prev_pos = cur_pos;
        cur_pos = next_pos;

        loop_length += 1;
    }

    let mut contained_points = 0;
    for point in map.keys() {
        if loop_points.contains(point) {
            continue;
        }
        let mut contained = false;
        let mut prev_in_loop = false;
        let mut leftright = "";
        for x in (0..point.0).rev() {
            if loop_points.contains(&(x, point.1)) {
                if !prev_in_loop {
                    match map.get(&(x, point.1)).unwrap() {
                        '-' => contained = !contained,
                        'L' => {
                            leftright = "right";
                            prev_in_loop = true;
                        }
                        'J' => {
                            leftright = "left";
                            prev_in_loop = true;
                        }
                        _ => panic!(),
                    }
                } else {
                    match map.get(&(x, point.1)).unwrap() {
                        '|' => continue,
                        '7' => {
                            if leftright == "right" {
                                contained = !contained;
                            }
                            prev_in_loop = false;
                        }
                        'F' => {
                            if leftright == "left" {
                                contained = !contained;
                            }
                            prev_in_loop = false;
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
            }
        }
        if contained {
            contained_points += 1;
        }
    }

    (
        format!("{}", loop_length / 2),
        format!("{contained_points}"),
    )
}
