use num::integer::lcm;
use std::collections::HashMap;

pub fn run(input: String) -> (String, String) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim_end();
    lines.next();
    let mut network = HashMap::new();
    let mut cur_nodes = vec![];

    for line in lines {
        let (node, paths) = line.split_once(" = (").unwrap();
        let (left, right) = paths.split_once(", ").unwrap();
        let right = right.strip_suffix(")").unwrap();
        if node.ends_with('A') {
            cur_nodes.push(node);
        }
        network.insert(node, (left, right));
    }

    let mut cur_node = "AAA";
    let mut steps = 0;
    let mut next_direction = instructions.chars();
    loop {
        if cur_node == "ZZZ" {
            break;
        }
        let direction = next_direction.next().unwrap_or_else(|| {
            next_direction = instructions.chars();
            next_direction.next().unwrap()
        });
        let reachable_nodes = network.get(cur_node).unwrap();
        if direction == 'L' {
            cur_node = reachable_nodes.0;
        } else {
            cur_node = reachable_nodes.1;
        }
        steps += 1;
    }

    let mut stepcounts: Vec<u64> = vec![];
    for cur_node in cur_nodes {
        let mut cur_node = cur_node;
        let mut steps = 0;
        let mut next_direction = instructions.chars();
        loop {
            if cur_node.ends_with('Z') {
                break;
            }
            let direction = next_direction.next().unwrap_or_else(|| {
                next_direction = instructions.chars();
                next_direction.next().unwrap()
            });
            let reachable_nodes = network.get(cur_node).unwrap();
            if direction == 'L' {
                cur_node = reachable_nodes.0;
            } else {
                cur_node = reachable_nodes.1;
            }
            steps += 1;
        }
        stepcounts.push(steps);
    }
    println!("{:?}", stepcounts);
    (
        format!("{steps}"),
        format!("{}", stepcounts.iter().fold(1, |acc, e| lcm(acc, *e))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_repeat() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        assert_eq!(run(input), ("2".to_owned(), "2".to_owned()));
    }

    #[test]
    fn test_repeat() {
        let input = "LLR 

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        assert_eq!(run(input), ("6".to_owned(), "2".to_owned()));
    }
}
