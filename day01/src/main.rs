use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let vec: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap()) 
        .collect();

    let mut ans = 0;
    for i in 1..vec.len() {
        if vec[i - 1] < vec[i] {
            ans += 1;
        }
    }

    // for part A
    println!("{}", ans);

    ans = 0;
    for i in 0..(vec.len() - 3) {
        if vec[i] + vec[i + 1] + vec[i + 2] < vec[i + 1] + vec[i + 2] + vec[i + 3] {
            ans += 1;
        }
    }

    // for part B
    println!("{}", ans);
}