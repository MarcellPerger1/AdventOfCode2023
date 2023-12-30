use itertools::Itertools;
use std::iter;
use std::fs;

fn main() {
    part1();
}

trait SumT<T> : Iterator<Item = T> + Sized {
    fn sumt(self) -> T;
}
impl<I: Iterator> SumT<I::Item> for I where I::Item: iter::Sum {
    fn sumt(self) -> I::Item {
        self.sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty, Stationary, Moving
}
impl Tile {
    fn from_char(c: char) -> Self {
        use Tile::*;
        match c {
            '.' => Empty,
            '#' => Stationary,
            'O' => Moving,
            _ => panic!("Bad char for Tile::from_char")
        }
    }

    fn weight(self) -> usize {
        match self {
            Tile::Empty => 0,
            Tile::Stationary => 0,
            Tile::Moving => 1,
        }
    }
}

fn parse_lines(lines: &Vec<&str>) -> Vec<Vec<Tile>> {
    lines.iter().map(|ln| ln.chars().map(Tile::from_char).collect()).collect()
}

fn grid_to_columns(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    (0..grid[0].len()).map(|j| grid.iter().map(|ln| ln[j]).collect_vec()).collect_vec()
}

fn fall_to_start(col: &[Tile]) -> Vec<Tile> {
    let mut out = col.iter().enumerate().fold(Vec::with_capacity(col.len()), |mut prev, (i, tile)| {
        // the value passed to next iteration is the reversed of the column 
        // - much more practical to have stuff 'fall' towards the end of the list
        match tile {
            Tile::Empty => prev,  // empty are removed and filled back in before a Tile::stationary
            Tile::Moving => {
                assert_ne!(prev.last(), Some(&Tile::Empty));
                prev.push(Tile::Moving);
                prev
            },
            Tile::Stationary => {
                // need to add on the air out of which the empties fell
                // This is Stationary so we know that the idx MUST be the same as before
                // so add enough Tile::Empty so that the last idx is `i - 1` so that
                // when the current one is added, its idx will be `i`
                // if new_len is `i`, the idx after the last (where the Stationary will go) is `i`
                prev.resize(i, Tile::Empty);
                prev.push(Tile::Stationary);
                assert_eq!(prev.len(), i + 1);
                prev
            },
        }
    });
    out.resize(col.len(), Tile::Empty);
    out
}

fn fall_north_col(col: &[Tile]) -> Vec<Tile> {
    fall_to_start(col)
}

fn fall_north(cols: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    cols.iter().map(|col| fall_north_col(col.as_slice())).collect_vec()
}

fn get_load_col(col: &[Tile]) -> usize {
    let height = col.len();
    let get_w_mult = |i: usize| -> usize {
        height - i
    };
    col.iter().enumerate().map(|(i, t)| get_w_mult(i) * t.weight()).sumt()
}

fn get_load(cols: &Vec<Vec<Tile>>) -> usize {
    cols.iter().map(|col| get_load_col(col)).sumt()
}

pub fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0).collect_vec();
    let grid = parse_lines(&lines);
    let cols = grid_to_columns(&grid);
    let fallen_cols = fall_north(&cols);
    let w = get_load(&fallen_cols);
    println!("Part 1: {}", w);
}
