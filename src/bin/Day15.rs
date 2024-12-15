use anyhow::Result;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};

trait FromChar {
    fn from_char(c: char) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone)]
enum Object {
    Robot,
    Box(BoxType),
    Wall,
    Water,
}

#[derive(Clone)]
enum BoxType {
    Regular,
    Begin,
    End,
}

impl FromChar for Object {
    fn from_char(c: char) -> Option<Object> {
        match c {
            '@' => Some(Object::Robot),
            'O' => Some(Object::Box(BoxType::Regular)),
            '[' => Some(Object::Box(BoxType::Begin)),
            ']' => Some(Object::Box(BoxType::End)),
            '#' => Some(Object::Wall),
            '.' => Some(Object::Water),
            _ => None,
        }
    }
}

#[derive(Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl FromChar for Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Ocean {
    warehouse: Vec<Vec<Object>>,
    moves: Vec<Move>,
}

impl Display for Ocean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.warehouse {
            for obj in row {
                let symbol = match obj {
                    Object::Robot => '@',
                    Object::Box(BoxType::Regular) => 'O',
                    Object::Box(BoxType::Begin) => '[',
                    Object::Box(BoxType::End) => ']',
                    Object::Wall => '#',
                    Object::Water => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Moves->")?;

        for mv in &self.moves {
            let symbol = match mv {
                Move::Up => 'U',
                Move::Down => 'D',
                Move::Left => 'L',
                Move::Right => 'R',
            };
            write!(f, "{}", symbol)?;
        }

        Ok(())
    }
}

impl Ocean {
    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let mut warehouse = Vec::new();
        let mut moves = Vec::new();
        let mut parsing_warehouse = true;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                parsing_warehouse = false;
                continue;
            }

            if parsing_warehouse {
                let row: Vec<Object> = line.chars().filter_map(Object::from_char).collect();
                warehouse.push(row);
            } else {
                let move_line: Vec<Move> = line.chars().filter_map(Move::from_char).collect();
                moves.extend(move_line);
            }
        }

        Ok(Ocean { warehouse, moves })
    }

    fn get_robot_position(&self) -> Result<(usize, usize)> {
        for (i, row) in self.warehouse.iter().enumerate() {
            for (j, obj) in row.iter().enumerate() {
                if let Object::Robot = obj {
                    return Ok((i, j));
                }
            }
        }

        Err(anyhow::anyhow!("Robot not found"))
    }

    fn move_robot(&mut self) -> Result<()> {
        let (mut robot_row, mut robot_col) = self.get_robot_position()?;

        for action in &self.moves {
            let (dr, dc) = match action {
                Move::Up => (-1, 0),
                Move::Down => (1, 0),
                Move::Left => (0, -1),
                Move::Right => (0, 1),
            };

            let mut boxes_to_move = vec![(robot_row, robot_col)];
            let (mut cr, mut cc) = (robot_row, robot_col);
            let mut can_move = true;

            loop {
                let new_row = (cr as isize + dr) as usize;
                let new_col = (cc as isize + dc) as usize;

                match self.warehouse[new_row][new_col] {
                    Object::Box(BoxType::Regular) => boxes_to_move.push((new_row, new_col)),
                    Object::Wall => {
                        can_move = false;
                        break;
                    }
                    Object::Water => break,
                    _ => unreachable!(),
                }

                cr = new_row;
                cc = new_col;
            }

            if !can_move {
                continue;
            }

            self.warehouse[robot_row][robot_col] = Object::Water;
            self.warehouse[(robot_row as isize + dr) as usize]
                [(robot_col as isize + dc) as usize] = Object::Robot;

            for &(br, bc) in &boxes_to_move[1..] {
                self.warehouse[(br as isize + dr) as usize][(bc as isize + dc) as usize] =
                    Object::Box(BoxType::Regular);
            }

            robot_row = (robot_row as isize + dr) as usize;
            robot_col = (robot_col as isize + dc) as usize;
        }

        Ok(())
    }

    fn part_1(&self) -> usize {
        let mut sum = 0;
        for (i, row) in self.warehouse.iter().enumerate() {
            for (j, obj) in row.iter().enumerate() {
                if let Object::Box(BoxType::Regular) = obj {
                    sum += 100 * i + j; //The GPS coordinate is given by 100*i+j
                }
            }
        }

        sum
    }

    fn wider_warehouse(&self) -> Ocean {
        let mut wider_warehouse = Vec::new();

        for row in &self.warehouse {
            let mut new_row = Vec::new();
            for obj in row {
                match obj {
                    Object::Robot => new_row.extend_from_slice(&[Object::Robot, Object::Water]),
                    Object::Box(BoxType::Regular) => new_row.extend_from_slice(&[
                        Object::Box(BoxType::Begin),
                        Object::Box(BoxType::End),
                    ]),
                    Object::Wall => new_row.extend_from_slice(&[Object::Wall, Object::Wall]),
                    Object::Water => new_row.extend_from_slice(&[Object::Water, Object::Water]),
                    _ => unreachable!(),
                }
            }
            wider_warehouse.push(new_row);
        }

        Ocean {
            warehouse: wider_warehouse,
            moves: self.moves.clone(),
        }
    }
    ///Now, we can move blocks connected to the block touching the robot as well
    fn move_robot_in_wider(&mut self) -> Result<()> {
        let (mut robot_row, mut robot_col) = self.get_robot_position()?;

        for action in &self.moves {
            let (dr, dc) = match action {
                Move::Up => (-1, 0),
                Move::Down => (1, 0),
                Move::Left => (0, -1),
                Move::Right => (0, 1),
            };

            let mut targets = vec![(robot_row, robot_col)];
            let mut can_move = true;

            let mut new_targets = Vec::new();
            for &(cr, cc) in &targets {
                let new_row = (cr as isize + dr) as usize;
                let new_col = (cc as isize + dc) as usize;

                // Bounds check
                if new_row >= self.warehouse.len() || new_col >= self.warehouse[new_row].len() {
                    can_move = false;
                    break;
                }

                let char = &self.warehouse[new_row][new_col];
                match char {
                    Object::Wall => {
                        can_move = false;
                        break;
                    }
                    Object::Box(BoxType::Begin) => {
                        // Ensure bounds for adjacent elements
                        if new_col + 1 < self.warehouse[new_row].len() {
                            new_targets.push((new_row, new_col));
                            new_targets.push((new_row, new_col + 1));
                        } else {
                            can_move = false;
                            break;
                        }
                    }
                    Object::Box(BoxType::End) => {
                        // Ensure bounds for adjacent elements
                        if new_col > 0 {
                            new_targets.push((new_row, new_col));
                            new_targets.push((new_row, new_col - 1));
                        } else {
                            can_move = false;
                            break;
                        }
                    }
                    Object::Water => {}
                    _ => unreachable!(),
                }
            }
            targets.extend(new_targets);

            if !can_move {
                continue;
            }

            let copy = self.warehouse.clone();
            self.warehouse[robot_row][robot_col] = Object::Water;
            self.warehouse[(robot_row as isize + dr) as usize]
                [(robot_col as isize + dc) as usize] = Object::Robot;

            for &(br, bc) in &targets[1..] {
                self.warehouse[br][bc] = Object::Water;
            }

            for &(br, bc) in &targets[1..] {
                let new_br = (br as isize + dr) as usize;
                let new_bc = (bc as isize + dc) as usize;

                // Bounds check
                if new_br < self.warehouse.len() && new_bc < self.warehouse[new_br].len() {
                    self.warehouse[new_br][new_bc] = copy[br][bc].clone();
                }
            }

            robot_row = (robot_row as isize + dr) as usize;
            robot_col = (robot_col as isize + dc) as usize;
        }

        Ok(())
    }


    fn part_2(&self) -> usize {
        let mut sum = 0;
        for (i, row) in self.warehouse.iter().enumerate() {
            for (j, obj) in row.iter().enumerate() {
                if let Object::Box(BoxType::Begin) = obj {
                    sum += 100 * i + j; //The GPS coordinate is given by 100*i+j
                }
            }
        }

        sum
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input15.txt")?;
    let ocean = Ocean::from_file(file)?;

    //Part-1
    let mut ocean_move = ocean.clone();
    ocean_move.move_robot()?;
    println!("{}", ocean_move.part_1());
    //1383666

    //Part-2
    let mut wider_ocean = ocean.wider_warehouse();
    wider_ocean.move_robot_in_wider()?;
    println!("{}", wider_ocean.part_2());
    //1412866
    //TODO- Completed in Python

    Ok(())
}
