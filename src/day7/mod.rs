use std::{cmp::Ordering, collections::HashMap, iter::zip};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Type {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    hand_type: Type,
    hand_type_2: Type,
}

pub fn run(input: String) -> (String, String) {
    let mut card_values = HashMap::new();
    card_values.insert('T', 10);
    card_values.insert('J', 11);
    card_values.insert('Q', 12);
    card_values.insert('K', 13);
    card_values.insert('A', 14);

    let mut all_hands = vec![];
    for line in input.lines() {
        let mut hand = Hand {
            cards: vec![],
            hand_type: Type::None,
            hand_type_2: Type::None,
        };
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let mut card_counter = HashMap::new();
        for card in cards.chars() {
            *card_counter.entry(card).or_insert(0u32) += 1;
            if let Some(parsed_card) = card.to_digit(10) {
                hand.cards.push(parsed_card);
            } else {
                hand.cards.push(*card_values.get(&card).unwrap());
            }
        }
        match card_counter.keys().count() {
            1 => hand.hand_type = Type::FiveOfAKind,
            2 => match card_counter.iter().next().unwrap().1 {
                1 | 4 => hand.hand_type = Type::FourOfAKind,
                2 | 3 => hand.hand_type = Type::FullHouse,
                _ => panic!("No other way to have only two types"),
            },
            3 => {
                if !card_counter.values().any(|&val| val == 2) {
                    hand.hand_type = Type::ThreeOfAKind;
                } else {
                    hand.hand_type = Type::TwoPair;
                }
            }
            4 => hand.hand_type = Type::OnePair,
            5 => hand.hand_type = Type::HighCard,
            _ => {
                panic!("can't have more than five different types of cards");
            }
        }
        let joker_count = *card_counter.get(&'J').unwrap_or(&0);
        match card_counter.keys().count() {
            1 => hand.hand_type_2 = Type::FiveOfAKind,
            2 => {
                if joker_count != 0 {
                    hand.hand_type_2 = Type::FiveOfAKind
                } else {
                    match card_counter.iter().next().unwrap().1 {
                        1 | 4 => hand.hand_type_2 = Type::FourOfAKind,
                        2 | 3 => hand.hand_type_2 = Type::FullHouse,
                        _ => panic!("No other way to have only two types"),
                    }
                }
            }
            3 => {
                if !card_counter.values().any(|&val| val == 2) {
                    if joker_count != 0 {
                        hand.hand_type_2 = Type::FourOfAKind;
                    } else {
                        hand.hand_type_2 = Type::ThreeOfAKind;
                    }
                } else if joker_count == 1 {
                    hand.hand_type_2 = Type::FullHouse;
                } else if joker_count == 2 {
                    hand.hand_type_2 = Type::FourOfAKind;
                } else {
                    hand.hand_type_2 = Type::TwoPair;
                }
            }
            4 => {
                if joker_count != 0 {
                    hand.hand_type_2 = Type::ThreeOfAKind;
                } else {
                    hand.hand_type_2 = Type::OnePair;
                }
            }
            5 => {
                if joker_count != 0 {
                    hand.hand_type_2 = Type::OnePair;
                } else {
                    hand.hand_type_2 = Type::HighCard;
                }
            }
            _ => {
                panic!("can't have more than five different types of cards");
            }
        }
        all_hands.push((hand, bid));
    }

    all_hands.sort_by(|a, b| match (a.0.hand_type).cmp(&b.0.hand_type) {
        Ordering::Greater | Ordering::Less => (a.0.hand_type).cmp(&b.0.hand_type),
        Ordering::Equal => {
            for (card_a, card_b) in zip(a.0.cards.as_slice(), b.0.cards.as_slice()) {
                match card_a.cmp(card_b) {
                    Ordering::Greater | Ordering::Less => return card_a.cmp(card_b),
                    Ordering::Equal => continue,
                }
            }
            Ordering::Equal
        }
    });
    let mut winnings = 0;
    for (rank, hand) in all_hands.iter().enumerate() {
        winnings += (rank + 1) * hand.1;
    }
    all_hands.sort_by(|a, b| match (a.0.hand_type_2).cmp(&b.0.hand_type_2) {
        Ordering::Greater | Ordering::Less => (a.0.hand_type_2).cmp(&b.0.hand_type_2),
        Ordering::Equal => {
            for (card_a, card_b) in zip(a.0.cards.as_slice(), b.0.cards.as_slice()) {
                let mut a = *card_a;
                let mut b = *card_b;
                if a == 11 {
                    a = 1;
                }
                if b == 11 {
                    b = 1;
                }
                match a.cmp(&b) {
                    Ordering::Greater | Ordering::Less => return a.cmp(&b),
                    Ordering::Equal => continue,
                }
            }
            Ordering::Equal
        }
    });
    let mut winnings2 = 0;
    for (rank, hand) in all_hands.iter().enumerate() {
        winnings2 += (rank + 1) * hand.1;
    }
    (format!("{winnings}"), format!("{winnings2}"))
}
