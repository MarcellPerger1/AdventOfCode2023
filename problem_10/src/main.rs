use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::iter;

fn main() {
    part1();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dirn {
    N, E, S, W
}
impl Dirn {
    fn opp(self) -> Self {
        use Dirn::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    PipeVert,
    PipeHoriz,
    PipeNE,
    PipeNW,
    PipeSW,
    PipeSE,
    Nothing,
    Start
}
impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::PipeVert,
            '-' => Self::PipeHoriz,
            'L' => Self::PipeNE,
            'J' => Self::PipeNW,
            '7' => Self::PipeSW,
            'F' => Self::PipeSE,
            '.' => Self::Nothing,
            'S' => Self::Start,
            _ => panic!("Bad char value to TileType::from_char")
        }
    }

    #[inline]
    fn get_connector_pair(self) -> Option<[Dirn; 2]> {
        use Dirn::*;
        match self {
            Self::PipeVert => Some([N, S]),
            Self::PipeHoriz => Some([E, W]),
            Self::PipeNW => Some([N, W]),
            Self::PipeNE => Some([N, E]),
            Self::PipeSW => Some([S, W]),
            Self::PipeSE => Some([S, E]),
            Self::Nothing | Self::Start => None
        }
    }

    fn has_connector(self, dirn: Dirn) -> bool {
        match self {
            Self::Nothing => false,
            Self::Start => true,
            _ => self.get_connector_pair().unwrap().contains(&dirn)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    lni: usize,
    xi: usize
}
impl Pos {
    fn from_ln_x(lni: usize, xi: usize) -> Self {
        Self { lni, xi }
    }

    fn index_in(self, grid: &Vec<Vec<TileType>>) -> TileType {
        grid[self.lni][self.xi]
    }

    fn get_adj(self, last_pos_excl: Pos) -> Vec<Pos> {
        let last_pos_incl = Pos::from_ln_x(last_pos_excl.lni - 1, last_pos_excl.xi - 1);
        let mut out: Vec<Pos> = Vec::with_capacity(4);
        if self.lni > 0 {
            out.push(Pos::from_ln_x(self.lni - 1, self.xi));
        }
        if self.lni < last_pos_incl.lni {  // if this isn't last
            out.push(Pos::from_ln_x(self.lni + 1, self.xi))
        }
        if self.xi > 0 {
            out.push(Pos::from_ln_x(self.lni, self.xi -1 ));
        }
        if self.xi < last_pos_incl.xi {  // if this isn't last
            out.push(Pos::from_ln_x(self.lni, self.xi +1 ))
        }
        out
    }
    fn get_adj_and_dirn(self, last_pos_excl: Pos) -> Vec<(Dirn, Pos)> {
        let last_pos_incl = Pos::from_ln_x(last_pos_excl.lni - 1, last_pos_excl.xi - 1);
        let mut out: Vec<(Dirn, Pos)> = Vec::with_capacity(4);
        if self.lni > 0 {
            out.push((Dirn::E, Pos::from_ln_x(self.lni - 1, self.xi)));
        }
        if self.lni < last_pos_incl.lni {  // if this isn't last
            out.push((Dirn::W, Pos::from_ln_x(self.lni + 1, self.xi)));
        }
        if self.xi > 0 {
            out.push((Dirn::N, Pos::from_ln_x(self.lni, self.xi -1)));
        }
        if self.xi < last_pos_incl.xi {  // if this isn't last
            out.push((Dirn::S, Pos::from_ln_x(self.lni, self.xi +1)));
        }
        out
    }
}

fn parse_line(ln: &str) -> Vec<TileType> {
    ln.chars().map(TileType::from_char).collect_vec()
}

fn find_start(grid: &Vec<Vec<TileType>>) -> Pos {
    grid.iter().enumerate().find_map(|(lni, ln)| {
        ln.iter().enumerate().find_map(|(xi, t)| (*t == TileType::Start).then_some(Pos::from_ln_x(lni, xi)))
    }).expect("Grid should contain a start")
}

// "always two [connecting tiles] there are, no more, no less"
fn find_connecting_to_start(grid: &Vec<Vec<TileType>>, start_pos: Pos, gsize: Pos) -> (Pos, Pos) {
    start_pos.get_adj_and_dirn(gsize)
        .into_iter()
        .filter_map(|(dirn_from_start, pos)| pos.index_in(grid).has_connector(dirn_from_start.opp()).then_some(pos))
        .collect_tuple()
        .expect("always two [connection to the start] there should be, no more, no less")
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0);
    let grid = lines.map(parse_line).collect_vec();
    let gsize = Pos::from_ln_x(grid.len(), grid[0].len());
    let start_pos = find_start(&grid);
    let start_adj = find_connecting_to_start(&grid, start_pos, gsize);
    let dist_map: HashMap<Pos, usize> = HashMap::from([(start_pos, 0), (start_adj.0, 1), (start_adj.1, 1)]);
    let curr = start_adj;
    // TODO: while ! next_pos in dist_map: expand by 1 from curr - find thing in dirn of conn
}
