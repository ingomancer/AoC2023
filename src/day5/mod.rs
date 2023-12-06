use std::cmp::min;

use rayon::prelude::*;
#[derive(Debug)]
struct Mapping {
    dest: u64,
    source: u64,
    len: u64,
}

pub fn run(input: String) -> (String, String) {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let mut seeds: Vec<u64> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let seeds2 = seeds.clone();
    let seed_pairs = seeds2.chunks(2);

    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut cur_mapping = vec![];
    lines.next();
    lines.next();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            lines.next();
            mappings.push(cur_mapping);
            cur_mapping = vec![];
            continue;
        }
        let mut split = line.split(' ');
        cur_mapping.push(Mapping {
            dest: split.next().unwrap().parse().unwrap(),
            source: split.next().unwrap().parse().unwrap(),
            len: split.next().unwrap().parse().unwrap(),
        });
    }

    mappings.push(cur_mapping);

    for seed in seeds.iter_mut() {
        for map_type in &mappings {
            for map in map_type {
                if *seed >= map.source && *seed < (map.source + map.len) {
                    *seed = map.dest + (*seed - map.source);
                    break;
                }
            }
        }
    }
    let mut min_seed = u64::MAX;
    for slice in seed_pairs {
        min_seed = min(
            min_seed,
            (slice[0]..slice[0] + slice[1])
                .into_par_iter()
                .map(|mut seed| {
                    for map_type in &mappings {
                        for map in map_type {
                            if seed >= map.source && seed < (map.source + map.len) {
                                seed = map.dest + (seed - map.source);
                                break;
                            }
                        }
                    }
                    seed
                })
                .min()
                .unwrap(),
        );
    }
    (
        format!("{}", seeds.iter().min().unwrap()),
        format!("{}", min_seed),
    )
}
