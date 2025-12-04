use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, anyhow};
use nom::{
    bytes::complete::tag, character::complete::alpha0, error::Error, multi::separated_list1,
    sequence::separated_pair,
};

type SolverInput = HashMap<u32, Node>;

const START_ID: u32 = compute_identifier(b"start");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small,
    Big,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node {
    kind: Cave,
    connections: Vec<u32>,
}

#[derive(Clone)]
struct Path<'a> {
    passed: HashSet<&'a Node>,
    double_passed_small: Option<&'a Node>,
    last_identifier: u32,
}

const fn compute_identifier(name: &[u8]) -> u32 {
    let mut value = 0;
    let mut i = 0;
    while i < name.len() {
        value *= (b'z' - b'A') as u32;
        value += (name[i] - b'A') as u32;
        i += 1;
    }
    value
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let make_empty_node = |name: &[u8]| {
        Some(match name.len() {
            3 => Node {
                kind: Cave::End,
                connections: vec![],
            },
            5 => Node {
                kind: Cave::Start,
                connections: vec![],
            },
            _ => {
                if name[0] >= b'A' && name[0] <= b'Z' {
                    Node {
                        kind: Cave::Big,
                        connections: vec![],
                    }
                } else if name[0] >= b'a' && name[0] <= b'z' {
                    Node {
                        kind: Cave::Small,
                        connections: vec![],
                    }
                } else {
                    return None;
                }
            }
        })
    };

    let connections = separated_list1::<_, _, _, Error<_>, _, _>(
        tag(b"\n"),
        separated_pair(alpha0, tag(b"-"), alpha0),
    )(file)
    .map_err(|_| anyhow!("Failed parsing digits"))?
    .1;
    let mut nodes = HashMap::new();
    for conn in connections {
        let side0 = compute_identifier(conn.0);
        let side1 = compute_identifier(conn.1);
        let node0 = make_empty_node(conn.0).context("Invalid side in connection")?;
        let node1 = make_empty_node(conn.1).context("Invalid side in connection")?;
        nodes.entry(side0).or_insert(node0).connections.push(side1);
        nodes.entry(side1).or_insert(node1).connections.push(side0);
    }
    Ok(nodes)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut found_paths = 0;
    let mut paths: Vec<Path> = vec![];
    {
        let mut first_path = Path {
            passed: HashSet::new(),
            last_identifier: START_ID,
            double_passed_small: None,
        };
        first_path.passed.insert(&input[&START_ID]);
        paths.push(first_path);
    }

    while let Some(path) = paths.pop() {
        for other_id in input[&path.last_identifier].connections.iter() {
            let other_node = &input[other_id];
            if other_node.kind == Cave::End {
                // finished path
                found_paths += 1;
                continue;
            }
            if other_node.kind == Cave::Start {
                // can't go back to start
                continue;
            }
            if other_node.kind == Cave::Small && path.passed.contains(other_node) {
                // can't enter a small cave 2 times
                continue;
            }

            let mut new_path = path.clone();
            new_path.passed.insert(other_node);
            new_path.last_identifier = *other_id;
            paths.push(new_path);
        }
    }
    found_paths
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut found_paths = 0;
    let mut paths: Vec<Path> = vec![];
    {
        let mut first_path = Path {
            passed: HashSet::new(),
            last_identifier: START_ID,
            double_passed_small: None,
        };
        first_path.passed.insert(&input[&START_ID]);
        paths.push(first_path);
    }

    while let Some(path) = paths.pop() {
        for other_id in input[&path.last_identifier].connections.iter() {
            let other_node = &input[other_id];
            if other_node.kind == Cave::End {
                // finished path
                found_paths += 1;
                continue;
            }
            if other_node.kind == Cave::Start {
                // can't go back to start
                continue;
            }
            if other_node.kind == Cave::Small && path.passed.contains(other_node) {
                if path.double_passed_small.is_none() {
                    // double pass
                    let mut new_path = path.clone();
                    new_path.double_passed_small = Some(other_node);
                    new_path.last_identifier = *other_id;
                    paths.push(new_path);
                }
                // else can't enter a small cave 2 times
                continue;
            }

            let mut new_path = path.clone();
            new_path.passed.insert(other_node);
            new_path.last_identifier = *other_id;
            paths.push(new_path);
        }
    }
    found_paths
}

#[cfg(test)]
mod test {
    use nom::AsBytes;

    use super::*;

    const EXAMPLE_SMALL: &[u8] = concat!(
        "start-A\n",
        "start-b\n",
        "A-c\n",
        "A-b\n",
        "b-d\n",
        "A-end\n",
        "b-end\n",
    )
    .as_bytes();

    #[test]
    fn parse_example_small() {
        let parsed = rules::parse_expect!(EXAMPLE_SMALL, "small example");
        {
            let from = parsed
                .get(&compute_identifier(b"start".as_bytes()))
                .expect("No paths originating from start");
            assert!(matches!(from.kind, Cave::Start));
            assert!(
                from.connections
                    .contains(&compute_identifier(b"A".as_bytes()))
            );
            assert!(
                from.connections
                    .contains(&compute_identifier(b"b".as_bytes()))
            );
        }
        {
            let from = parsed
                .get(&compute_identifier(b"end".as_bytes()))
                .expect("No paths originating from A");
            assert!(matches!(from.kind, Cave::End));
            assert!(
                from.connections
                    .contains(&compute_identifier(b"A".as_bytes()))
            );
            assert!(
                from.connections
                    .contains(&compute_identifier(b"b".as_bytes()))
            );
        }
        {
            let from = parsed
                .get(&compute_identifier(b"A".as_bytes()))
                .expect("No paths originating from A");
            assert!(matches!(from.kind, Cave::Big));
            assert!(
                from.connections
                    .contains(&compute_identifier(b"c".as_bytes()))
            );
            assert!(
                from.connections
                    .contains(&compute_identifier(b"b".as_bytes()))
            );
            assert!(
                from.connections
                    .contains(&compute_identifier(b"start".as_bytes()))
            );
            assert!(
                from.connections
                    .contains(&compute_identifier(b"end".as_bytes()))
            );
        }
    }
}
