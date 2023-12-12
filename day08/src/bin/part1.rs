use std::collections::HashMap;

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

    let mut current_position: &str = "AAA";
    let mut count = 0;

    while current_position != "ZZZ" {
        match intructions[count % intructions.len()] {
            'L' => current_position = nodes.get(current_position).unwrap().0,
            'R' => current_position = nodes.get(current_position).unwrap().1,
            _ => (),
        }

        count += 1;
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = part1(
            "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ",
        );
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn test_b() {
        let result = part1(
            "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)        
        ",
        );
        assert_eq!(result, "6".to_string());
    }
}
