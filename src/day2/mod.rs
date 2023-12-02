pub fn run(input: String) -> (String, String) {
    for line in input.lines() {
        let (id, cubesets) = parse_line(line);
    }
    ("".to_owned(), "".to_owned())
}

fn parse_line(line: &str) -> (u32, &str) {
    let (id, cubes) = line.split_once(":").unwrap();
    let id: u32 = id.parse().unwrap();
    (id, cubes)
}
