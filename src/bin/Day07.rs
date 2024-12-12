use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn get_calibration_equations(file: File) -> Result<HashMap<u64, Vec<u64>>> {
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut parts = line.split(':');
            let key: u64 = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing key"))?
                .trim()
                .parse()?;
            let values: Vec<u64> = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing values"))?
                .split_whitespace()
                .map(|c| Ok(c.trim().parse()?))
                .collect::<Result<Vec<u64>>>()?;

            Ok((key, values))
        })
        .collect::<Result<HashMap<u64, Vec<u64>>>>()
}

fn can_obtain(target: u64, values: &[u64]) -> bool {
    match (target, values) {
        (0, _) => true,
        (_, []) => false,
        (t, [rest @ .., last]) => {
            t == *last
                || (t >= *last && can_obtain(t - last, rest))
                || (t % last == 0 && can_obtain(target / last, rest))
        }
    }
}

#[test]
fn check_can_obtain() {
    assert_eq!(can_obtain(190, &[19, 10]), true);
    assert_eq!(can_obtain(3267, &[81, 40, 27]), true);
    assert_eq!(can_obtain(83, &[17, 5]), false);
}

fn part_1(calibration_equations: &HashMap<u64, Vec<u64>>) -> u64 {
    calibration_equations
        .iter()
        .filter_map(|(&key, value)| {
            if can_obtain(key, value) {
                Some(key)
            } else {
                None
            }
        })
        .sum()
}

fn can_obtain_with_concat(target: u64, values: &[u64]) -> bool {
    match (target, values) {
        (0, _) => true,
        (_, []) => false,
        (t, [rest @ .., last]) => {
            t == *last
                || (t >= *last && can_obtain_with_concat(t - last, rest))
                || (t % last == 0 && can_obtain_with_concat(target / last, rest))
                || {
                    let target_str = t.to_string();
                    let last_str = last.to_string();
                    if target_str.len() > last_str.len() && target_str.ends_with(&last_str) {
                        let new_target: u64 = target_str[..target_str.len() - last_str.len()]
                            .parse()
                            .unwrap();
                        can_obtain_with_concat(new_target, rest)
                    } else {
                        false
                    }
                }
        }
    }
}

fn part_2(calibration_equations: &HashMap<u64, Vec<u64>>) -> u64 {
    calibration_equations
        .iter()
        .filter_map(|(&key, value)| {
            if can_obtain_with_concat(key, value) {
                Some(key)
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let file = File::open("inputs/input07.txt")?;
    let calibration_equations = get_calibration_equations(file)?;

    //Part-1
    println!("{}", part_1(&calibration_equations));
    //7579994664753

    //Part-2
    println!("{}", part_2(&calibration_equations));
    //438027111276610

    Ok(())
}
