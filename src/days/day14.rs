use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
    sequence::separated_pair,
};

// hashmap of first sign to (hashmap of second sign to (count of occurences of sign pair))
type PairMap = HashMap<u8, HashMap<u8, u32>>;
// hashmap of sign pair to sign to insert into the middle
type InsertionRules = HashMap<[u8; 2], u8>;
// polymer template, insertion rules, the first sign in starting template
type SolverInput = (PairMap, InsertionRules, u8);

pub fn parse_input(file: &[u8]) -> anyhow::Result<SolverInput> {
    let insertion_rule_parser = separated_pair(alpha1, tag(b" -> "), alpha1);
    let insertion_rules_parser = separated_list1(tag(b"\n"), insertion_rule_parser);

    let (template_parsed, rules_parsed) =
        separated_pair::<_, _, _, _, nom::error::Error<_>, _, _, _>(
            alpha1,
            tag(b"\n\n"),
            insertion_rules_parser,
        )(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing input"))?
        .1;

    let mut template: PairMap = HashMap::new();
    for pair in template_parsed.windows(2) {
        *template
            .entry(pair[0])
            .or_default()
            .entry(pair[1])
            .or_default() += 1;
    }

    let mut rules: InsertionRules = HashMap::new();
    for (pair, inserted) in rules_parsed {
        let pair = [pair[0], pair[1]];
        rules.insert(pair, inserted[0]);
    }

    Ok((template, rules, template_parsed[0]))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut pairs = input.0.clone();
    let rules = &input.1;
    let mut to_add = vec![];
    for _ in 0..10 {
        for (sign1, hm) in pairs.iter_mut() {
            for (sign2, count) in hm.iter_mut() {
                let pair = [*sign1, *sign2];
                if rules.contains_key(&pair) {
                    to_add.push((pair, *count));
                }
            }
        }
        // first remove
        for ([first, second], _) in to_add.iter() {
            // no need to check if they're present
            pairs.get_mut(first).and_then(|hm| hm.remove(second));
        }
        // then add
        for &([first, second], count) in to_add.iter() {
            let middle = rules[&[first, second]];
            *pairs.entry(first).or_default().entry(middle).or_default() += count;
            *pairs.entry(middle).or_default().entry(second).or_default() += count;
        }
        to_add.clear();
    }

    let mut counts: HashMap<u8, u32> = HashMap::new();
    for (_, hm) in pairs.iter() {
        for (sign, count) in hm.iter() {
            *counts.entry(*sign).or_default() += *count;
        }
    }
    *counts.entry(input.2).or_default() += 1;

    let most_common = *counts.values().max().unwrap();
    let least_common = *counts.values().min().unwrap();
    most_common - least_common
}
