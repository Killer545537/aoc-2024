use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn get_initial_stones(file: File) -> Result<Vec<u128>> {
    let reader = io::BufReader::new(file);

    let stones = reader
        .lines()
        .next()
        .ok_or(anyhow::anyhow!(
            "Input does not have the initial configuration"
        ))??
        .split_whitespace()
        .map(|s| Ok(s.parse::<u128>()?))
        .collect::<Result<Vec<u128>>>()?;

    Ok(stones)
}

fn transformation(initial_stones: &[u128]) -> Result<Vec<u128>> {
    let rule_1 = || -> u128 { 1 };
    let rule_2 = |x: &str| -> Result<Vec<u128>> {
        let len = x.len();
        let (first_half, second_half) = x.split_at(len / 2);
        Ok(vec![first_half.parse()?, second_half.parse()?])
    };
    let rule_3 = |x: u128| 2024 * x;

    let mut modified_stones = Vec::new();

    for &stone in initial_stones {
        if stone == 0 {
            modified_stones.push(rule_1());
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                if let Ok(new_stones) = rule_2(&stone_str) {
                    modified_stones.extend(new_stones);
                } else {
                    modified_stones.push(rule_3(stone));
                }
            } else {
                modified_stones.push(rule_3(stone));
            }
        }
    }

    Ok(modified_stones)
}

fn part_1(initial_stones: &[u128], blinks: usize) -> Result<Vec<u128>> {
    let mut stones = initial_stones.to_vec();
    for _ in 0..blinks {
        stones = transformation(&stones)?;
    }

    Ok(stones)
}

//Since each stone is independent of every other stone, we can sum the number of stones each stone eventually splits into
fn count_stones_split_into(
    stone: u128,
    blinks: usize,
    memo: &mut HashMap<(u128, usize), u128>,
) -> Result<u128> {
    if let Some(&result) = memo.get(&(stone, blinks)) {
        return Ok(result);
    }

    let result = if blinks == 0 {
        1
    } else if stone == 0 {
        count_stones_split_into(1, blinks - 1, memo)?
    } else {
        let stone_string = stone.to_string();
        let len = stone_string.len();

        if len % 2 == 0 {
            let (first_half, second_half) = stone_string.split_at(len / 2);
            count_stones_split_into(first_half.parse()?, blinks - 1, memo)?
                + count_stones_split_into(second_half.parse()?, blinks - 1, memo)?
        } else {
            count_stones_split_into(stone * 2024, blinks - 1, memo)?
        }
    };

    memo.insert((stone, blinks), result);
    Ok(result)
}

fn main() -> Result<()> {
    let file = File::open("inputs/input11.txt")?;

    let initial_stones = get_initial_stones(file)?;

    //Part-1
    println!("{:?}", part_1(&initial_stones, 25)?.len());
    //184927

    let mut memo = HashMap::new();
    let total_stones: u128 = initial_stones
        .iter()
        .map(|&stone| count_stones_split_into(stone, 75, &mut memo))
        .collect::<Result<Vec<u128>>>()?
        .iter()
        .sum();

    //Part-2
    println!("{}", total_stones);
    //220357186726677

    Ok(())
}
