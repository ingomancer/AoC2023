use std::collections::HashSet;

use num::abs;

pub fn run(input: String) -> (String, String) {
    let mut galaxies = HashSet::new();
    let mut x_offset = 0;
    let mut x_offset_2 = 0;
    let mut lines = input.lines().enumerate();
    let cols = input.lines().next().unwrap().len();
    let mut empty_cols: HashSet<usize> = (0..cols).collect();
    while let Some((x, line)) = lines.next() {
        let mut no_galaxies = true;
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                empty_cols.remove(&y);
                no_galaxies = false;
                galaxies.insert((x + x_offset, y, x + x_offset_2, y));
            }
        }
        if no_galaxies {
            x_offset += 1;
            x_offset_2 += 999999;
        }
    }
    let mut counted_galaxies = HashSet::new();
    let mut sum = 0;
    let mut sum2 = 0;
    for galaxy in galaxies.iter().map(|(x, y, x2, y2)| {
        let mut new_y = *y;
        let mut new_y2 = *y;
        for col in empty_cols.iter() {
            if y > col {
                new_y += 1;
                new_y2 += 999999;
            }
        }
        (*x, new_y, *x2, new_y2)
    }) {
        counted_galaxies.insert(galaxy);
        for other_galaxy in galaxies.iter().map(|(x, y, x2, y2)| {
            let mut new_y = *y;
            let mut new_y2 = *y;
            for col in empty_cols.iter() {
                if y > col {
                    new_y += 1;
                    new_y2 += 999999;
                }
            }
            (*x, new_y, *x2, new_y2)
        }) {
            if galaxy != other_galaxy && !counted_galaxies.contains(&other_galaxy) {
                sum += abs(galaxy.0 as isize - other_galaxy.0 as isize)
                    + abs(galaxy.1 as isize - other_galaxy.1 as isize);
                sum2 += abs(galaxy.2 as isize - other_galaxy.2 as isize)
                    + abs(galaxy.3 as isize - other_galaxy.3 as isize);
            }
        }
    }
    (format!("{sum}"), format!("{sum2}"))
}
