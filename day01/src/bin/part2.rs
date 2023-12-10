use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{:}",output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        sum += number_from_line(line.to_string())
    }

    sum.to_string()
}

fn number_from_line(mut input: String) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    let spelled_out_numbers = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine")
    ]);

    for (key, value) in  spelled_out_numbers{
        input = input.replace(key, &value.to_string());
    }

    dbg!(input.to_owned());

    for i in input.chars() {
        if i.is_numeric() {
            v.push(i.to_digit(10).unwrap());
        }
    }
    let r = v.first().unwrap() * 10 + v.last().unwrap();
    dbg!(r);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, "281".to_string());
    }
}