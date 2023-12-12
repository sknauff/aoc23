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

    // dbg!(&result);

    result.iter().sum::<i64>().to_string()
}

fn parse_line(input: &str) -> i64 {
    let mut parse_line: Vec<Vec<i64>> = vec![input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()];

    // dbg!(&parse_line[0][0]);

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
            dbg!(number_counts);
            break;
        }

        let mut new_line: Vec<i64> = Vec::new();

        for n in 1..last_line.len() {
            new_line.push(last_line[n] - last_line[n - 1])
        }
        // dbg!(&new_line);
        parse_line.push(new_line.to_owned());
    }

    for n in 1..(parse_line.len()) {
        let current_line = parse_line.len() - n;
        let next_line = parse_line.len() - n - 1;

        // dbg!(&parse_line);
        let new_value =
            parse_line[next_line].last().unwrap() + parse_line[current_line].last().unwrap();
        parse_line[next_line].push(new_value);
    }
    parse_line[0].last().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line1() {
        let result = part1(
            "
        0 3 6 9 12 15
        ",
        );
        assert_eq!(result, "18".to_string());
    }

    #[test]
    fn line2() {
        let result = part1(
            "
        1 3 6 10 15 21
        ",
        );
        assert_eq!(result, "28".to_string());
    }

    #[test]
    fn line3() {
        let result = part1(
            "
        10 13 16 21 30 45
        ",
        );
        assert_eq!(result, "68".to_string());
    }

    // #[test]
    // fn negative1() {
    //     let result = part1(
    //         "
    //     -1 -7 -11 4 83 326 938 2325 5274 11283 23158 46078 89456 170104 317453 581895 1047713 1852557 3216019 5480568 9168937
    //     ",
    //     );
    //     assert_eq!(result, "2325".to_string());
    // }

    #[test]
    fn all() {
        let result = part1(
            "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ",
        );
        assert_eq!(result, "114".to_string());
    }
}
