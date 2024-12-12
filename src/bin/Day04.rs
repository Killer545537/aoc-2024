use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn get_puzzle(file: File) -> Result<Vec<Vec<char>>> {
    let reader = io::BufReader::new(file);
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            let line = line?;
            Ok(line.chars().collect::<Vec<char>>())
        })
        .collect::<Result<_>>()?;

    Ok(matrix)
}

fn find_occurrences(puzzle: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = puzzle.len() as isize;
    let cols = puzzle[0].len() as isize;

    const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let is_valid = |row: isize, col: isize, dr: isize, dc: isize| {
        (0..TARGET.len()).all(|i| {
            let new_row = row + i as isize * dr;
            let new_col = col + i as isize * dc;
            new_row >= 0
                && new_col >= 0
                && new_row < rows
                && new_col < cols
                && puzzle[new_row as usize][new_col as usize] == TARGET[i]
        })
    };

    DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| is_valid(row as isize, col as isize, dr, dc))
        .count()
}

fn part_1(puzzle: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            if puzzle[row][col] == 'X' {
                count += find_occurrences(puzzle, row, col);
            }
        }
    }

    count
}

#[test]
fn check_part_1() {
    let puzzle: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];
    assert_eq!(part_1(&puzzle), 18);
}

fn is_x_mas(puzzle: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
    let patterns: HashSet<&str> = ["MMSS", "MSSM", "SSMM", "SMMS"].iter().cloned().collect();

    patterns.contains(
        DIRECTIONS
            .iter()
            .map(|&(dr, dc)| puzzle[(row as isize + dr) as usize][(col as isize + dc) as usize])
            .collect::<String>()
            .as_str(),
    )
}

fn part_2(puzzle: &Vec<Vec<char>>) -> usize {
    let rows = puzzle.len();
    let cols = puzzle[0].len();

    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if puzzle[row][col] == 'A' {
                if is_x_mas(&puzzle, row, col) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[test]
fn check_part_2() {
    let puzzle: Vec<Vec<char>> = vec![
        vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
        vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
        vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
        vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
        vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
        vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(part_2(&puzzle), 9);
}

fn main() -> Result<()> {
    let file = File::open("inputs/input04.txt")?;
    let puzzle_input = get_puzzle(file)?;

    //Part-1
    println!("{}", part_1(&puzzle_input));
    //2447

    //Part-2
    println!("{}", part_2(&puzzle_input));

    Ok(())
}
