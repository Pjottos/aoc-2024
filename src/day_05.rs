use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug};

pub fn part_1(input: &str) -> impl Debug {
    let input2 = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    let mut before_rules = BTreeMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        before_rules.entry(a).or_insert(Vec::new()).push(b);
    }

    let mut result = 0;
    for line in lines {
        let update = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let is_invalid = update.iter().enumerate().any(|(i, n)| {
            update[i + 1..]
                .iter()
                .filter_map(|n2| before_rules.get(n2))
                .any(|rules| rules.contains(n))
        });
        if !is_invalid {
            result += update[update.len() / 2];
        }
    }

    result
}
pub fn part_2(input: &str) -> impl Debug {
    let mut before_rules = BTreeMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        before_rules.entry(a).or_insert(Vec::new()).push(b);
    }

    let mut result = 0;
    for line in lines {
        let mut update = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let is_invalid = update.iter().enumerate().any(|(i, n)| {
            update[i + 1..]
                .iter()
                .filter_map(|n2| before_rules.get(n2))
                .any(|rules| rules.contains(n))
        });
        if is_invalid {
            update.sort_by(|a, b| {
                let a_rules = before_rules.get(a);
                let b_rules = before_rules.get(b);
                match (a_rules, b_rules) {
                    (None, None) => Ordering::Equal,
                    (None, Some(_)) => Ordering::Greater,
                    (Some(_), None) => Ordering::Less,
                    (Some(a_rules), Some(b_rules)) => {
                        match (a_rules.contains(b), b_rules.contains(a)) {
                            (false, false) => Ordering::Equal,
                            (false, true) => Ordering::Greater,
                            (true, false) => Ordering::Less,
                            (true, true) => panic!("unsolvable rules"),
                        }
                    }
                }
            });

            result += update[update.len() / 2];
        }
    }

    result
}
