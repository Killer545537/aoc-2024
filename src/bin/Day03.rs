use anyhow::{Result, anyhow};
use regex::Regex;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{self, Read};

fn read_instructions(file: File) -> Result<String> {
    let mut reader = io::BufReader::new(file);
    let mut corrupted_instructions = String::new();

    reader.read_to_string(&mut corrupted_instructions)?;

    Ok(corrupted_instructions)
}

fn part_1(corrupted_instructions: &str) -> Result<i32> {
    let regex = Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)")?;

    let sum = regex
        .captures_iter(corrupted_instructions)
        .map(|caps| {
            let num1: i32 = caps["num1"].parse()?;
            let num2: i32 = caps["num2"].parse()?;

            Ok(num1 * num2)
        })
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .fold(0, |acc, x| acc + x);

    Ok(sum)
}

#[test]
fn check_part_1() {
    let corrupted_instructions =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(part_1(corrupted_instructions).unwrap(), 161);
}

#[derive(Eq, PartialEq)]
enum Command {
    Do,
    Dont,
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "do()" => Ok(Command::Do),
            "don't()" => Ok(Command::Dont),
            _ => Err(anyhow!("Invalid Command: {}", s)),
        }
    }
}

fn part_2(corrputed_instructions: &str) -> Result<i32> {
    let regex =
        Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)|(?P<cmd>do\(\)|don't\(\))")?;
    let mut last_command = Command::Do;

    let sum = regex
        .captures_iter(corrputed_instructions)
        .map(|caps| {
            if let Some(cmd) = caps.name("cmd") {
                last_command = cmd.as_str().parse()?;
            } else if last_command == Command::Do {
                let num1: i32 = caps["num1"].parse()?;
                let num2: i32 = caps["num2"].parse()?;
                return Ok(num1 * num2);
            }
            Ok(0)
        })
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .fold(0, |acc, x| acc + x);

    Ok(sum)
}

#[test]
fn check_part_2() {
    let corrupted_instructions =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part_2(corrupted_instructions).unwrap(), 48);
}

fn main() -> Result<()> {
    let file = File::open("inputs/input03.txt")?;
    let corrupted_instructions = read_instructions(file)?;

    //Part-1
    println!("{}", part_1(&corrupted_instructions)?);
    //192767529

    //Part-2
    println!("{}", part_2(&corrupted_instructions)?);

    Ok(())
}
