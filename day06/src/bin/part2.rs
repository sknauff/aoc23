use rayon::prelude::*;

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

    let v_times = v_lines[0].split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let v_distances = v_lines[1].split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let v_times = v_times.join("");
    let v_distances = v_distances.join("");

    let race = Race {
        time: v_times.parse().unwrap(),
        distance: v_distances.parse().unwrap(),
    };

    let answer: u64 = race.get_amount_of_options_to_set_new_record();

    answer.to_string()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_amount_of_options_to_set_new_record(&self) -> u64 {
        let n_wins: u64 = (0..=self.time)
            .into_par_iter()
            .map(|x| self.set_new_record_with_seconds_pressed(x) as u64)
            .collect::<Vec<u64>>()
            .par_iter()
            .sum();
        n_wins
    }

    fn set_new_record_with_seconds_pressed(&self, seconds_pressed: u64) -> bool {
        self.distance < seconds_pressed * (self.time - seconds_pressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "
            Time:      7  15   30
            Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string());
    }
}
