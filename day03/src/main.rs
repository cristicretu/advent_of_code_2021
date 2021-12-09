use std::fs;

fn binary_to_decimal(digits: &Vec<char>) -> i64 {
    let mut res = 0;
    let num_digits = digits.len();

    for (pos, c) in digits.iter().enumerate() {
        let exp = num_digits as i32 - pos as i32 - 2;
        let mul = match exp {
            -1 => 1,
            _ => 2 << exp,
        };

        if *c == '1' {
            res += mul;
        }
    }

    return res;
}

fn get_mcb(vec: &Vec<Vec<char>>, pos: i32) -> i64 {
    vec.iter().fold(0, |accum, row| {
        if row[pos as usize] == '0' {
            accum - 1
        } else {
            accum + 1
        }
    })
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let nums: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut oxy = nums.clone();
    let mut co2 = nums.clone();

    for i in 0..12 {
        if oxy.len() > 1 {
            let mcb = get_mcb(&oxy, i as i32);
            let my_char = if mcb >= 0 { '1' } else { '0' };

            oxy = oxy
                .iter()
                .filter(|val| val[i as usize] == my_char)
                .map(|val| val.clone())
                .collect();
        }

        if co2.len() > 1 {
            let mcb = get_mcb(&co2, i as i32);
            let my_char = if mcb >= 0 { '0' } else { '1' };

            co2 = co2
                .iter()
                .filter(|val| val[i as usize] == my_char)
                .map(|val| val.clone())
                .collect();
        }
    }

    let ooxy = binary_to_decimal(&oxy[0]);
    let ccoo2 = binary_to_decimal(&co2[0]);

    println!("{}", ooxy * ccoo2);
}
