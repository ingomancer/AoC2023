use std::cmp::max;

pub fn run(input: String) -> (String, String) {
    let (max_red, max_blue, max_green) = (12, 14, 13);
    let mut sum = 0;
    let mut power = 0;
    for line in input.lines() {
        let (mut seen_red, mut seen_blue, mut seen_green) = (0, 0, 0);
        let (id, cubesets) = parse_line(line);
        for cubeset in cubesets.split(';') {
            for cube in cubeset.split(',') {
                let (num, colour) = cube.strip_prefix(' ').unwrap().split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                match colour {
                    "red" => seen_red = max(num, seen_red),
                    "green" => seen_green = max(num, seen_green),
                    "blue" => seen_blue = max(num, seen_blue),
                    _ => panic!(),
                }
            }
        }
        if seen_red <= max_red && seen_green <= max_green && seen_blue <= max_blue {
            sum += id;
        }
        power += seen_blue * seen_green * seen_red;
    }
    (format!("{sum}"), format!("{power}"))
}

fn parse_line(line: &str) -> (u32, &str) {
    let (id, cubes) = line.split_once(':').unwrap();
    let id: u32 = id.split_once(' ').unwrap().1.parse().unwrap();
    (id, cubes)
}
