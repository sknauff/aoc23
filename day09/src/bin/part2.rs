use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}", output);
}

fn part1(input: &str) -> String {
    let v_lines = input
        .trim()
        .lines()
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let result: Vec<i64> = v_lines.iter().map(|x| parse_line(x)).collect();

    result.iter().sum::<i64>().to_string()
}

fn parse_line(input: &str) -> i64 {
    let mut parse_line: Vec<Vec<i64>> = vec![input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()];

    loop {
        let last_line = parse_line.last().unwrap();

        let mut number_counts: HashMap<i64, i64> = HashMap::new();
        for num in last_line {
            number_counts
                .entry(num.to_owned())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if last_line.iter().sum::<i64>() == 0 && number_counts.len() == 1 {
            break;
        }

        let mut new_line: Vec<i64> = Vec::new();

        for n in 1..last_line.len() {
            new_line.push(last_line[n] - last_line[n - 1])
        }
        parse_line.push(new_line.to_owned());
    }

    for n in 1..(parse_line.len()) {
        let current_line = parse_line.len() - n;
        let next_line = parse_line.len() - n - 1;

        let new_value = parse_line[next_line][0] - parse_line[current_line][0];

        let mut new_next_line: Vec<i64> = vec![new_value];
        new_next_line.append(&mut parse_line[next_line]);

        parse_line[next_line] = new_next_line;
    }

    parse_line[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let result = part1(
            "
            10  13  16  21  30  45
        ",
        );
        assert_eq!(result, "5".to_string());
    }
}
