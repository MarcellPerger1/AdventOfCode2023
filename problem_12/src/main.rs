use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    part1();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Normal,
    Broken,
    Unknown
}
impl State {
    fn from_char(c: char) -> Self {
        use State::*;
        match c {
            '.' => Normal,
            '#' => Broken,
            '?' => Unknown,
            _ => panic!("Unknown char fo State::from_char")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    states: Vec<State>,
    nums: Vec<u32>
}

fn parse_states(state_s: &str) -> Vec<State> {
    state_s.chars().map(State::from_char).collect_vec()
}
fn parse_num_list(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.trim()
        .split(',')
        .map(|num_s| num_s.parse().expect("item should be u64"))
}
fn parse_line(line: &str) -> Line {
    let (state_s, nums_s) = line.split_once(' ').expect("Line should be <springs> <nums>");
    Line { states: parse_states(state_s), nums: parse_num_list(nums_s).collect_vec() }
}

fn matches_states(expected: &Vec<State>, actual: &Vec<State>) -> bool {
    expected.iter().zip(actual).all(|(exp, act)| *exp == State::Unknown || *exp == *act)
}

fn handle_line((states, broken_lens): (Vec<State>, Vec<u64>)) -> u64 {
    // TODO
    
    todo!()
}

fn part1() {
    let contents =
        fs::read_to_string("./src/example.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect();
    let lines_v = lines.into_iter().map(parse_line).collect_vec();

}
