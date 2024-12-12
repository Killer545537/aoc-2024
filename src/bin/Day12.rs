use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

struct Garden {
    rows: usize,
    columns: usize,
    garden: Vec<Vec<char>>,
}

impl Garden {
    //This makes it available in all the functions without making it a field
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let garden = reader
            .lines()
            .map(|line| Ok(line?.chars().collect()))
            .collect::<Result<Vec<Vec<char>>>>()?;

        let rows = garden.len();
        let columns = garden[0].len();

        Ok(Garden {
            rows,
            columns,
            garden,
        })
    }

    fn is_inside_garden(&self, x: isize, y: isize) -> bool {
        (0..self.rows as isize).contains(&x) && (0..self.columns as isize).contains(&y)
    }

    fn get_regions(&self) -> Vec<HashSet<(usize, usize)>> {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();

        for row in 0..self.rows {
            for col in 0..self.columns {
                if visited.contains(&(row, col)) {
                    continue;
                }

                visited.insert((row, col));

                let mut region = HashSet::new();
                let mut queue = VecDeque::from(vec![(row, col)]);
                let current_crop = self.garden[row][col];
                //BFS
                while let Some((curr_row, curr_col)) = queue.pop_front() {
                    region.insert((curr_row, curr_col));
                    for &(dx, dy) in &Self::DIRECTIONS {
                        let new_row = curr_row as isize + dx;
                        let new_col = curr_col as isize + dy;

                        if self.is_inside_garden(new_row, new_col) {
                            let new_row = new_row as usize;
                            let new_col = new_col as usize;
                            //A region consists of the same crops
                            if self.garden[new_row][new_col] == current_crop
                                && !region.contains(&(new_row, new_col))
                            {
                                region.insert((new_row, new_col));
                                queue.push_back((new_row, new_col));
                            }
                        }
                    }
                }

                visited = visited.union(&region).cloned().collect();
                regions.push(region);
            }
        }

        regions
    }

    fn perimeter(&self, region: &HashSet<(usize, usize)>) -> u32 {
        let mut perimeter = 0;

        for &(row, col) in region {
            perimeter += 4;
            //Perimeter reduces by 1 for each neighbor
            for &(dx, dy) in &Self::DIRECTIONS {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;

                if self.is_inside_garden(new_row, new_col)
                    && region.contains(&(new_row as usize, new_col as usize))
                {
                    perimeter -= 1;
                }
            }
        }

        perimeter
    }

    fn part_1(&self) -> u32 {
        self.get_regions()
            .iter()
            .map(|region| region.len() as u32 * self.perimeter(region))
            .sum()
    }

    fn corners(&self, region: &HashSet<(usize, usize)>) -> u32 {
        region
            .iter()
            .flat_map(|&(r, c)| {
                [(-1, -1), (1, -1), (1, 1), (-1, 1)]
                    .iter()
                    .map(move |&(dr, dc)| (r as isize * 2 + dr, c as isize * 2 + dc))
            })
            .collect::<HashSet<_>>() //This is used collect all possible corner configurations
            .iter()
            .map(|&(cr, cc)| {
                let config: Vec<bool> = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
                    .iter()
                    .map(|&(dr, dc)| {
                        region.contains(&(((cr + dr) / 2) as usize, ((cc + dc) / 2) as usize))
                    })
                    .collect(); //Checks which corners are a part of the region
                //Count the number of corners
                match config.iter().filter(|&&x| x).count() {
                    1 => 1,
                    //If the cells are opposite then they are a corner
                    2 if vec![vec![true, false, true, false], vec![false, true, false, true]].contains(&config)  => 2,
                    //If the cells are adjacent, then it was not a corner
                    3 => 1,
                    _ => 0,
                }
            })
            .sum()
    }

    fn part_2(&self) -> u32 {
        self.get_regions()
            .iter()
            .map(|region| region.len() as u32 * self.corners(region))
            .sum()
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input12.txt")?;
    let garden = Garden::from_file(file)?;

    //Part-1
    println!("{}", garden.part_1());
    //1374934

    //Part-2
    println!("{}", garden.part_2());

    Ok(())
}
