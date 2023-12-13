use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn main() {
    assert_eq!(solve_a("test_data").unwrap(), 4361);
    println!("{}", solve_a("input").unwrap());
}

fn solve_a(filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Vec<_>>();

    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        let l = line.as_ref().unwrap();
        let mut digit_start: i32 = -1;
        let mut digit_end: i32 = -1;

        for (j, c) in l.chars().enumerate() {
            if c.is_digit(10) {
                if digit_start == -1 {
                    digit_start = j.try_into()?;
                    digit_end = j.try_into()?;
                } else {
                    digit_end = j.try_into()?;
                }

                if j != l.len() - 1 {
                    continue;
                }
            }

            if digit_start != -1 {
                let starting_iter = i.checked_sub(1).unwrap_or(0);
                let mut taking_iter = 3;
                if i == 0 {
                    taking_iter = 2;
                }

                let mut valid_code = false;
                for symbol_line in lines.iter().skip(starting_iter).take(taking_iter) {
                    let d = symbol_line.as_ref().unwrap();
                    let mut starting_symbol: usize = digit_start.try_into()?;
                    starting_symbol = starting_symbol.checked_sub(1).unwrap_or(0);

                    let mut end_symbol: usize = digit_end.try_into()?;
                    end_symbol = end_symbol
                        .checked_add(2)
                        .unwrap_or(end_symbol.checked_add(1).unwrap_or(end_symbol));

                    for char_v in d
                        .chars()
                        .skip(starting_symbol)
                        .take(end_symbol - starting_symbol)
                    {
                        if !char_v.is_digit(10) && char_v != '.' {
                            valid_code = true
                        }
                    }
                }

                if valid_code {
                    let b: String = l.chars().collect::<Vec<_>>()
                        [digit_start.try_into()?..(digit_end + 1).try_into()?]
                        .into_iter()
                        .collect();

                    sum += b.parse::<u32>()?;
                }

                (digit_start, digit_end) = (-1, -1);
            }
        }
    }

    Ok(sum)
}
