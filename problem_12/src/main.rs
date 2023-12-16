use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs, iter};

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Normal,
    Broken,
    Unknown,
}
impl State {
    fn from_char(c: char) -> Self {
        use State::*;
        match c {
            '.' => Normal,
            '#' => Broken,
            '?' => Unknown,
            _ => panic!("Unknown char fo State::from_char"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    states: Vec<State>,
    nums: Vec<usize>,
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
    let (state_s, nums_s) = line
        .split_once(' ')
        .expect("Line should be <springs> <nums>");
    Line {
        states: parse_states(state_s),
        nums: parse_num_list(nums_s).collect_vec(),
    }
}

fn matches_states(expected_v: &[State], actual_v: &[State]) -> bool {
    expected_v.iter().zip_eq(actual_v).all(|(expect, actual)| {
        *expect == State::Unknown || *actual == State::Unknown || *expect == *actual
    })
}

fn get_combs_nolengths(states: &[State]) -> usize {
    // no lengths left so the rest must be normal or unknown (i.e. not broken) = 1
    // or if doesn't match, 0
    if states.iter().all(|s| *s != State::Broken) {
        1
    } else {
        0
    }
}

lazy_static! {
    static ref CMB_CACHE: Mutex<HashMap<(usize, usize), usize>> = Mutex::new(HashMap::new());
}
fn get_combs_cached(states: &[State], lengths: &[usize]) -> usize {
    // using addresses of the slice as keys is rather sketchy
    // but they are all just pointers into a `main`-owned vec
    // so **SHOULDN'T** be Drop'd until `main` exits so should
    // stay equivalent as nothing is being cloned
    // (wait, its all references?? - always has been)
    let state_ptr = states.as_ptr() as usize;
    let lengths_ptr = lengths.as_ptr() as usize;
    if let Some(cached_result) = CMB_CACHE.lock().unwrap().get(&(state_ptr, lengths_ptr)) {
        return *cached_result;
    }
    let result = get_combs(states, lengths);
    CMB_CACHE
        .lock()
        .unwrap()
        .insert((state_ptr, lengths_ptr), result);
    return result;
}

fn get_combs(states: &[State], lengths: &[usize]) -> usize {
    let (&len_curr, lengths_rest) = match lengths.split_first() {
        None => return get_combs_nolengths(states),
        Some(v) => v,
    };
    if len_curr > states.len() {
        // impossible to fulfill so return 0
        return 0;
    }
    if len_curr == states.len() {
        // this means that all (len_curr==states.len()) of them must be broken or unknwon (i.e. not normal) for this to be 1
        // But this will also be 0 if there are more that need to be fulfilled
        if lengths_rest.len() > 0 {
            // has more other than this whcih won't have any states left to satisfy them
            // (this one uses up all the states until the end)
            return 0;
        }
        let r = if states.iter().all(|s| *s != State::Normal) {
            1
        } else {
            0
        };
        return r;
    }
    let actual_states_sl = [State::Broken]
        .repeat(len_curr)
        .into_iter()
        .chain(iter::once(State::Normal))
        .collect_vec();
    // start:        0 ..= last_start
    // end-excl:  size ..= len(states)
    // ==> last_start = len(states) - size
    // ALSO, this can't be later than the fist '#' otherwise that would be unfulfilled
    let first_broken = states
        .iter()
        .find_position(|s| **s == State::Broken)
        .and_then(|(i, _)| Some(i))
        .unwrap_or(usize::MAX); // won't be selected by min
    let starts = 0..=(states.len() - len_curr).min(first_broken);
    let possiblities_it = starts.filter_map(|start| {
        let end_excl_no_trailing_normal = start + len_curr;
        if end_excl_no_trailing_normal == states.len() {
            // this is the case where the pattern itself fits,
            // just the trailing State::Normal is OOB so just don't check for extra normal
            // (there can't be extra Broken's in OOB)
            let end_excl = end_excl_no_trailing_normal;
            let expected_sl = &states[start..end_excl];
            if !matches_states(expected_sl, &[State::Broken].repeat(len_curr)) {
                // Can't place it here as it would break the states_list
                return None;
            }
            // states_rest = []
            // so only return 1 if no more lengths to match (if there are, we have no states to satisfy them so return None)
            if lengths_rest.len() == 0 {
                return Some(1);
            } else {
                return None;
            }
        }
        let end_excl = start + len_curr + 1; // + 1 to include extra normal at end of actual states
        let expected_sl = &states[start..end_excl];
        if !matches_states(expected_sl, &actual_states_sl) {
            // Can't place it here as it would break the states_list
            return None;
        }
        let states_rest = &states[end_excl..];
        Some(get_combs_cached(states_rest, lengths_rest))
    });
    possiblities_it.sum()
}

fn handle_line(line: &Line) -> usize {
    get_combs_cached(&line.states, &line.nums)
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
    let out: usize = lines_v.iter().map(handle_line).sum();
    println!("Part1: {}", out);
}

fn part2() {
    // don't need last run's ptrs
    CMB_CACHE.lock().unwrap().clear();
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect();
    let lines_v = lines.into_iter().map(parse_line).collect_vec();
    let unfolded = lines_v
        .into_iter()
        .map(|ln| {
            let nums = ln.nums.repeat(5);
            let states =
                Itertools::intersperse((0..5).map(|_| ln.states.clone()), vec![State::Unknown])
                    .flatten()
                    .collect_vec();
            Line { nums, states }
        })
        .collect_vec();
    let out: usize = unfolded.iter().map(handle_line).sum();
    let _ = unfolded; // ensure that rust knowns not to Drop `unfolded` until here
    println!("Part1: {}", out);
}
