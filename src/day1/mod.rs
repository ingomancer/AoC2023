use aho_corasick::AhoCorasick;

pub fn run(input: String) -> (String, String) {
    let mut partonesum = 0;
    let mut parttwosum = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut second = 0;
        for char in line.chars() {
            if let Some(i) = char.to_digit(10) {
                first = 10 * i;
                break;
            }
        }
        for char in line.chars().rev() {
            if let Some(i) = char.to_digit(10) {
                second = i;
                break;
            }
        }
        partonesum += first + second;
    }

    let words = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let backwards_words = &[
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];
    let numbers = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for line in input.lines() {
        let ac = AhoCorasick::new(words).unwrap();
        let new_line = ac.replace_all(line, numbers);
        let mut first = 0;
        let mut second = 0;
        for char in new_line.chars() {
            if let Some(i) = char.to_digit(10) {
                first = 10 * i;
                break;
            }
        }
        let ac = AhoCorasick::new(backwards_words).unwrap();
        let new_line = ac.replace_all(&line.chars().rev().collect::<String>(), numbers);
        for char in new_line.chars() {
            if let Some(i) = char.to_digit(10) {
                second = i;
                break;
            }
        }
        parttwosum += first + second;
    }

    (format!("{}", partonesum), format!("{}", parttwosum))
}
