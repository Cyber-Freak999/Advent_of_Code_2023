use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn extract_calibration_values(line: &str) -> Option<u32> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
    
    let first = first_digit.to_digit(10)?;
    let last = last_digit.to_digit(10)?;
    
    Some(first * 10 + last)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;

    for line in reader.lines() {
        if let Ok(input) = line {
            if let Some(calibration_value) = extract_calibration_values(&input) {
                total_sum += calibration_value;
            }
        }
    }

    println!("Total sum of calibration values: {}", total_sum);

    Ok(())
}
