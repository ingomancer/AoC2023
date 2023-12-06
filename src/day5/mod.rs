use rayon::prelude::*;
#[derive(Debug)]
struct Mapping {
    dest: u64,
    source: u64,
    len: u64
}

pub fn run(input: String) -> (String, String) {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let mut seeds: Vec<u64> = seeds.strip_prefix("seeds: ").unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
    let mut seeds2 = vec![];
    let seed_pairs = seeds.chunks(2);
    for slice in seed_pairs {
        for seed in slice[0]..slice[0]+slice[1] {
            seeds2.push(seed);
        }
    }
    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut cur_mapping = vec![];
    lines.next();
    lines.next();
    loop {
        let line = if let Some(line) = lines.next() {
            line
        } else {
            break;
        };
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
            len: split.next().unwrap().parse().unwrap()
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
    seeds2.par_iter_mut().for_each(|seed| {
        for map_type in &mappings {
            for map in map_type {
                if *seed >= map.source && *seed < (map.source + map.len) {
                    *seed = map.dest + (*seed - map.source);
                    break;
                }
            }
        }
    });
    (format!("{}", seeds.iter().min().unwrap()), format!("{}", seeds2.iter().min().unwrap()))
}
