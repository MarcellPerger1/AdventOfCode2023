use itertools::Itertools;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    part1();
    part2();
}


fn parse_ws_list<T: FromStr>(s: &str) -> impl Iterator<Item = T> + '_ where T::Err: Debug {
    s.trim()
        .split_whitespace()
        .map(|num_s| num_s.parse().expect("item should be u64"))
}

fn find_1st_diff(ln: &Vec<i64>) -> Vec<i64> {
    ln.iter().tuple_windows().map(|(a, b)| {
        b - a
    }).collect()
}

fn handle_line_vec(ln: &Vec<i64>) -> i64 {
    if ln.iter().all(|x| *x == 0) {
        // BASE CASE: this line is all zeroes therefore the next one is just a 0
        return 0;
    }

    let diff_v = find_1st_diff(ln);
    let next_diff = handle_line_vec(&diff_v);
    // ...        ln.last      next
    //    diffv.last   next_diff

    // next_diff = next - ln.last
    // => next = ln.last + next_diff
    let next = next_diff + ln.last().expect("Expected non-empty line");
    next
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0);
    let parsed_lines: Vec<Vec<i64>> = lines.map(|ln| parse_ws_list(ln).collect()).collect();
    let next_values = parsed_lines.into_iter().map(|ln| {
        handle_line_vec(&ln)
    }).collect_vec();
    // println!("Next values: {:?}", next_values);
    println!("Part 1: {}", next_values.iter().sum::<i64>());
}

// same as above (hopefully) just reverse it
fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0);
    let parsed_lines: Vec<Vec<i64>> = lines.map(|ln| parse_ws_list(ln).collect()).collect();
    let next_values = parsed_lines.into_iter().map(|ln| {
        handle_line_vec(&ln.into_iter().rev().collect_vec())
    }).collect_vec();
    // println!("Next values: {:?}", next_values);
    println!("Part 2: {}", next_values.iter().sum::<i64>());
}
