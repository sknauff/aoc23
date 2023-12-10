fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{:}", output);
}

fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("TODOQUESTION");
        assert_eq!(result, "TODOANSWER".to_string());
    }
}
