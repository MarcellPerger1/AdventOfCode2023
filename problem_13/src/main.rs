use itertools::Itertools;
use std::fs;
use std::iter;

fn main() {
    part1();
    part2();
}

trait SumT<T> : Iterator<Item = T> + Sized {
    fn sumt(self) -> T;
}
impl<I: Iterator> SumT<I::Item> for I where I::Item: iter::Sum {
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

    fn opp(self) -> Self {
        use Tile::*;
        match self {
            Ash => Rock,
            Rock => Ash
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

fn get_horiz_symmetry(pattern: &Vec<Vec<Tile>>, ignore_after: Option<usize>) -> usize {
    // O(n^3) approach - not very good, must be a better way
    // can't have a symmertry after last line
    let last_idx = pattern.len() - 1;
    for after_line in 0..last_idx {
        if ignore_after == Some(after_line) { continue; }
        if try_horiz_symmetry_line(pattern, after_line) {
            // after idx 2=> 0,1,2 before => 3 before
            let n_lines_before = after_line + 1;
            return n_lines_before as usize;
        }
    }
    0
}

fn get_vertical_symmetry(pattern: &Vec<Vec<Tile>>, ignore_after: Option<usize>) -> usize {
    // just use the get_horiz with reversed dimensions
    let transposed_list = (0..pattern[0].len()).map(|j| pattern.iter().map(|ln| ln[j]).collect_vec()).collect_vec();
    get_horiz_symmetry(&transposed_list, ignore_after)
}

fn get_pattern_symmetry_tup(pattern: &Vec<Vec<Tile>>, ignore_after: (Option<usize>, Option<usize>)) -> (usize, usize) {
    (get_horiz_symmetry(pattern, ignore_after.0), get_vertical_symmetry(pattern, ignore_after.1))
}

fn get_pattern_symmetry(pattern: &Vec<Vec<Tile>>) -> usize {
    let (horiz, vert) = get_pattern_symmetry_tup(pattern, (None, None));
    100 * horiz + vert
}

fn parse_pattern(pattern: &[&str]) -> Vec<Vec<Tile>> {
    pattern.iter().map(|ln| ln.chars().map(Tile::from_char).collect_vec()).collect_vec()
}

pub fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).collect_vec();
    let patterns_result = lines
        .split(|ln| ln.len() == 0)
        .map(parse_pattern)
        .map(|pattern| get_pattern_symmetry(&pattern));
    let s = patterns_result.sumt();
    println!("Part 1: {}", s);
}

fn has_new_in_dirn(new_value: usize, old_value: usize) -> bool {
    return new_value != 0 && new_value != old_value;
}

fn get_symmetry_2(pattern: &Vec<Vec<Tile>>) -> usize {
    // VERY bad slution, O(n^5) !!!
    let orig_value = get_pattern_symmetry_tup(pattern, (None, None));
    let ignore_value = (
        // 1-based to zero-based
        (orig_value.0 != 0).then(|| orig_value.0 - 1),
        (orig_value.1 != 0).then(|| orig_value.1 - 1));
    let mut pat_mut = pattern.clone();
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let orig_tile = pat_mut[i][j];
            pat_mut[i][j] = orig_tile.opp();
            let new_value = get_pattern_symmetry_tup(&pat_mut, ignore_value);
            if has_new_in_dirn(new_value.0, orig_value.0) {
                return 100 * new_value.0; 
            }
            if has_new_in_dirn(new_value.1, orig_value.1) {
                return new_value.1; 
            }
            // reset values & try again
            pat_mut[i][j] = orig_tile;
        }
    }
    panic!("No 2nd line of symmetry")
}

pub fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).collect_vec();
    let patterns = lines
        .split(|ln| ln.len() == 0)
        .map(parse_pattern).collect_vec();
    let patterns_out = patterns.into_iter().map(|pattern| get_symmetry_2(&pattern));
    let s = patterns_out.sumt();
    println!("Part 2: {}", s);
}
