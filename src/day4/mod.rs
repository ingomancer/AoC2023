use std::collections::{HashMap, HashSet};

pub fn run(input: String) -> (String, String) {
    let mut sum = 0;
    let mut cards = HashMap::new();
    for (card, line) in input.lines().enumerate() {
        let card = card + 1;
        *cards.entry(card).or_insert(0) += 1;
        let cardcount = *cards.get(&card).unwrap();
        let mut win_set: HashSet<u32> = HashSet::new();
        let mut points = 1;
        let (winners, yours) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        for winner in winners.split(' ') {
            if !winner.is_empty() {
                win_set.insert(winner.parse().unwrap());
            }
        }
        let mut wins = 0;

        for number in yours.split(' ') {
            if !number.is_empty() && win_set.contains(&number.parse().unwrap()) {
                points *= 2;
                wins += 1;
            }
        }
        if wins > 0 {
            for i in 1..=wins {
                *cards.entry(card + i).or_insert(0) += cardcount;
            }
        }
        sum += points / 2;
    }
    (format!("{sum}"), format!("{}", cards.values().sum::<u32>()))
}
