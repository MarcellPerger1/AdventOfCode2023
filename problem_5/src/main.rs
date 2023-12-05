#![allow(unused_imports)]
use itertools::Itertools;
use std::fs;
use std::iter;

fn main() {
    part1();
}
// This file uses u64 throughout as the input has some numbers up to 2^32-1 and I don't want signed/unsigned to become an issue

fn parse_num_list(s: &str) -> impl Iterator<Item = u64> + '_ {
    s.trim()
        .split_whitespace()
        .map(|num_s| num_s.parse::<u64>().expect("item should be u64"))
}

#[derive(Debug, Clone, Copy)]
struct MapLine {
    pub dest_start: u64,
    pub src_start: u64,
    pub range_len: u64,
}
// tuple ctor
impl MapLine {
    pub fn from_tuple((dest_start, src_start, range_len): (u64, u64, u64)) -> MapLine {
        MapLine {
            dest_start,
            src_start,
            range_len,
        }
    }
}
// convenience one-line getter
impl MapLine {
    pub fn get_src_end_excl(&self) -> u64 {
        self.src_start + self.range_len
    }
    pub fn get_dest_end_excl(&self) -> u64 {
        self.dest_start + self.range_len
    }
    pub fn get_src_end_incl(&self) -> u64 {
        self.get_src_end_excl() - 1
    }
    pub fn get_dest_end_incl(&self) -> u64 {
        self.get_dest_end_excl() - 1
    }
    pub fn get_src_range_excl(&self) -> (u64, u64) {
        (self.src_start, self.get_src_end_excl())
    }
    pub fn get_dest_range_excl(&self) -> (u64, u64) {
        (self.dest_start, self.get_dest_end_excl())
    }
    pub fn get_src_range_incl(&self) -> (u64, u64) {
        (self.src_start, self.get_src_end_incl())
    }
    pub fn get_dest_range_incl(&self) -> (u64, u64) {
        (self.dest_start, self.get_dest_end_incl())
    }
}
// apply line
impl MapLine {
    fn src_contains(&self, num: u64) -> bool {
        self.src_start <= num && num <= self.get_src_end_incl()
    }

    fn apply_line(&self, num: u64) -> Option<u64> {
        self.src_contains(num).then(|| {
            let offset = num - self.src_start;
            self.dest_start + offset
        })
    }
}

#[derive(Debug, Clone)]
struct FullMap {
    lines: Vec<MapLine>,
}
impl FullMap {
    fn new(lines: Vec<MapLine>) -> Self {
        FullMap { lines }
    }

    fn apply_map(&self, num: u64) -> u64 {
        // try apply each of them, do first one that succeeds, if none work, keep it same
        self.lines
            .iter()
            .find_map(|ln| ln.apply_line(num))
            .unwrap_or(num)
    }
}

#[derive(Debug, Clone)]
struct MapsData {
    maps: Vec<FullMap>,
}
impl MapsData {
    fn new(maps: Vec<FullMap>) -> Self {
        Self { maps }
    }

    fn apply_maps(&self, num: u64) -> u64 {
        self.maps.iter().fold(num, |prev, mp| mp.apply_map(prev))
    }
}

fn parse_map_line(line: &str) -> MapLine {
    MapLine::from_tuple(
        parse_num_list(line)
            .collect_tuple()
            .expect("Line should contain 3 nums"),
    )
}

fn parse_maps(all_lines: Vec<&str>) -> MapsData {
    // [2..] to exclude 'seeds' and first map name
    MapsData::new(
        all_lines[2..]
            .split(|ln| {
                let is_num_line = ln
                    .chars()
                    .nth(0)
                    .expect("Empty lines should've been filtered out")
                    .is_numeric();
                !is_num_line
            })
            .map(|mp_lines| FullMap::new(mp_lines.iter().map(|ln| parse_map_line(&ln)).collect()))
            .collect_vec(),
    )
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect_vec();
    let seeds_line = lines[0];
    let seeds_v = parse_num_list(
        seeds_line
            .strip_prefix("seeds: ")
            .expect("first line should be seeds:"),
    )
    .collect_vec();
    let maps = parse_maps(lines);
    let seed_loc_v = seeds_v
        .iter()
        .map(|s| (*s, maps.apply_maps(*s)))
        .collect_vec();
    let min_pair = seed_loc_v
        .iter()
        .min_by_key(|(sd, loc)| loc)
        .expect("Should have a min location");
    let min_loc = min_pair.1;
    println!("Part1: {}", min_loc);
}
