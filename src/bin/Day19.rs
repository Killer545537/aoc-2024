use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Stripes {
    White, //w
    Blue,  //u
    Black, //b
    Red,   //r
    Green, //g
}

impl Stripes {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'w' => Some(Stripes::White),
            'u' => Some(Stripes::Blue),
            'b' => Some(Stripes::Black),
            'r' => Some(Stripes::Red),
            'g' => Some(Stripes::Green),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Towel(Vec<Stripes>);

struct HotSpring {
    patterns: HashSet<Towel>,
    towels: Vec<Towel>,
}

impl HotSpring {
    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        let patterns = lines
            .next()
            .context("Missing patterns")??
            .split(", ")
            .map(|pattern| Towel(pattern.chars().filter_map(Stripes::from_char).collect()))
            .collect();

        lines
            .next()
            .context("No line separating patterns from towels")??;

        let towels = lines
            .map(|line| {
                let line = line?;
                Ok(Towel(line.chars().filter_map(Stripes::from_char).collect()))
            })
            .collect::<Result<Vec<Towel>>>()?;

        Ok(HotSpring { patterns, towels })
    }

    ///Checks recursively if a sequence of Stripes can be made from the given stripes
    fn helper_can_be_made(
        &self,
        towel: &[Stripes],
        cache: &mut HashMap<Vec<Stripes>, bool>,
    ) -> bool {
        if towel.is_empty() {
            return true;
        }

        if let Some(&result) = cache.get(towel) {
            return result;
        }

        for pattern in &self.patterns {
            let pattern_len = pattern.0.len();
            //If the towel starts with a valid pattern and the rest of the towel can be made using some patterns
            if towel.starts_with(&pattern.0)
                && self.helper_can_be_made(&towel[pattern_len..], cache)
            {
                cache.insert(towel.to_vec(), true);
                return true;
            }
        }

        cache.insert(towel.to_vec(), false);
        false
    }

    fn can_be_made_using_patterns(&self, towel: &Towel) -> bool {
        let mut cache = HashMap::new();
        self.helper_can_be_made(&towel.0, &mut cache)
    }

    fn part_1(&self) -> usize {
        self.towels
            .iter()
            .filter(|&towel| self.can_be_made_using_patterns(towel))
            .count()
    }

    fn helper_count_ways_to_make(
        &self,
        towel: &[Stripes],
        cache: &mut HashMap<Vec<Stripes>, usize>,
    ) -> usize {
        if towel.is_empty() {
            return 1;
        }

        if let Some(&count) = cache.get(towel) {
            return count;
        }

        let mut count = 0;
        for pattern in &self.patterns {
            let pattern_len = pattern.0.len();
            if towel.starts_with(&pattern.0) {
                count += self.helper_count_ways_to_make(&towel[pattern_len..], cache);
            }
        }

        cache.insert(towel.to_vec(), count);
        count
    }

    fn count_ways_to_make_towel(&self, towel: &Towel) -> usize {
        let mut cache = HashMap::new();
        self.helper_count_ways_to_make(&towel.0, &mut cache)
    }

    fn part_2(&self) -> usize {
        self.towels
            .iter()
            .map(|towel| self.count_ways_to_make_towel(towel))
            .sum()
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input19.txt")?;
    let hot_spring = HotSpring::from_file(file)?;

    //Part-1
    println!("{}", hot_spring.part_1());
    //283

    //Part-2
    println!("{}", hot_spring.part_2());
    //615388132411142

    Ok(())
}
