use std::collections::HashMap;

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

    let mut v_hands: Vec<Hand> = v_lines
        .iter()
        .map(|x| hand_from_str(x.to_owned()))
        .collect();

    v_hands.sort_by(|a, b| a.value.cmp(&b.value));

    let mut total = 0;

    for n in 1..=v_hands.len() {
        total += n as u64 * v_hands[n - 1].bid;
    }

    total.to_string()
}

fn hand_from_str(input: &str) -> Hand {
    let split: Vec<&str> = input.split_whitespace().collect();
    let mut return_hand = Hand {
        cards: split[0].chars().collect(),
        bid: split[1].parse().unwrap(),
        value: 0,
    };
    return_hand.value = return_hand.get_hand_value().to_owned();
    return_hand
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    value: u64,
}

impl Hand {
    // fn get_unique_faces(&self) -> Vec<char> {
    //     let mut unique_faces: Vec<char> = Vec::new();
    //     for face in self.cards.to_owned() {
    //         if !unique_faces.contains(&face) {
    //             unique_faces.push(face);
    //         }
    //     }
    //     unique_faces
    // }

    fn get_face_counts(&self) -> HashMap<char, u64> {
        let mut face_counts: HashMap<char, u64> = HashMap::new();
        for face in &self.cards {
            face_counts
                .entry(face.to_owned())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        face_counts
    }

    fn get_hand_type(&self) -> HandType {
        let all_face_counts = self.get_face_counts();
        let n_jacks = match all_face_counts.get(&'J') {
            Some(n) => n.to_owned() as u64,
            None => 0 as u64,
        };

        let mut unique_faces = all_face_counts.clone();
        unique_faces.remove(&'J');

        let mut face_counts = unique_faces.values().cloned().collect::<Vec<u64>>();
        face_counts.sort();

        if n_jacks == 5 {
            HandType::FiveOfAKind
        } else if face_counts[0] + n_jacks == 5 {
            HandType::FiveOfAKind
        } else if face_counts[1] + n_jacks == 4 {
            HandType::FourOfAKind
        } else if (face_counts[1] + n_jacks == 3 && face_counts[0] == 2)
            || (face_counts[1] == 3 && face_counts[0] + n_jacks == 2)
        {
            HandType::FullHouse
        } else if face_counts.last().unwrap().to_owned() + n_jacks == 3 {
            HandType::ThreeOfAKind
        } else if (face_counts[2] + n_jacks == 2 && face_counts[1] == 2)
            || (face_counts[2] == 2 && face_counts[1] + n_jacks == 2)
        {
            HandType::TwoPair
        } else if face_counts.last().unwrap().to_owned() + n_jacks == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn get_hand_type_value(&self) -> u8 {
        match self.get_hand_type() {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn get_hand_value(&self) -> u64 {
        let hand_type_value = format!("{}", self.get_hand_type_value());
        let card_0_value = format!("{:0>2}", get_face_value(self.cards[0]));
        let card_1_value = format!("{:0>2}", get_face_value(self.cards[1]));
        let card_2_value = format!("{:0>2}", get_face_value(self.cards[2]));
        let card_3_value = format!("{:0>2}", get_face_value(self.cards[3]));
        let card_4_value = format!("{:0>2}", get_face_value(self.cards[4]));
        format!("{hand_type_value}{card_0_value}{card_1_value}{card_2_value}{card_3_value}{card_4_value}").parse().unwrap()
    }
}

fn get_face_value(face: char) -> u8 {
    match face {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_sub() {
    //     let hand = hand_from_str("QQQQQ 765");

    //     dbg!(hand.get_hand_type());

    //     assert_eq!("".to_string(), "6440".to_string());
    // }

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
        assert_eq!(result, "5905".to_string());
    }
}
