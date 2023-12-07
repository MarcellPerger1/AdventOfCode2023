#![allow(unused_imports)]  // TODO: remove this after it's complete
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::iter;
use std::str::FromStr;
use counter::Counter;

pub fn part2() {
    let contents = fs::read_to_string("./src/example.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let _ = lines;  // TODO: implement part 2
}
