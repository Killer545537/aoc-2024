use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lists(file: File) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.context("Failed to read a line")?;
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().context("Unexpected character"))
            .collect::<Result<Vec<i32>, _>>()?;

        if parts.len() == 2 {
            left.push(parts[0]);
            right.push(parts[1]);
        }
    }

    Ok((left, right))
}

fn part_1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let left = {
        let mut copy = left.clone();
        copy.sort();
        copy
    };
    let right = {
        let mut copy = right.clone();
        copy.sort();
        copy
    };

    left.into_iter()
        .zip(right)
        .fold(0, |mut sum: i32, (left, right)| {
            sum += (left - right).abs();
            sum
        })
}

#[test]
fn check_part_1() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    assert_eq!(part_1(&left, &right), 11);
}

fn part_2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let counter = right.into_iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });

    left.into_iter()
        .map(|item| item * counter.get(&item).unwrap_or(&0))
        .sum()
}

#[test]
fn check_part_2() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    assert_eq!(part_2(&left, &right), 31);
}

fn main() -> Result<()> {
    let file = File::open("inputs/input01.txt")?;
    let (left, right) = read_lists(file)?;

    //Part-1
    println!("{}", part_1(&left, &right));
    // 1651298

    //Part-2
    println!("{}", part_2(&left, &right));
    // 21306195

    Ok(())
}
