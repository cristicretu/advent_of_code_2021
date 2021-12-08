use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::str::FromStr;
#[derive(Debug)]

enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //     let mut iter = s.split_whitespace();
        //     let direction = iter.next().ok_or("no direction")?;
        //     let distance = iter.next().ok_or("no distance")?;
        //     let distance = distance.parse::<i32>()?;

        //     match direction {
        //         "forward" => return Ok(Instruction::Forward(distance)),
        //         "down" => return Ok(Instruction::Down(distance)),
        //         "up" => return Ok(Instruction::Up(distance)),
        //         _ => return Err(From::from("invalid direction")),
        //     }
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^(forward|down|up) (\d+)$"#).unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from(format!("{}", s))),
            Some(caps) => caps,
        };

        let distance: i64 = caps[2].parse()?;
        let direction = caps[1].to_string();

        match direction.as_str() {
            "forward" => return Ok(Instruction::Forward(distance)),
            "down" => return Ok(Instruction::Down(distance)),
            "up" => return Ok(Instruction::Up(distance)),
            _ => return Err(From::from("invalid direction")),
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let vec: Vec<Instruction> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    /*
        Part 1.
    */
    let mut horiz = 0;
    let mut vert = 0;

    // for i in vec {
    //     match i {
    //         Instruction::Forward(dist) => horiz += dist,
    //         Instruction::Down(dist) => vert += dist,
    //         Instruction::Up(dist) => vert -= dist,
    //     }
    // }

    // println!("{}", vert * horiz);

    /*
        Part 2
    */
    let mut aim: i64 = 0;

    for i in vec {
        match i {
            Instruction::Forward(x) => {
                horiz += x;
                vert += aim * x;
            }
            Instruction::Down(x) => aim += x,
            Instruction::Up(x) => aim -= x,
        }
    }

    println!("{}", vert * horiz);
}
