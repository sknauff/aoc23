fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}", output);
}

fn part1(input: &str) -> String {
    let input = input
        .lines()
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .join("\n");
    let input = input.split("\n\n").collect::<Vec<&str>>();

    println!("Post input.");

    let binding: Vec<&str> = input[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

    println!("Post binding.");

    let v_seeds: Vec<Seed> = binding
        .iter()
        .map(|x| Seed {
            number: x.parse::<u64>().unwrap(),
        })
        .collect();

    println!("Post v_seeds.");

    let v_maps: Vec<Map> = input[1..].iter().map(|x| new_map_from(x)).collect();

    println!("Post v_maps.");

    let tmp = v_seeds
        .iter()
        .map(|x| seed_location(x, &v_maps))
        .collect::<Vec<u64>>();

    tmp.iter().min().unwrap().to_owned().to_string()
    // "".to_string()
}

fn seed_location(seed: &Seed, v_maps: &Vec<Map>) -> u64 {
    let mut current_location = seed.number.to_owned();
    println!("START: {}", current_location);
    for map in v_maps {
        println!(
            "{} {} Current number = {}",
            map.source, map.destination, current_location
        );
        for (destination, source, length) in &map.destination_source_length {
            let destination = destination.to_owned();
            let source = source.to_owned();
            let length = length.to_owned();

            print!("{} {} {} //  ", destination, source, length);

            if (source..(source + length)).contains(&current_location) {
                print!(
                    "current_location {} - source {} + destination {} ==> ",
                    current_location, source, destination
                );
                current_location =
                    (current_location as i64 - source as i64 + destination as i64) as u64;
                break;
            }
            println!("{}", current_location);
        }
        println!("TMP END {}", current_location);
    }
    println!("END: {}", current_location);
    current_location
}

#[derive(Debug)]
struct Seed {
    number: u64,
}

#[derive(Debug, PartialEq)]
struct Map {
    source: String,
    destination: String,
    destination_source_length: Vec<(u64, u64, u64)>,
}

fn new_map_from(input: &str) -> Map {
    let v_lines = input.trim().lines().collect::<Vec<&str>>();
    let definition_line = v_lines[0].trim();
    let definition_line = &definition_line[..definition_line.len() - 5]
        .split('-')
        .collect::<Vec<&str>>();

    let mut destination_source_length: Vec<(u64, u64, u64)> = Vec::new();

    println!("new_map_from {}", v_lines[0]);

    for line in &v_lines[1..] {
        let v: Vec<u64> = line
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        destination_source_length.push((v[0], v[1], v[2]));
    }

    Map {
        source: definition_line[0].to_string(),
        destination: definition_line[2].to_string(),
        destination_source_length: destination_source_length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!(result, "35".to_string());
    }
}
