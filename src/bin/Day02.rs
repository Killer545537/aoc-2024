use anyhow::{Context, Result};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_reports(file: File) -> Result<Vec<Vec<i32>>> {
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.context("Failed to read a line")?
                .split_whitespace()
                .map(|s| s.parse().context("Unexpected character"))
                .collect()
        })
        .collect()
}

enum ReportState {
    Increasing,
    Decreasing,
    Neither,
}

fn determine_state(report: &[i32]) -> ReportState {
    let is_increasing = report.windows(2).all(|w| {
        let difference = w[1] - w[0];
        (1..=3).contains(&difference)
    });
    let is_decreasing = report.windows(2).all(|w| {
        let difference = w[0] - w[1];
        (1..=3).contains(&difference)
    });

    match (is_increasing, is_decreasing) {
        (true, false) => ReportState::Increasing,
        (false, true) => ReportState::Decreasing,
        _ => ReportState::Neither,
    }
}

fn is_safe(report: &[i32]) -> bool {
    matches!(
        determine_state(report),
        ReportState::Increasing | ReportState::Decreasing
    )
}

fn part_1(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

#[test]
fn check_part_1() {
    let reports: Vec<Vec<i32>> = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    assert_eq!(part_1(&reports), 2);
}

fn part_2(reports: &Vec<Vec<i32>>) -> usize {
    fn is_safe_with_weaker_condition(report: &[i32]) -> bool {
        for i in 0..report.len() {
            let modified_report = {
                let mut temp = report.to_vec();
                temp.remove(i);
                temp
            };

            if is_safe(&modified_report) {
                return true;
            }
        }

        false
    }

    reports
        .iter()
        .filter(|report| is_safe_with_weaker_condition(report))
        .count()
}

#[test]
fn check_part_2() {
    let reports: Vec<Vec<i32>> = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    assert_eq!(part_2(&reports), 4);
}

fn main() -> Result<()> {
    let file = File::open("inputs/input02.txt")?;
    let reports = read_reports(file)?;

    //Part-1
    println!("{}", part_1(&reports));
    // 314

    //Part-2
    println!("{}", part_2(&reports));
    // 373

    Ok(())
}
