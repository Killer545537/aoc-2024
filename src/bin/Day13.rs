use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq)]
struct Button {
    x: u64,
    y: u64,
}

impl FromStr for Button {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0]
            .split('+')
            .nth(1)
            .context("Invalid input")?
            .trim()
            .parse()?;
        let y = parts[1]
            .split('+')
            .nth(1)
            .context("Invalid input")?
            .trim()
            .parse()?;

        Ok(Button { x, y })
    }
}

#[derive(Hash, PartialEq, Eq)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
}

struct Prize {
    x: u64,
    y: u64,
}

impl FromStr for Prize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0]
            .split('=')
            .nth(1)
            .context("Invalid input")?
            .trim()
            .parse()?;
        let y = parts[1]
            .split('=')
            .nth(1)
            .context("Invalid input")?
            .trim()
            .parse()?;

        Ok(Prize { x, y })
    }
}

fn parse_claw_machines(file: File) -> Result<HashMap<ClawMachine, Prize>> {
    let reader = io::BufReader::new(file);
    let mut claw_machines = HashMap::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.starts_with("Button A:") {
            let button_a = Button::from_str(&line)?;
            let button_b = Button::from_str(&lines.next().unwrap()?)?;
            let prize = Prize::from_str(&lines.next().unwrap()?)?;
            claw_machines.insert(ClawMachine { button_a, button_b }, prize);
        }
    }

    Ok(claw_machines)
}

/*
The solution for the equation,
aA+bB=P (where A, B and P are vectors)
is given by,
a = (By*Px-Bx*Py)/(Ax*By-Ay*Bx) and thus, b = (Px-aAx)/Bx
If the solutions for a and b are integral, then we can say that the prize is achievable.
If the prize is achievable, then the cost to get the price is 3*a+b
 */

impl ClawMachine {
    fn cost(&self, prize: &Prize) -> Option<u64> {
        let times_a = (self.button_b.y as i64 * prize.x as i64)
            .checked_sub(self.button_b.x as i64 * prize.y as i64)? as f64
            / (self.button_a.x as i64 * self.button_b.y as i64)
                .checked_sub(self.button_a.y as i64 * self.button_b.x as i64)? as f64;
        let times_b = (prize.x as f64 - times_a * self.button_a.x as f64) / self.button_b.x as f64;

        if times_a.fract() == 0.0 && times_b.fract() == 0.0 {
            let cost = 3 * times_a as u64 + times_b as u64;
            return Some(cost);
        }

        None
    }
}

fn part_1(claw_machines: &HashMap<ClawMachine, Prize>) -> u64 {
    claw_machines
        .iter()
        .flat_map(|(claw_machine, prize)| claw_machine.cost(prize))
        .sum()
}

fn part_2(claw_machines: &HashMap<ClawMachine, Prize>) -> u64 {
    let correction = 10_000_000_000_000;
    claw_machines
        .iter()
        .flat_map(|(claw_machine, prize)| {
            let corrected_prize = Prize {
                x: prize.x + correction,
                y: prize.y + correction,
            };
            claw_machine.cost(&corrected_prize)
        })
        .sum()
}

fn main() -> Result<()> {
    let file = File::open("inputs/input13.txt")?;
    let claw_machines = parse_claw_machines(file)?;

    //Part-1
    println!("{}", part_1(&claw_machines));
    //28262

    //Part-2
    println!("{}", part_2(&claw_machines));
    //101406661266314

    Ok(())
}
