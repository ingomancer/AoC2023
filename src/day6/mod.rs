use std::iter::zip;


pub fn run(input: String) -> (String, String) {
    let mut prod = 1;
    let (line1, line2) = input.split_once('\n').unwrap();
    let mut p2time = String::new();
    let mut p2dist = String::new();
    for (time, distance) in zip(line1.split_ascii_whitespace().skip(1), line2.split_ascii_whitespace().skip(1)) {
        p2time += time;
        p2dist += distance;
        let time: u32 = time.parse().unwrap();
        let distance: u32 = distance.parse().unwrap();
        let mut wins = 0;
        for speed in 0..time {
            if speed * (time - speed) > distance {
                wins = (time-1) - 2*(speed-1);
                break;
            }
        }
        prod *= wins;
    }
    let p2time: u64 = p2time.parse().unwrap();
    let p2dist: u64 = p2dist.parse().unwrap();
    let mut wins = 0;
    for speed in 0..p2time {
        if speed * (p2time - speed) > p2dist {
            wins = (p2time-1) - 2*(speed-1);
            break;
        }
    }
    (format!("{prod}"),format!("{wins}"))
}