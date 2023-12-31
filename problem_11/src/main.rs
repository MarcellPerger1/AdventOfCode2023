use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Galaxy,
}
impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Bad char for Tile"),
        }
    }
    fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Galaxy => '#',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    lni: usize,
    xi: usize,
}
impl Pos {
    fn from_ln_x(lni: usize, xi: usize) -> Self {
        Self { lni, xi }
    }

    fn index_in(self, grid: &Vec<Vec<Tile>>) -> Tile {
        grid[self.lni][self.xi]
    }
}

fn main() {
    part1();
    part2();
}

fn manhattan_dist(a: Pos, b: Pos) -> usize {
    // x dist + ln dist
    a.lni.abs_diff(b.lni) + a.xi.abs_diff(b.xi)
}

// Debugging funcs
#[allow(dead_code)]
fn fmt_grid(grid: &Vec<Vec<Tile>>) -> String {
    grid.iter()
        .map(|ln| ln.iter().map(|t| t.to_char()).join(""))
        .join("\n")
}
#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Tile>>) {
    println!("{}", fmt_grid(grid));
}

fn duplicate_empty_rows(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    grid.into_iter()
        .flat_map(|ln| {
            if ln.iter().all(|t| *t == Tile::Empty) {
                vec![ln.clone(), ln]
            } else {
                vec![ln]
            }
        })
        .collect_vec()
}
fn duplicate_empty_cols(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let gsize = Pos::from_ln_x(grid.len(), grid[0].len());
    // duplicate emtpy columns
    let empty_cols = (0..gsize.xi)
        .filter(|xi| {
            (0..gsize.lni).all(|lni| Pos::from_ln_x(lni, *xi).index_in(&grid) == Tile::Empty)
        })
        .collect::<HashSet<_>>();
    grid.into_iter()
        .map(|ln| {
            ln.into_iter()
                .enumerate()
                .flat_map(|(i, t)| {
                    if empty_cols.contains(&i) {
                        vec![t, t]
                    } else {
                        vec![t]
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect();
    let grid = lines
        .iter()
        .map(|ln| ln.chars().map(|c| Tile::from_char(c)).collect_vec())
        .collect_vec();
    let grid = duplicate_empty_rows(grid);
    let grid = duplicate_empty_cols(grid);

    let posn_list = grid
        .into_iter()
        .enumerate()
        .flat_map(|(lni, ln)| {
            ln.into_iter()
                .enumerate()
                .filter_map(|(xi, t)| (t == Tile::Galaxy).then_some(Pos::from_ln_x(lni, xi)))
                .collect_vec()
        })
        .collect_vec();
    let posn_pairs = posn_list.iter().cartesian_product(posn_list.iter());
    // divide by 2 as each pair counted twice
    let s = posn_pairs
        .map(|(apos, bpos)| manhattan_dist(*apos, *bpos))
        .sum::<usize>()
        / 2;
    println!("Part 1: {}", s);
}

fn find_empty_rows(grid: &Vec<Vec<Tile>>) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(lni, ln)| ln.iter().all(|t| *t == Tile::Empty).then_some(lni))
        .collect()
}
fn find_empty_cols(grid: &Vec<Vec<Tile>>) -> Vec<usize> {
    let gsize_xi = grid[0].len();
    (0..gsize_xi)
        .filter_map(|xi| grid.iter().all(|ln| ln[xi] == Tile::Empty).then_some(xi))
        .collect()
}

fn manhattan_dist_expanded(
    a: Pos,
    b: Pos,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> usize {
    let lo = Pos::from_ln_x(a.lni.min(b.lni), a.xi.min(b.xi));
    let hi = Pos::from_ln_x(a.lni.max(b.lni), a.xi.max(b.xi));
    let lni_range = lo.lni..=hi.lni;
    let xi_range = lo.xi..=hi.xi;
    let n_expanded_rows = empty_rows
        .iter()
        .filter(|lni| lni_range.contains(lni))
        .count();
    let n_expanded_cols = empty_cols.iter().filter(|xi| xi_range.contains(xi)).count();
    // each expanded row/col adds extra (EXPANSION_SIZE - 1) onto the manhattan distance
    // so output = non_expanded + expanded * EXPANSION_SIZE
    //           = non_expanded + expanded + expanded * (EXPANSION_SIZE - 1)
    //           =    manhattan_dist       + expanded * (EXPANSION_SIZE - 1)
    let n_expanded = n_expanded_cols + n_expanded_rows;
    manhattan_dist(a, b) + n_expanded * (EXPANSION_SIZE - 1)
}

const EXPANSION_SIZE: usize = 1_000_000;
fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect();
    let grid = lines
        .iter()
        .map(|ln| ln.chars().map(|c| Tile::from_char(c)).collect_vec())
        .collect_vec();

    let empty_rows = find_empty_rows(&grid);
    let empty_cols = find_empty_cols(&grid);

    let posn_list = grid
        .into_iter()
        .enumerate()
        .flat_map(|(lni, ln)| {
            ln.into_iter()
                .enumerate()
                .filter_map(|(xi, t)| (t == Tile::Galaxy).then_some(Pos::from_ln_x(lni, xi)))
                .collect_vec()
        })
        .collect_vec();
    let posn_pairs = posn_list.iter().cartesian_product(posn_list.iter());

    let s = posn_pairs
        .map(|(apos, bpos)| manhattan_dist_expanded(*apos, *bpos, &empty_rows, &empty_cols))
        .sum::<usize>()
        / 2; // divide by 2 as each pair counted twice
    println!("Part 2: {}", s);
}
