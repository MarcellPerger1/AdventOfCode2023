#![allow(unused_imports)]  // TODO: remove this after it's complete
use itertools::Itertools;
use std::fs;
use std::iter;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let _ = lines;  // TODO: implement part 1
}

fn part2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let _ = lines;  // TODO: implement part 2
}
