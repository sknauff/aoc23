use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{:}",output);
}

fn part1(input: &str) -> String {
    let input_v = input.lines().collect::<Vec<&str>>();
    let mut v_engine: Vec<Engine> = Vec::new();

    // Distance to check for characters around Engine Number
    let distance: i32 = 1;
    
    // Generate Engines
    for n in 0..input_v.len() {        
        let re = Regex::new(r"\D").unwrap();
        let v_numbers = re
            .split(input_v[n].trim())
            .collect::<Vec<&str>>();

        let max_line_len = input_v[n].len() as i32;

        let mut start = 0;

        for i in v_numbers {        
            // dbg!(i);
            if i.len() > 0 {
                
                let context_start = start - distance;
                let context_end = context_start + ( i.len() as i32) + 2* distance;

                let context_start = if context_start < 0 { 0 } else { context_start };
                let context_end = if context_end > max_line_len { max_line_len } else { context_end };

                let context_start = context_start as usize;
                let context_end = context_end as usize;

                let mut e_context: Vec<&str> = Vec::new();

                if n > 0 {
                    e_context.push(&input_v[n - 1].trim()[context_start..context_end]);
                }
        
                e_context.push(&input_v[n].trim()[context_start..context_end]);
        
                if n < input_v.len() - 1 {
                    e_context.push(&input_v[n + 1].trim()[context_start..context_end]);
                }

                // println!("value: {}, len: {}, start: {}, end: {}, res: {}", i, i.len(), start, start + (i.len() as i32),  context);
                v_engine.push(Engine { e_context: e_context.join(""), value: i.parse::<u64>().unwrap() })
            
            }
            
            // keep track of position within string
            if i.len() != 0 { start += i.len() as i32 };
            start += 1;
        }
    }

    // Evaluate Engines
    let total = v_engine.iter().map(|engine| 
        if engine.is_valid_pt1() {
            engine.value
        } else {
            0
        }
    );
    let answer: u64 = total.collect::<Vec<u64>>().iter().sum();
    format!("{answer}").to_string()
}

#[derive(Debug)]
struct Engine {
    e_context: String,
    value: u64,
}

impl Engine {
    fn is_valid_pt1(&self) -> bool {
        let re = Regex::new(r"\d").unwrap();
        (re.replace_all(&self.e_context.replace(".", ""), "")).len() > 0
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(result, "4361".to_string());
    }
}