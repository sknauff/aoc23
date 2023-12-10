fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}",output);
}


fn part1(input: &str) -> String {
    let total = input.lines().map(|line| {
        let game = read_line(line);
        if game.contains_at_least(Draw{ red: 12, green: 13, blue: 14 }) {
            game.id
        } else {
            0
        }
    });
    let answer: u64 = total.collect::<Vec<u64>>().iter().sum();
    format!("{answer}").to_string()
}


fn read_line(input: &str) -> Game {
    let input: Vec<&str> = input.split(":").collect::<Vec<&str>>();
    let game_id = input[0].trim()[5..].parse::<u64>().unwrap();
    let mut draws: Vec<Draw> = Vec::new();

    for draw in input[1].split(";") {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color in draw.split(","){
            let elem: Vec<&str> = color.trim().split(" ").collect();
            match elem[1] {
                "red" => {
                    red = elem[0].parse::<u64>().unwrap();
                }
                "green" => {
                    green = elem[0].parse::<u64>().unwrap();
                }
                "blue" => {
                    blue = elem[0].parse::<u64>().unwrap();
                }
                _ => {}
            }   
        }
        draws.push(Draw { red: red, green: green, blue: blue })
    }

    Game { 
        id: game_id, 
        draws: draws
    }
}

#[derive(Debug)]
struct Draw {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug)]
struct Game {
    id: u64,
    draws: Vec<Draw>,
}

impl Game {
    fn contains_at_least(&self, draw: Draw) -> bool {
        let max_blue = self.draws.iter().map(|x| x.blue).max().unwrap();
        let max_red = self.draws.iter().map(|x| x.red).max().unwrap();
        let max_green = self.draws.iter().map(|x| x.green).max().unwrap();
        
        if max_blue <= draw.blue && max_red <= draw.red && max_green <= draw.green {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8".to_string());
    }
}