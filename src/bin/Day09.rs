use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn get_disk(file: File) -> Result<Vec<Option<u64>>> {
    let reader = io::BufReader::new(file);
    let mut disk = Vec::new();
    let mut id = 0;

    if let Some(Ok(line)) = reader.lines().next() {
        let chars: Vec<char> = line.chars().collect();
        for (i, &char) in chars.iter().enumerate() {
            let times = char
                .to_digit(10)
                .ok_or(anyhow::anyhow!("Should've been a digit"))?;
            let value = if i % 2 == 0 { Some(id) } else { None };
            disk.extend(std::iter::repeat(value).take(times as usize));
            if i % 2 == 0 {
                id += 1;
            }
        }
    }

    Ok(disk)
}

fn part_1(disk: &[Option<u64>]) -> u64 {
    let compacted_disk = {
        //Swap the dots and file blocks
        let mut compacted_disk = disk.to_vec();
        let mut left = 0;
        let mut right = disk.len().saturating_sub(1);

        while left < right {
            while left < right && disk[left].is_some() {
                left += 1;
            }
            while left < right && disk[right].is_none() {
                right -= 1;
            }
            if left < right {
                compacted_disk.swap(left, right);
                left += 1;
                right = right.saturating_sub(1);
            }
        }

        compacted_disk
    };

    compacted_disk
        .iter()
        .enumerate()
        .fold(0, |acc, (position, file_id)| {
            acc + position as u64 * file_id.unwrap_or(0)
        })
}

fn blanks(disk: &[Option<u64>]) -> Vec<(usize, usize)> {
    let mut blanks = Vec::new();
    let mut start = None;

    disk.iter()
        .enumerate()
        .for_each(|(i, &val)| match (start, val) {
            (None, None) => start = Some(i),
            (Some(s), Some(_)) => {
                blanks.push((s, i - s));
                start = None;
            }
            _ => {}
        });

    blanks
}

fn collect_files(disk: &[Option<u64>]) -> HashMap<Option<u64>, (usize, usize)> {
    let mut files = HashMap::new();
    let mut current_id = None;
    let mut start_index = 0;

    for (i, &val) in disk.iter().enumerate() {
        match (current_id, val) {
            (None, Some(id)) => {
                current_id = Some(id);
                start_index = i;
            }
            (Some(id), Some(new_id)) if id != new_id => {
                files.insert(Some(id), (start_index, i - start_index));
                current_id = Some(new_id);
                start_index = i;
            }
            (Some(_), None) => {
                files.insert(current_id, (start_index, i - start_index));
                current_id = None;
            }
            _ => {}
        }
    }

    if let Some(id) = current_id {
        files.insert(Some(id), (start_index, disk.len() - start_index));
    }

    files
}

fn calculate_total(files: &HashMap<Option<u64>, (usize, usize)>) -> u64 {
    files
        .iter()
        .map(|(&fid, &(pos, size))| {
            (pos..pos + size)
                .map(|x| fid.unwrap() * x as u64)
                .sum::<u64>()
        })
        .sum()
}

fn part_2(disk: &[Option<u64>]) -> u64 {
    let mut files = collect_files(disk);
    let mut blank_spaces = blanks(disk);
    let mut file_id = files.keys().filter_map(|&k| k).max().unwrap();

    while file_id > 0 {
        if let Some(&(position, size)) = files.get(&Some(file_id)) {
            for (i, &(start, len)) in blank_spaces.iter().enumerate() {
                if start >= position {
                    blank_spaces = blank_spaces[..i].to_owned();
                    break;
                }

                if size <= len {
                    files.insert(Some(file_id), (start, size));
                    if size == len {
                        blank_spaces.remove(i);
                    } else {
                        blank_spaces[i] = (start + size, len - size);
                    }
                    break;
                }
            }
        }
        file_id -= 1;
    }

    calculate_total(&files)
}

fn main() -> Result<()> {
    let file = File::open("inputs/input09.txt")?;
    let disk = get_disk(file)?;

    //Part-1
    println!("{}", part_1(&disk));
    //6446899523367

    //Part-2
    println!("{}", part_2(&disk));
    //6478232739671

    Ok(())
}
