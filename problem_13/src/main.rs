use itertools::Itertools;
use std::fs;
use std::iter;

fn main() {
    part1();
}

trait SumT<T> : Iterator<Item = T> + Sized {
    fn sumt(self) -> T;
}
impl<I: Iterator> SumT<I::Item> for I where I::Item: num::Zero + iter::Sum {
    fn sumt(self) -> I::Item {
        self.sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Bad char for Tile::from_char")
        }
    }
}

fn try_horiz_symmetry_line(pattern: &Vec<Vec<Tile>>, after_line: usize) -> bool {
    let last_idx = pattern.len() - 1;
    let mut i_first = after_line as isize;
    let mut i_last = (after_line + 1) as isize;
    while i_first >= 0 && i_last <= (last_idx as isize) {
        // this comparison is O(n) - could make it better by using hash table-like stuff
        if pattern[i_first as usize] != pattern[i_last as usize] { return false; }
        i_first -= 1;
        i_last += 1;
    }
    true
}

fn get_horiz_symmetry(pattern: &Vec<Vec<Tile>>) -> u64 {
    // O(n^3) approach - not very good, must be a better way
    // can't have a symmertry after last line
    let last_idx = pattern.len() - 1;
    for after_line in 0..last_idx {
        if try_horiz_symmetry_line(pattern, after_line) {
            // after idx 2=> 0,1,2 before => 3 before
            let n_lines_before = after_line + 1;
            return n_lines_before as u64;
        }
    }
    0
}

fn get_vertical_symmetry(pattern: &Vec<Vec<Tile>>) -> u64 {
    // just use the get_horiz with reversed dimensions
    let transposed_list = (0..pattern[0].len()).map(|j| pattern.iter().map(|ln| ln[j]).collect_vec()).collect_vec();
    get_horiz_symmetry(&transposed_list)
}

fn get_pattern_symmetry(pattern: &Vec<Vec<Tile>>) -> u64 {
    100 * get_horiz_symmetry(pattern) + get_vertical_symmetry(pattern)
}

fn parse_pattern(pattern: &[&str]) -> Vec<Vec<Tile>> {
    pattern.iter().map(|ln| ln.chars().map(Tile::from_char).collect_vec()).collect_vec()
}

pub fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).collect_vec();
    let patterns = lines
        .split(|ln| ln.len() == 0)
        .map(parse_pattern)
        .map(|pattern| get_pattern_symmetry(&pattern));
    let s = patterns.sumt();
    println!("Part 1: {}", s);
}
