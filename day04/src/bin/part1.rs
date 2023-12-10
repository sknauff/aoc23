fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}",output);
}

fn part1(input: &str) -> String {
    let mut v_cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        if line.trim().len() > 0 {
            let split1 = line.trim().split(":").collect::<Vec<&str>>();
            let split2 = split1[1].split("|").collect::<Vec<&str>>();
           
            v_cards.push(Card{
                // id: split1[0].split(" ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap(),
                winning_numbers: split2[0].trim().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<u16>().unwrap()).collect(),
                numbers_you_have: split2[1].trim().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<u16>().unwrap()).collect(),
            })
        }
    };

    let answer: u64 = v_cards.iter().map(|x| {
        match x.get_number_of_wins() {
            0 => 0 as u64,
            1 => 1 as u64,
            x => 2_i32.pow((x - 1) as u32) as u64
        }
    }).sum();
    format!("{answer}").to_string()
}

#[derive(Debug)]
struct Card {
    // id: u64,
    winning_numbers: Vec<u16>,
    numbers_you_have: Vec<u16>,
}

impl Card {
    fn get_number_of_wins(&self) -> u16 {
        let tmp = self.numbers_you_have
            .iter()
            .map(|x| if self.winning_numbers.contains(x) {1} else {0}).collect::<Vec<u16>>().iter().sum();
        tmp 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let result = part1("
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ");
        assert_eq!(result, "13".to_string());
    }

    #[test]
    fn card1(){
        let result = part1("
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        ");
        assert_eq!(result, "8".to_string());
    }

    #[test]
    fn card2() {
        let result = part1("
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        ");
        assert_eq!(result, "2".to_string());
    }


    #[test]
    fn card3() {
        let result = part1("
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        ");
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn card4() {
        let result = part1("
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        ");
        assert_eq!(result, "1".to_string());
    }

    #[test]
    fn card5() {
        let result = part1("
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        ");
        assert_eq!(result, "0".to_string());
    }


    #[test]
    fn card6() {
        let result = part1("
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ");
        assert_eq!(result, "0".to_string());
    }

}