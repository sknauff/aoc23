use std::collections::HashMap;

use num;
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}", output);
}

fn part1(input: &str) -> String {
    let v_lines: Vec<&str> = input
        .trim()
        .lines()
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let intructions: Vec<char> = v_lines[0].chars().collect();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in &v_lines[2..] {
        let s1: Vec<&str> = line.split(" = ").collect();
        let s2: Vec<&str> = s1[1].split(", ").collect();
        nodes
            .entry(s1[0])
            .or_insert((&s2[0][1..], &s2[1][..(&s2[1].len() - 1)]));
    }

    let v_current_positions: Vec<&str> = Vec::from_iter(nodes.keys().cloned())
        .iter()
        .filter(|&&x| x.chars().collect::<Vec<char>>()[2] == 'A')
        .map(|x| x.to_owned())
        .collect();

    let v_counts = v_current_positions
        .par_iter()
        .map(|x| get_iter_count(x, &nodes, &intructions))
        .collect::<Vec<u64>>();

    let mut lcm = 0;
    for i in 1..v_counts.len() {
        match lcm {
            0 => lcm = num::integer::lcm(v_counts[0], v_counts[1]),
            _ => lcm = num::integer::lcm(lcm, v_counts[i]),
        }
    }

    lcm.to_string()
}

fn get_iter_count(
    start: &str,
    nodes: &HashMap<&str, (&str, &str)>,
    intructions: &Vec<char>,
) -> u64 {
    let mut current_position = start;
    let mut count = 0;

    loop {
        match intructions[count % intructions.len()] {
            'L' => current_position = nodes.get(current_position).unwrap().0,
            'R' => current_position = nodes.get(current_position).unwrap().1,
            _ => (),
        }

        count += 1;

        if current_position.chars().collect::<Vec<char>>()[2] == 'Z' {
            break;
        }
    }
    count as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = part1(
            "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ",
        );
        assert_eq!(result, "6".to_string());
    }
}
