use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum CellType {
    Guard,
    Obstacle,
    Empty,
}

#[derive(Debug)]
struct Lab {
    rows: usize,
    columns: usize,
    lab: Vec<Vec<CellType>>,
}

impl Lab {
    fn get_matrix(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);

        let lab = reader
            .lines()
            .map(|line| {
                line?
                    .chars()
                    .map(|c| match c {
                        '^' => Ok(CellType::Guard),
                        '#' => Ok(CellType::Obstacle),
                        '.' => Ok(CellType::Empty),
                        _ => Err(anyhow::anyhow!("Unexpected char type {}", c)),
                    })
                    .collect::<Result<Vec<CellType>>>()
            })
            .collect::<Result<Vec<Vec<CellType>>>>()?;

        let rows = lab.len();
        let columns = lab[0].len();

        Ok(Lab { rows, columns, lab })
    }

    fn guard(&self) -> Result<(usize, usize)> {
        for i in 0..self.rows {
            for j in 0..self.columns {
                if let CellType::Guard = self.lab[i][j] {
                    return Ok((i, j));
                }
            }
        }

        Err(anyhow::anyhow!("No guard found in the lab"))
    }

    fn is_out_of_free(&self, x: isize, y: isize) -> bool {
        x < 0 || x >= self.rows as isize || y < 0 || y >= self.columns as isize
    }

    fn rotate_right(&self, direction: (isize, isize)) -> (isize, isize) {
        match direction {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => unreachable!(),
        }
    }

    ///Part-1
    fn count_guard_walk(&self) -> Result<(HashSet<(usize, usize)>, usize)> {
        let (mut x, mut y) = self.guard()?;
        let mut direction = (-1, 0); // Start moving up
        let mut vis = HashSet::new();
        vis.insert((x, y));

        loop {
            let (dx, dy) = direction;
            let (new_x, new_y) = (x as isize + dx, y as isize + dy);

            if self.is_out_of_free(new_x, new_y) {
                break;
            }

            match self.lab[new_x as usize][new_y as usize] {
                CellType::Obstacle => {
                    // Rotate right until a valid direction is found
                    direction = self.rotate_right(direction);
                }
                _ => {
                    x = new_x as usize;
                    y = new_y as usize;
                    vis.insert((x, y));
                }
            }
        }

        let cells_covered = vis.len();
        Ok((vis, cells_covered))
    }

    fn is_loop(&self) -> Result<bool> {
        let (mut x, mut y) = self.guard()?;
        let mut direction = (-1, 0);
        let mut vis = HashSet::new();

        loop {
            if !vis.insert(((x, y), direction)) {
                return Ok(true);
            }

            let (dx, dy) = direction;
            let (new_x, new_y) = (x as isize + dx, y as isize + dy);

            if self.is_out_of_free(new_x, new_y) {
                break;
            }

            match self.lab[new_x as usize][new_y as usize] {
                CellType::Obstacle => {
                    // Rotate right until a valid direction is found
                    direction = self.rotate_right(direction);
                }
                _ => {
                    x = new_x as usize;
                    y = new_y as usize;
                }
            }
        }

        Ok(false)
    }
    ///Part-2
    fn count_multiverses_with_loops(&mut self) -> Result<usize> {
        //Check only the cells in the guard's path since the other cells cannot be visited
        let path = self.count_guard_walk()?.0;
        let guard = self.guard()?;
        let mut count = 0;

        for &(i, j) in &path {
            if (i, j) == guard {
                continue;
            }

            self.lab[i][j] = CellType::Obstacle;

            if self.is_loop()? {
                count += 1;
            }

            self.lab[i][j] = CellType::Empty;
        }

        Ok(count)
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input06.txt")?;

    let mut lab = Lab::get_matrix(file)?;
    //Part-1
    println!("{}", lab.count_guard_walk()?.1);
    //5101

    //Part-2
    println!("{}", lab.count_multiverses_with_loops()?);
    //1951

    Ok(())
}
