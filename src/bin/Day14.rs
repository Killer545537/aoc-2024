use anyhow::{Context, Result};
use std::cell::Cell;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Mul;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn add_velocity_wrapping(&self, velocity: &Velocity, rows: i32, cols: i32) -> Position {
        Position {
            x: (self.x + velocity.v_x).rem_euclid(cols),
            y: (self.y + velocity.v_y).rem_euclid(rows),
        }
    }
}

#[derive(Clone)]
struct Velocity {
    v_x: i32,
    v_y: i32,
}

impl Mul<i32> for &Velocity {
    type Output = Velocity;

    fn mul(self, factor: i32) -> Self::Output {
        Velocity {
            v_x: self.v_x * factor,
            v_y: self.v_y * factor,
        }
    }
}

#[derive(Clone)]
struct Robot {
    position: Cell<Position>,
    velocity: Velocity,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let position_part = parts.get(0).context("No position in input")?;
        let velocity_part = parts.get(1).context("No velocity in input")?;
        let position_values: Vec<i32> = position_part[2..]
            .split(',')
            .map(|v| v.parse().context("Invalid position data"))
            .collect::<Result<Vec<_>>>()?;
        let velocity_values: Vec<i32> = velocity_part[2..]
            .split(',')
            .map(|v| v.parse().context("Invalid velocity data"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Robot {
            position: Cell::new(Position {
                x: position_values[0],
                y: position_values[1],
            }),
            velocity: Velocity {
                v_x: velocity_values[0],
                v_y: velocity_values[1],
            },
        })
    }
}

impl Robot {
    fn move_by(&self, t: i32, rows: i32, cols: i32) {
        let scaled_velocity = &self.velocity * t;
        let new_position = self
            .position
            .get()
            .add_velocity_wrapping(&scaled_velocity, rows, cols);
        self.position.set(new_position);
    }
}

#[derive(Clone)]
struct Lab<const ROWS: usize, const COLS: usize> {
    robots: Vec<Robot>,
}

impl<const ROWS: usize, const COLS: usize> FromIterator<Robot> for Lab<ROWS, COLS> {
    fn from_iter<T: IntoIterator<Item = Robot>>(iter: T) -> Self {
        let robots: Vec<Robot> = iter.into_iter().collect();
        Lab { robots }
    }
}

impl<const ROWS: usize, const COLS: usize> Lab<ROWS, COLS> {
    fn new(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);

        reader
            .lines()
            .map(|line| Robot::from_str(&*line?))
            .collect()
    }

    fn move_robots_by(&self, duration: i32) {
        for robot in &self.robots {
            robot.move_by(duration, ROWS as i32, COLS as i32);
        }
    }

    fn count_robots_in_quadrants(&self) -> (u32, u32, u32, u32) {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        let rows_half = ROWS as i32 / 2;
        let cols_half = COLS as i32 / 2;

        for robot in &self.robots {
            let position = robot.position.get();

            if position.x == cols_half || position.y == rows_half {
                continue;
            }

            if position.x < cols_half && position.y < rows_half {
                q1 += 1; // Top-left
            } else if position.x < cols_half && position.y > rows_half {
                q2 += 1; // Bottom-left
            } else if position.x > cols_half && position.y < rows_half {
                q3 += 1; // Top-right
            } else if position.x > cols_half && position.y > rows_half {
                q4 += 1; // Bottom-right
            }
        }

        (q1, q2, q3, q4)
    }
    ///Finding the safety factor
    fn safety_factor(&self) -> u32 {
        let (q1, q2, q3, q4) = self.count_robots_in_quadrants();

        q1 * q2 * q3 * q4
    }

    fn safety_factors_over_time(&self, time: usize) -> Vec<u32> {
        let mut safety_factors = Vec::with_capacity(time);
        for _ in 0..time {
            let safety_factor = self.safety_factor();
            safety_factors.push(safety_factor);
            self.move_robots_by(1);
        }

        safety_factors
    }

    ///IDK SHIT ABOUT THIS
    fn part_2(&self) -> Option<usize> {
        self.safety_factors_over_time(ROWS * COLS)
            .iter()
            .enumerate()
            .min_by_key(|&(_, factor)| factor)
            .map(|(ind, _)| ind)
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input14.txt")?;
    let lab: Lab<103, 101> = Lab::new(file)?;

    //Part-1
    let part_1_lab = lab.clone();
    println!("{}", part_1_lab.safety_factor());
    //218295000

    //Part-2
    println!("{}", lab.part_2().unwrap());
    //6870

    Ok(())
}
