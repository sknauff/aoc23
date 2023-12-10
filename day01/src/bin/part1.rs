fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}",output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        sum += number_from_line(line)
    }

    sum.to_string()
}

fn number_from_line(input: &str) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for i in input.chars() {
        if i.is_numeric() {
            v.push(i.to_digit(10).unwrap());
        }
    }
    v.first().unwrap() * 10 + v.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142".to_string());
    }
}