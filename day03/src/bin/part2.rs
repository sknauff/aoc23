use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{:}",output);
}

fn part2(input: &str) -> String {
    let input_v = input.lines().collect::<Vec<&str>>();
    let mut v_engine: Vec<Engine> = Vec::new();
    let mut v_gears: Vec<Gear> = Vec::new();

    // Distance to check for characters around Engine Number
    let distance: i32 = 1;
    
    // Generate Engines
    for n in 0..input_v.len() {        
        let re = Regex::new(r"\D").unwrap();
        let v_numbers = re
            .split(input_v[n].trim())
            .collect::<Vec<&str>>();

        let max_line_len = input_v[n].trim().len() as i32;

        let mut start = 0;

        for i in v_numbers {        
            if i.len() > 0 {
                
                let context_start = start - distance;
                let context_end = context_start + ( i.len() as i32) + 2 * distance;

                let context_start = if context_start < 0 { 0 } else { context_start };
                let context_end = if context_end > max_line_len { max_line_len } else { context_end };

                let context_start = context_start as usize;
                let context_end = context_end as usize;

                // dbg!(&context_start);
                // dbg!(&context_end);
                // dbg!(&max_line_len);

                let mut e_context: Vec<&str> = Vec::new();

                if n > 0 {
                    e_context.push(&input_v[n - 1].trim()[context_start..context_end]);
                }
        
                e_context.push(&input_v[n].trim()[context_start..context_end]);
        
                if n < input_v.len() - 1 {
                    e_context.push(&input_v[n + 1].trim()[context_start..context_end]);
                }

                // println!("value: {}, len: {}, start: {}, end: {}, res: {}", i, i.len(), start, start + (i.len() as i32),  context);
                v_engine.push(
                    Engine { 
                        e_context: e_context.join(""), 
                        value: i.parse::<u64>().unwrap(),
                        line: n as i32,
                        start: start,
                        end: start + (i.len() as i32),
                    }
                )
            
            }
            
            // keep track of position within string
            if i.len() != 0 { start += i.len() as i32 };
            start += 1;
        }
    }

    // Generate Gears
    for n in 0..input_v.len() {

        let gears: Vec<_> = input_v[n].trim().match_indices("*").collect();

        if gears.len() > 0 {
            for (gear, _) in gears {
                v_gears.push(Gear { line: n as i32, start_end: gear as i32 });
            }
        }
    }

    // Evaluate Engines
    let total = v_gears.iter().map(|gear| 
        gear.find_adjacent_Engine(v_engine.clone())
    );

    let answer: u64 = total.collect::<Vec<u64>>().iter().sum();

    // dbg!(v_engine);
    // dbg!(v_gears);

    format!("{answer}").to_string()
}


#[derive(Debug,Clone)]
struct Engine {
    e_context: String,
    value: u64,
    line: i32,
    start: i32,
    end: i32,
}


// impl Engine {
//     fn is_valid_pt2(&self) -> bool {
//         let re = Regex::new(r"\d").unwrap();
//         (re.replace_all(&self.e_context.replace(".", ""), "")).len() > 0
//     }
// }


impl Engine {
    fn touches(&self, line: i32, point: i32) -> bool {
        
        let distance = 1;

        let context_start = self.start - distance;
        let context_end = self.end;

        let range_point = context_start..=context_end;
        let range_line = (self.line-1)..=(self.line+1);

        // dbg!(self);
        // dbg!(&range_point);
        // dbg!(&range_line);

        range_point.contains(&point) & range_line.contains(&line)

    }
}


#[derive(Debug,Copy,Clone)]
struct Gear {
    line: i32,
    start_end: i32,
}

impl Gear {
    fn find_adjacent_Engine(&self, v_engine: Vec<Engine>) -> u64 {
        // let mut v_engine = v_engine.to_owned();
        let mut valid_engines: Vec<Engine> = Vec::new();

        for e in v_engine {
            if e.touches(self.line, self.start_end) {
                valid_engines.push(e);
            }
        }

        // dbg!(&valid_engines);

        if valid_engines.len() == 2 {
            valid_engines.iter().map(|x| x.value).product()
        } else {
            0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(result, "467835".to_string());
    }
}