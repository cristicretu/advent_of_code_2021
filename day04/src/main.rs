use std::fs;

struct Board {
    elem: [i32; 25],
}

impl Board {
    fn mark(&mut self, val: i32) {
        for i in 0..self.elem.len() {
            if self.elem[i] == val {
                self.elem[i] = -1;
                break;
            }
        }
    }
    fn check(&self) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if self.elem[row * 5 + col] != -1 {
                    continue;
                }
            }
            return true;
        }
        for col in 0..5 {
            for row in 0..5 {
                if self.elem[row * 5 + col] != -1 {
                    continue;
                }
            }
            return true;
        }
        return false;
    }
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.elem.len() {
            if self.elem[i] != -1 {
                sum += self.elem[i];
            }
        }
        return sum;
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let lines: Vec<&str> = input.lines().collect();
    let (first, rest) = lines.split_at(2);
    let nums: Vec<i32> = first[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut a: &[&str];
    let mut b = rest;

    while b.len() > 0 {
        let vals = b.split_at(6);
        a = vals.0;
        b = vals.1;

        let mut board = Board { elem: [0; 25] };

        for row in 0..5 {
            for (col, num_str) in a[row].trim().split_ascii_whitespace().enumerate() {
                board.elem[row * 5 + col] = num_str.parse().unwrap();
            }
        }

        boards.push(board);
    }

    // Check who won
    for val in nums.iter() {
        for board in boards.iter_mut() {
            board.mark(*val);
            if board.check() {
                println!("{}", board.sum() * val);
                return;
            }
        }
    }

    // println!("{:?}", nums);
    // let splitted_board = rest[0].split("\n").collect::<Vec<&str>>();
    // let arr = splitted_board[0]
    //     .split_whitespace()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect::<Vec<i32>>();
}
