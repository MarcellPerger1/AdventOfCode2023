#![allow(unused_imports)]
use std::fs;
use std::iter;
use itertools::Itertools;

fn main() {
    part1();
}

fn part1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let _ = lines;  // TODO implement solution
}
