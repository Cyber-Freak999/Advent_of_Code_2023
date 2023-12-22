use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn extract_calibration_values(line: &str) -> Option<u32> {
    let first_digit_char = line
        .chars()
        .find(|c| c.is_digit(10))
        .or_else(|| line.parse::<u32>().ok())?;

    let last_digit_char = line
        .chars()
        .rev()
        .find(|c| c.is_digit(10))
        .or_else(|| line.parse::<u32>().ok())?;

    Some(
        first_digit_char.to_digit(10)? as u32 * 10
            + last_digit_char.to_digit(10)? as u32,
    )
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let mut reader = io::BufReader::new(file);

    let mut total_sum = 0;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_until(b'\n', &mut line)?;

        if bytes_read == 0 {
            break;
        }

        if let Some(calibration_value) = extract_calibration_values(&line) {
            total_sum += calibration_value;
        }
    }

    println!("Total sum of calibration values: {}", total_sum);

    Ok(())
}
