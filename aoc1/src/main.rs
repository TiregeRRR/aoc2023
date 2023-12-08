use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

fn main() {
    assert_eq!(solve_a("test_data_a").unwrap(), 142);
    println!("{}", solve_a("input").unwrap());

    assert_eq!(solve_b("test_data_b").unwrap(), 281);
    println!("{}", solve_b("input").unwrap())
}

fn solve_b(filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let l = line?;

        let mut first_num: u32 = 0;
        let mut second_num: u32 = 0;

        for (i, c) in l.chars().enumerate() {
            let v = match_string(&l, i);
            if let Some(v) = v {
                first_num = v;

                break;
            }

            if c.is_numeric() {
                first_num = c.to_digit(10).unwrap();

                break;
            }
        }

        for (i, c) in l.chars().rev().enumerate() {
            let v = match_string(&l, l.len() - i);
            if let Some(v) = v {
                second_num = v;

                break;
            }

            if c.is_numeric() {
                second_num = c.to_digit(10).unwrap();

                break;
            }
        }

        sum += first_num * 10 + second_num;
    }

    Ok(sum)
}

fn solve_a(filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let l = line?;

        let mut first_num: u32 = 0;
        let mut second_num: u32 = 0;

        for (_, c) in l.chars().enumerate() {
            if c.is_numeric() {
                first_num = c.to_digit(10).unwrap();

                break;
            }
        }

        for (_, c) in l.chars().rev().enumerate() {
            if c.is_numeric() {
                second_num = c.to_digit(10).unwrap();

                break;
            }
        }

        sum += first_num * 10 + second_num;
    }

    Ok(sum)
}

fn match_string(l: &String, i: usize) -> Option<u32> {
    if let Some(part) = l.get(i..i + 3) {
        match part {
            "one" => {
                return Some(1);
            }
            "two" => {
                return Some(2);
            }
            "six" => {
                return Some(6);
            }
            _ => (),
        }
    }

    if let Some(part) = l.get(i..i + 4) {
        match part {
            "four" => {
                return Some(4);
            }
            "five" => {
                return Some(5);
            }
            "nine" => {
                return Some(9);
            }
            _ => (),
        }
    }

    if let Some(part) = l.get(i..i + 5) {
        match part {
            "three" => {
                return Some(3);
            }
            "seven" => {
                return Some(7);
            }
            "eight" => {
                return Some(8);
            }
            _ => (),
        }
    }

    None
}
