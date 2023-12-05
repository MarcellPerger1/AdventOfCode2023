#![allow(unused_imports)]
use itertools::Itertools;
use std::fs;
use std::iter;

fn main() {
    part1();
    part2();
}
// This file uses u64 throughout as the input has some numbers up to 2^32-1 and I don't want signed/unsigned to become an issue

#[derive(Debug, Clone)]
struct NumRange {
    pub start: u64,
    pub len: u64,
}
impl NumRange {
    pub fn new(start: u64, len: u64) -> Self {
        Self { start, len }
    }

    pub fn from_excl(start: u64, end_excl: u64) -> Self {
        Self {
            start,
            len: end_excl - start,
        }
    }
    pub fn from_incl(start: u64, end_incl: u64) -> Self {
        Self::from_excl(start, end_incl + 1)
    }

    pub fn end_excl(&self) -> u64 {
        self.start + self.len
    }
    pub fn end_incl(&self) -> u64 {
        self.end_excl() - 1
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let is_disjoint: bool = self.end_incl() < other.start || other.end_incl() < self.start;
        !is_disjoint
    }
}

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
    pub fn get_src_range(&self) -> NumRange {
        NumRange {
            start: self.src_start,
            len: self.range_len,
        }
    }
    pub fn get_dest_range(&self) -> NumRange {
        NumRange {
            start: self.dest_start,
            len: self.range_len,
        }
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

    fn priv_apply_range_contained(&self, r: NumRange) -> NumRange {
        let start_offset = r.start - self.src_start;
        NumRange::new(self.dest_start + start_offset, r.len)
    }

    fn apply_line_r_list(&self, rlist: Vec<NumRange>) -> Vec<NumRange> {
        // TODO: this list thing doesn't work: need to apply them in parallel as it will be applied a 2nd time to the intermediate result
        // e.g. if 10->21 = +15; 20->50: +8 a number in 10->20 has 2 tranformations applied to it which is bad.
        // Solution: do it separately for each line and somehow merge it
        rlist
            .iter()
            .map(|r| self.apply_line_r(r.to_owned()))
            .flatten()
            .collect_vec()
    }

    fn apply_line_r(&self, r1: NumRange) -> Vec<NumRange> {
        self.apply_line_r_inner(r1)
            .into_iter()
            .filter(|r| r.len > 0)
            .collect_vec()
    }

    fn apply_line_r_inner(&self, r1: NumRange) -> Vec<NumRange> {
        let rself = self.get_src_range();
        // no intersect so no change
        if !rself.intersects(&r1) {
            return iter::once(r1).collect();
        }
        // <-- self -->
        //    <-- other -->
        if rself.start <= r1.start
            && r1.start <= rself.end_incl()
            && rself.end_incl() <= r1.end_incl()
        {
            // split other at self.end_incl: left=apply, right, don't
            let left_apply = NumRange::from_incl(r1.start, rself.end_incl());
            let right_same = NumRange::from_incl(rself.end_excl(), r1.end_incl());
            return vec![self.priv_apply_range_contained(left_apply), right_same];
        }
        //      <-- self -->
        //  <-- other -->
        if r1.start <= rself.start
            && rself.start <= r1.end_incl()
            && r1.end_incl() <= rself.end_incl()
        {
            // split other at self.end_incl: left=same, right=apply
            let left_same = NumRange::from_excl(r1.start, rself.start);
            let right_apply = NumRange::from_incl(rself.start, r1.end_incl());
            return vec![left_same, self.priv_apply_range_contained(right_apply)];
        }
        //  <--   self    -->
        //    <-- other -->
        if rself.start <= r1.start && r1.start <= r1.end_incl() && r1.end_incl() <= rself.end_incl()
        {
            // everything is apply
            return vec![self.priv_apply_range_contained(r1)];
        }
        //     <--self-->
        // <--   other   -->
        if r1.start <= rself.start
            && rself.start <= rself.end_incl()
            && rself.end_incl() <= r1.end_incl()
        {
            let left_same = NumRange::from_excl(r1.start, rself.start);
            let mid_apply = NumRange::from_incl(rself.start, rself.end_incl());
            let right_same = NumRange::from_incl(rself.end_excl(), r1.end_incl());
            return vec![
                left_same,
                self.priv_apply_range_contained(mid_apply),
                right_same,
            ];
        }
        unreachable!(); // I hope
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

    fn apply_map_r(&self, rlist: Vec<NumRange>) -> Vec<NumRange> {
        print!("{rlist:#?} ===> ");
        let r = self.lines
            .iter()
            .fold(rlist, |prev, mp_line| mp_line.apply_line_r_list(prev));
        println!("{:#?}", r);
        r
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

    fn apply_maps_r(&self, rlist: Vec<NumRange>) -> Vec<NumRange> {
        self.maps
            .iter()
            .fold(rlist, |prev, mp| mp.apply_map_r(prev))
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
        .min_by_key(|(_sd, loc)| loc)
        .expect("Should have a min location");
    let min_loc = min_pair.1;
    println!("Part1: {}", min_loc);
}

fn parse_seeds_line_part2(ln: &str) -> Vec<NumRange> {
    parse_num_list(
        ln.strip_prefix("seeds: ")
            .expect("first line should be seeds:"),
    )
    .tuples()
    .map(|(start, len)| NumRange::new(start, len))
    .collect_vec()
}

fn part2() {
    let contents =
        fs::read_to_string("./src/example.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect_vec();
    let seeds_line = lines[0];
    let seeds_v = parse_seeds_line_part2(seeds_line);
    let maps = parse_maps(lines);
    let seed_loc_v = seeds_v
        .iter()
        .map(|s| (s.to_owned(), maps.apply_maps_r(vec![s.to_owned()])))
        .collect_vec();
    println!("{:#?}; \n\n {:#?}", seeds_v, seed_loc_v);
    // TODO this will work but only because we don't need orig thing only result
    let min_value = seed_loc_v.iter().map(|(_src, dest)| {
        // start is, by definition, the min of a range
        dest.iter().min_by_key(|r| r.start).expect("Expected non-zero dest").start
    }).min().expect("Should have non-empty seed_loc_v");
    println!("Part2: {}", min_value);
}
