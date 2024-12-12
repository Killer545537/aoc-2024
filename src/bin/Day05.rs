use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_rules_and_updates(file: File) -> Result<(HashSet<(u32, u32)>, Vec<Vec<u32>>)> {
    let mut reader = BufReader::new(file).lines();

    let mut rules = HashSet::new();
    let mut updates = Vec::new();

    while let Some(Ok(line)) = reader.next() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let parts: Vec<u32> = line
            .split('|')
            .map(|s| Ok(s.trim().parse::<u32>()?))
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(parts.len(), 2);

        rules.insert((parts[0], parts[1]));
    }

    while let Some(Ok(line)) = reader.next() {
        let line = line.trim();
        if !line.is_empty() {
            let update: Vec<u32> = line
                .split(',')
                .map(|x| Ok(x.trim().parse()?))
                .collect::<Result<Vec<_>>>()?;

            updates.push(update);
        }
    }

    Ok((rules, updates))
}

fn is_valid_update(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if !rules.contains(&(update[i], update[j])) {
                return false;
            }
        }
    }

    true
}
fn part_1(rules: &HashSet<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            if is_valid_update(update, rules) {
                update.get(update.len() / 2).copied()
            } else {
                None
            }
        })
        .sum()
}

fn part_2(rules: &HashSet<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    fn comparator(a: &u32, b: &u32, rules: &HashSet<(u32, u32)>) -> std::cmp::Ordering {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
    updates
        .iter()
        .filter_map(|update| {
            if !is_valid_update(update, rules) {
                let mut update = update.clone();
                let mid = update.len() / 2;
                update.select_nth_unstable_by(mid, |a, b| comparator(a, b, rules));
                Some(update[mid])
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let file = File::open("inputs/input05.txt")?;

    let (rules, updates) = get_rules_and_updates(file)?;

    //Part-1
    println!("{}", part_1(&rules, &updates));
    //6498

    //Part-2
    println!("{}", part_2(&rules, &updates));
    //5017
    Ok(())
}
