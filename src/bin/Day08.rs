use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn get_map(file: File) -> Result<(HashMap<char, Vec<(usize, usize)>>, usize, usize)> {
    let reader = io::BufReader::new(file);
    let mut map = HashMap::new();
    let mut rows = 0;
    let mut columns = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        columns = line.len();
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert_with(Vec::new).push((i, j));
            }
        }
        rows += 1;
    }

    Ok((map, rows, columns))
}

//For two antennas at (r1,c1) and (r2,c2), the anti-nodes are at (2*r1-r2, 2*c1-c2) and (2*r2-r1,2*c2-c1)
fn part_1(map: &HashMap<char, Vec<(usize, usize)>>, rows: usize, columns: usize) -> usize {
    let mut anti_node_positions = HashSet::new();
    let is_within_bounds = |x: isize, y: isize| -> bool {
        x >= 0 && x < rows as isize && y >= 0 && y < columns as isize
    };

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                let anti_node = (2 * r1 as isize - r2 as isize, 2 * c1 as isize - c2 as isize);

                if is_within_bounds(anti_node.0, anti_node.1) {
                    anti_node_positions.insert((anti_node.0 as usize, anti_node.1 as usize));
                }
            }
        }
    }

    anti_node_positions.len()
}

fn part_2(map: &HashMap<char, Vec<(usize, usize)>>, rows: usize, columns: usize) -> usize {
    let mut anti_node_positions = HashSet::new();
    let is_within_bounds = |x: isize, y: isize| -> bool {
        x >= 0 && x < rows as isize && y >= 0 && y < columns as isize
    };

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let (dr, dc) = (r1 as isize - r2 as isize, c1 as isize - c2 as isize);

                let (mut r, mut c) = (r1 as isize, c1 as isize);
                while is_within_bounds(r, c) {
                    anti_node_positions.insert((r as usize, c as usize));
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    anti_node_positions.len()
}

fn main() -> Result<()> {
    let file = File::open("inputs/input08.txt")?;
    let (map, rows, columns) = get_map(file)?;

    //Part-1
    println!("{}", part_1(&map, rows, columns));
    //409

    //Part-2
    println!("{}", part_2(&map, rows, columns));
    //1308

    Ok(())
}
