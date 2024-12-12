use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Given height cannot be a trailhead!!")]
struct InvalidTrailHead;

struct Map {
    rows: usize,
    columns: usize,
    map: Vec<Vec<u32>>,
    trailheads: HashSet<(usize, usize)>,
}

impl Map {
    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let map: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                Ok(line?
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>())
            })
            .collect::<Result<Vec<Vec<u32>>>>()?;

        let rows = map.len();
        let columns = map[0].len();
        //Find the locations of trailheads (locations where height is 0)
        let trailheads: HashSet<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &height)| (height == 0).then(|| (i, j)))
            })
            .collect();

        Ok(Map {
            rows,
            columns,
            map,
            trailheads,
        })
    }

    fn is_within_map(&self, x: isize, y: isize) -> bool {
        (0..self.rows as isize).contains(&x) && (0..self.columns as isize).contains(&y)
    }
    ///This is the number of summits that any trailhead can reach
    fn find_score_of_trailhead(&self, i: usize, j: usize) -> Result<usize, InvalidTrailHead> {
        if !self.trailheads.contains(&(i, j)) {
            return Err(InvalidTrailHead);
        }

        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        //A summit is the opposite of a trailhead (where height = 9)
        let mut summits = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        //BFS
        queue.push_back((i, j));
        while let Some((x, y)) = queue.pop_front() {
            visited.insert((i, j));

            if self.map[x][y] == 9 {
                summits.insert((x, y));
            }

            for &(dx, dy) in &DIRECTIONS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if self.is_within_map(nx, ny) && !visited.contains(&(nx as usize, ny as usize)) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if self.map[nx][ny] == self.map[x][y] + 1 {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        Ok(summits.len())
    }

    fn part_1(&self) -> usize {
        self.trailheads
            .iter()
            .flat_map(|&(i, j)| self.find_score_of_trailhead(i, j))
            .sum()
    }

    fn dfs(
        &self,
        x: usize,
        y: usize,
        visited: &mut HashSet<(usize, usize)>,
        directions: &[(isize, isize)],
    ) -> usize {
        if visited.contains(&(x, y)) {
            return 0;
        }

        visited.insert((x, y));

        if self.map[x][y] == 9 {
            visited.remove(&(x, y));
            return 1;
        }

        let mut paths = 0;
        for &(dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if self.is_within_map(nx, ny) {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.map[nx][ny] == self.map[x][y] + 1 {
                    paths += self.dfs(nx, ny, visited, directions);
                }
            }
        }

        visited.remove(&(x, y));
        paths
    }

    fn find_rating_of_trailhead(&self, i: usize, j: usize) -> Result<usize, InvalidTrailHead> {
        if !self.trailheads.contains(&(i, j)) {
            return Err(InvalidTrailHead);
        }

        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut visited = HashSet::new();

        Ok(self.dfs(i, j, &mut visited, &DIRECTIONS))
    }

    fn part_2(&self) -> usize {
        self.trailheads
            .iter()
            .flat_map(|&(i, j)| self.find_rating_of_trailhead(i, j))
            .sum()
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input10.txt")?;
    let map = Map::from_file(file)?;

    //Part-1
    println!("{}", map.part_1());
    //688

    //Part-2
    println!("{}", map.part_2());
    //1459

    Ok(())
}