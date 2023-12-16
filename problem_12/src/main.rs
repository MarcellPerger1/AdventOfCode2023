use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, iter};

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
    nums: Vec<usize>
}

fn parse_states(state_s: &str) -> Vec<State> {
    state_s.chars().map(State::from_char).collect_vec()
}
fn parse_num_list(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.trim()
        .split(',')
        .map(|num_s| num_s.parse().expect("item should be u64"))
}
fn parse_line(line: &str) -> Line {
    let (state_s, nums_s) = line.split_once(' ').expect("Line should be <springs> <nums>");
    Line { states: parse_states(state_s), nums: parse_num_list(nums_s).collect_vec() }
}

fn matches_states(expected_v: &[State], actual_v: &[State], dp: usize) -> bool {
    let indent = " ".repeat(dp);
    // println!("{indent}  expect: {:?}\n{indent}  actual: {:?}", expected_v, actual_v);
    expected_v.iter().zip_eq(actual_v).all(|(expect, actual)| *expect == State::Unknown || *actual == State::Unknown || *expect == *actual)
}

fn get_combs_nolengths(states: &[State]) -> usize {
    // no lengths left so the rest must be normal or unknown (i.e. not broken) = 1
    // or if doesn't match, 0
    if states.iter().all(|s| *s != State::Broken) { 1 } else { 0 }
}

fn get_combs(states: &[State], lengths: &[usize], dp: usize) -> usize {
    let indent = " ".repeat(dp);
    // println!("{indent}states: {:?};   lengths: {:?}", states, lengths);
    let (&len_curr, lengths_rest) = match lengths.split_first() {
        None => return get_combs_nolengths(states),
        Some(v) => v
    };
    if len_curr > states.len() {  // impossible to fulfill so return 0
        // println!("{indent}Ret: 0 [LenTooBig]");
        return 0;
    }
    if len_curr == states.len() {
        // this means that all (len_curr==states.len()) of them must be broken or unknwon (i.e. not normal) for this to be 1
        // But this will also be 0 if there are more that need to be fulfilled
        if lengths_rest.len() > 0 {
            // has more other than this whcih won't have any states left to satisfy them
            // (this one uses up all the states until the end)
            // println!("{indent}Ret: 0 [SameLen,HasMore]");
            return 0;
        }
        let r = if states.iter().all(|s| *s != State::Normal) { 1 } else { 0 };
        // println!("{indent}Ret: {} [SameLen]", r);
        return r;
    }
    let actual_states_sl = [State::Broken].repeat(len_curr).into_iter().chain(iter::once(State::Normal)).collect_vec();
    // start:        0 ..= last_start
    // end-excl:  size ..= len(states)
    // ==> last_start = len(states) - size
    // ALSO, this can't be later than the fist '#' otherwise that would be unfulfilled
    let first_broken = states.iter()
        .find_position(|s| **s == State::Broken)
        .and_then(|(i, _)| Some(i))
        .unwrap_or(usize::MAX);  // won't be selected by min
    let starts =  0..=(states.len() - len_curr).min(first_broken);
    // println!("{indent}{:?}", starts);
    let possiblities_it = starts.filter_map(|start| {
        // println!("{indent}start: {}", start);
        let end_excl_no_trailing_normal = start + len_curr;
        if end_excl_no_trailing_normal == states.len() {
            // this is the case where the pattern itself fits, 
            // just the trailing State::Normal is OOB so just don't check for extra normal
            // (there can't be extra Broken's in OOB)
            let end_excl = end_excl_no_trailing_normal;
            let expected_sl = &states[start..end_excl];
            if !matches_states(expected_sl, &[State::Broken].repeat(len_curr), dp) {
                // println!("{indent}  Doesn't match, discard [AtEnd]");
                // Can't place it here as it would break the states_list
                return None;
            }
            // states_rest = []
            // so only return 1 if no more lengths to match (if there are, we have no states to satisfy them so return None)
            if lengths_rest.len() == 0 {
                // println!("{indent}  Matches [AtEnd,NoMore] - return 1 -");
                return Some(1); 
            } else {
                // println!("{indent}  Matches but has more lengths without more states [AtEnd,NeedsMore]");
                return None;
            }
        }
        let end_excl = start + len_curr + 1;  // + 1 to include extra normal at end of actual states
        let expected_sl = &states[start..end_excl];
        if !matches_states(expected_sl, &actual_states_sl, dp) {
            // println!("{indent}  Doesn't match, discard");
            // Can't place it here as it would break the states_list
            return None;
        }
        let states_rest = &states[end_excl..];
        // println!("{indent}  Matches, - goto inner -");
        Some(get_combs(states_rest, lengths_rest, dp+4))
    });
    let r = possiblities_it.sum();
    // println!("{indent}Ret: {}", r);
    r
}

fn handle_line(line: &Line) -> usize {
    let ret = get_combs(&line.states, &line.nums, 0);
    println!("{:?} => {}", line, ret);
    ret
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect();
    let lines_v = lines.into_iter().map(parse_line).collect_vec();
    // let f = handle_line(&lines_v[5]);
    // println!("{}", f);
    // return;
    let out: usize = lines_v.iter().map(handle_line).sum();
    println!("Part1: {}", out);
}
