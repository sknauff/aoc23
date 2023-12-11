use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}", output);
}

fn part1(input: &str) -> String {
    let _faces_in_order = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let v_lines = input
        .trim()
        .lines()
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let v_hands: Vec<Hand> = v_lines
        .par_iter()
        .map(|x| hand_from_str(x.to_owned()))
        .collect();

    dbg!(v_hands);

    input.to_string()
}

fn hand_from_str(input: &str) -> Hand {
    let split: Vec<&str> = input.split_whitespace().collect();
    Hand {
        cards: split[0].chars().collect(),
        bid: split[1].parse().unwrap(),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ",
        );
        assert_eq!(result, "6440".to_string());
    }
}
