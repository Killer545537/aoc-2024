use anyhow::{Context, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;

trait FromChar {
    fn from_char(c: char) -> Option<Self>
    where
        Self: Sized;
}

#[derive(PartialEq)]
enum Object {
    Start,
    End,
    Road,
    Wall,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match &self {
            Object::Start => 'S',
            Object::End => 'E',
            Object::Road => '.',
            Object::Wall => '#',
        };

        write!(f, "{}", symbol)
    }
}

impl FromChar for Object {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Object::Start),
            'E' => Some(Object::End),
            '.' => Some(Object::Road),
            '#' => Some(Object::Wall),
            _ => None,
        }
    }
}

struct Track {
    track: Vec<Vec<Object>>,
    starting_position: (usize, usize),
    ending_position: (usize, usize),
}

impl Track {
    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let mut track = Vec::new();
        let mut starting_position = None;
        let mut ending_position = None;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if let Some(obj) = Object::from_char(c) {
                    if obj == Object::Start {
                        starting_position = Some((i, j));
                    } else if obj == Object::End {
                        ending_position = Some((i, j));
                    }
                    row.push(obj);
                }
            }
            track.push(row);
        }

        Ok(Track {
            track,
            starting_position: starting_position.context("No starting position found")?,
            ending_position: ending_position.context("No ending position found")?,
        })
    }

    fn lowest_possible_score(&self) -> Result<i32> {
        let mut heap: BinaryHeap<Reverse<(i32, (usize, usize), (isize, isize))>> =
            BinaryHeap::new(); //Min-heap
        let mut visited = HashSet::new();

        let (start_row, start_col) = self.starting_position;
        let (end_row, end_col) = self.ending_position;

        heap.push(Reverse((0, (start_row, start_col), (0, 1))));

        while let Some(Reverse((cost, (row, col), (dr, dc)))) = heap.pop() {
            if (row, col) == (end_row, end_col) {
                return Ok(cost);
            }

            visited.insert((row, col, dr, dc));
            let possible_moves = [
                (
                    cost + 1,
                    (row as isize + dr) as usize,
                    (col as isize + dc) as usize,
                    dr,
                    dc,
                ),
                (cost + 1000, row, col, dc, -dr),
                (cost + 1000, row, col, -dc, dr),
            ];

            for (new_cost, new_row, new_col, new_dr, new_dc) in possible_moves {
                if self.track[new_row][new_col] == Object::Wall
                    || new_row >= self.track.len()
                    || new_col >= self.track[0].len()
                {
                    continue;
                }
                if visited.contains(&(new_row, new_col, new_dr, new_dc)) {
                    continue;
                }
                heap.push(Reverse((new_cost, (new_row, new_col), (new_dr, new_dc))));
            }
        }

        Err(anyhow::anyhow!("No path from start to end"))
    }

    fn number_of_good_seats(&self) -> usize {
        let mut heap: BinaryHeap<Reverse<(i32, (usize, usize), (isize, isize))>> =
            BinaryHeap::new(); // Min-heap
        let mut lowest_cost = HashMap::new();
        let mut back_track = HashMap::new();
        let mut best_cost = i32::MAX;
        let mut end_states = HashSet::new();

        let (start_row, start_col) = self.starting_position;
        let (end_row, end_col) = self.ending_position;
        lowest_cost.insert((start_row, start_col, 0, 1), 0);
        heap.push(Reverse((0, (start_row, start_col), (0, 1))));

        while let Some(Reverse((cost, (row, col), (dr, dc)))) = heap.pop() {
            if cost > *lowest_cost.get(&(row, col, dr, dc)).unwrap_or(&i32::MAX) {
                continue;
            }

            if (row, col) == (end_row, end_col) {
                if cost > best_cost {
                    break;
                }
                best_cost = cost;
                end_states.insert((row, col, dr, dc));
            }

            let possible_moves = [
                (
                    cost + 1,
                    (row as isize + dr) as usize,
                    (col as isize + dc) as usize,
                    dr,
                    dc,
                ),
                (cost + 1000, row, col, dc, -dr),
                (cost + 1000, row, col, -dc, dr),
            ];
            for (new_cost, new_row, new_col, new_dr, new_dc) in possible_moves {
                if new_row >= self.track.len()
                    || new_col >= self.track[0].len()
                    || self.track[new_row][new_col] == Object::Wall
                {
                    continue;
                }

                let lowest = *lowest_cost
                    .get(&(new_row, new_col, new_dr, new_dc))
                    .unwrap_or(&i32::MAX);

                if new_cost > lowest {
                    continue;
                }

                if new_cost < lowest {
                    back_track
                        .entry((new_row, new_col, new_dr, new_dc))
                        .and_modify(|ways| *ways = HashSet::new());
                    lowest_cost.insert((new_row, new_col, new_dr, new_dc), new_cost);
                }

                back_track
                    .entry((new_row, new_col, new_dr, new_dc))
                    .and_modify(|ways| {
                        ways.insert((row, col, dr, dc));
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert((row, col, dr, dc));
                        set
                    });

                heap.push(Reverse((new_cost, (new_row, new_col), (new_dr, new_dc))));
            }
        }

        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        for end_state in &end_states {
            queue.push_back((vec![(end_state.0, end_state.1)], *end_state));
        }

        while let Some((mut path, state)) = queue.pop_front() {
            if state.0 == start_row && state.1 == start_col {
                path.reverse();
                paths.push(path);
                continue;
            }

            if let Some(prev_states) = back_track.get(&state) {
                for &prev_state in prev_states {
                    let mut new_path = path.clone();
                    new_path.push((prev_state.0, prev_state.1));
                    queue.push_back((new_path, prev_state));
                }
            }
        }

        paths.iter().flatten().cloned().collect::<HashSet<(usize, usize)>>().len()
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input16.txt")?;
    let track = Track::from_file(file)?;

    //Part-1
    println!("{}", track.lowest_possible_score()?);
    //103512

    //Part-2
    println!("{}", track.number_of_good_seats());
    //554

    Ok(())
}
