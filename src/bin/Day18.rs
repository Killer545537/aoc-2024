use anyhow::{Context, Result};
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;

const MAX_X: usize = 70;
const MAX_Y: usize = 70;
const BYTES: usize = 1024;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Safe,
    Corrupt,
}

struct Computer {
    memory: [[Cell; MAX_Y + 1]; MAX_X + 1],
}

impl Computer {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn from_file(file: File, bytes: usize) -> Result<Self> {
        let mut memory = [[Cell::Safe; MAX_Y + 1]; MAX_X + 1];
        let reader = io::BufReader::new(file);

        for line in reader.lines().take(bytes) {
            let line = line?;
            let coords: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().context("Invalid input"))
                .collect::<Result<Vec<usize>>>()?;
            if coords.len() != 2 {
                return Err(anyhow::anyhow!("Invalid coordinates"));
            }

            let (x, y) = (coords[0], coords[1]);
            if x > MAX_X || y > MAX_Y {
                return Err(anyhow::anyhow!("OOB"));
            }

            memory[x][y] = Cell::Corrupt;
        }

        Ok(Computer { memory })
    }

    fn shortest_path(&self) -> Option<usize> {
        let start = (0, 0);
        let end = (MAX_Y, MAX_Y);
        let mut queue = VecDeque::new();
        let mut visited = [[false; MAX_Y + 1]; MAX_X + 1];

        queue.push_back((start, 0));
        visited[start.0][start.1] = true;

        while let Some(((x, y), dist)) = queue.pop_front() {
            if (x, y) == end {
                return Some(dist);
            }

            for &(dx, dy) in &Self::DIRECTIONS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx <= MAX_X as isize && ny >= 0 && ny <= MAX_Y as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[nx][ny] && self.memory[nx][ny] == Cell::Safe {
                        visited[nx][ny] = true;
                        queue.push_back(((nx, ny), dist + 1));
                    }
                }
            }
        }

        None
    }

    fn first_byte_so_no_escape(file: File) -> Result<(usize, usize)> {
        let mut memory = [[Cell::Safe; MAX_Y + 1]; MAX_X + 1];
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let coords: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().context("Invalid input"))
                .collect::<Result<Vec<usize>>>()?;
            if coords.len() != 2 {
                return Err(anyhow::anyhow!("Invalid coordinates"));
            }

            let (x, y) = (coords[0], coords[1]);
            if x > MAX_X || y > MAX_Y {
                return Err(anyhow::anyhow!("OOB"));
            }

            memory[x][y] = Cell::Corrupt;

            let computer = Computer { memory };
            if computer.shortest_path().is_none() {
                return Ok((x, y));
            }
        }

        Ok((0, 0))
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input18.txt")?;
    let computer = Computer::from_file(file, BYTES)?;

    //Part-1
    println!("{}", computer.shortest_path().unwrap());
    //320

    //Part-2
    let file = File::open("inputs/input18.txt")?;
    println!("{}", Computer::first_byte_so_no_escape(file).map(|(x, y)| format!("{},{}", x, y))?);
    //34,40

    Ok(())
}
