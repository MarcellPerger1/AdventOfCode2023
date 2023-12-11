use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::mem;

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

    fn add_dirn(self, dirn: Dirn, gsize: Pos) -> Result<Self, &'static str> {
        use Dirn::*;
        let Self {lni, xi} = self;
        match dirn {
            N => self.lni.checked_sub(1).and_then(|lni| Some(Self {lni, xi})).ok_or("Cannot access N direction (outside of the map)"),
            S => (lni + 1 < gsize.lni).then_some(Self { lni: lni + 1, xi }).ok_or("Cannot access S direction (outside of the map)"),
            E => (xi + 1 < gsize.xi).then_some(Self { lni, xi: xi + 1 }).ok_or("Cannot access E direction (outside of the map)"),
            W => self.xi.checked_sub(1).and_then(|xi| Some(Self {lni, xi})).ok_or("Cannot access W direction (outside of the map)")
        }
    }

    fn get_adj_and_dirn(self, last_pos_excl: Pos) -> Vec<(Dirn, Pos)> {
        let last_pos_incl = Pos::from_ln_x(last_pos_excl.lni - 1, last_pos_excl.xi - 1);
        let mut out: Vec<(Dirn, Pos)> = Vec::with_capacity(4);
        if self.lni > 0 {
            out.push((Dirn::N, Pos::from_ln_x(self.lni - 1, self.xi)));
        }
        if self.lni < last_pos_incl.lni {  // if this isn't last
            out.push((Dirn::S, Pos::from_ln_x(self.lni + 1, self.xi)));
        }
        if self.xi > 0 {
            out.push((Dirn::W, Pos::from_ln_x(self.lni, self.xi -1)));
        }
        if self.xi < last_pos_incl.xi {  // if this isn't last
            out.push((Dirn::E, Pos::from_ln_x(self.lni, self.xi +1)));
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
fn find_connecting_to_start(grid: &Vec<Vec<TileType>>, start_pos: Pos, gsize: Pos) -> [(Dirn, Pos); 2] {
    start_pos.get_adj_and_dirn(gsize)
        .into_iter()
        .filter_map(|(dirn_from_start, pos)| pos.index_in(grid).has_connector(dirn_from_start.opp()).then_some((dirn_from_start, pos)))
        .collect_vec()
        .try_into()
        .expect("always two [connection to the start] there are, no more, no less")
}

fn find_next(grid: &Vec<Vec<TileType>>, (dirn_from_prev, pos): (Dirn, Pos), gsize: Pos) -> (Dirn, Pos) {
    let dirn_to_prev = dirn_from_prev.opp();
    let curr_tile = pos.index_in(grid);
    let curr_connectors = curr_tile.get_connector_pair().expect("find_next should only be called on a normal tile");
    let dirn_to_next = curr_connectors.iter().copied().filter(|c| *c != dirn_to_prev).exactly_one()
        .expect("Each connector should have 2 connections: 1 to the prev, 1 to next");
    // println!("find_next: {:?};   {:?}", dirn_to_next, pos); 
    let next_pos = pos.add_dirn(dirn_to_next, gsize).expect("Error: pipe is pointing out of map");
    (dirn_to_next, next_pos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LoopControl {
    Continue, Break
}

#[must_use="Should check to end loop or not"]
fn handle_next_node(grid: &Vec<Vec<TileType>>, curr_info: &mut ((Dirn, Pos), usize), dist_map: &mut HashMap<Pos, usize>, gsize: Pos) -> LoopControl {
    // println!("{:?};   {:?};   {:?}", curr_info, dist_map, gsize);
    // this mem::replace is sorta hacky but whatev
    let _ = mem::replace(curr_info, (find_next(&grid, curr_info.0, gsize), curr_info.1 + 1));
    // if it is in, already found by traversing other so finished, ...
    if dist_map.contains_key(&curr_info.0.1) {
        return LoopControl::Break;
    }
    // ... else, add it and continue
    dist_map.insert(curr_info.0.1, curr_info.1);
    LoopControl::Continue
}

fn get_loop_dists(contents: &String) -> HashMap<Pos, usize> {
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0);
    let grid = lines.map(parse_line).collect_vec();
    // println!("{}", grid.iter().map(|ln| format!("{:?}", ln)).join("\n"));
    let gsize = Pos::from_ln_x(grid.len(), grid[0].len());
    let start_pos = find_start(&grid);
    let start_adj = find_connecting_to_start(&grid, start_pos, gsize);
    // rather unidiomatic rust but whatever... (should've / could've used fold_while)
    let mut dist_map: HashMap<Pos, usize> = HashMap::from([(start_pos, 0), (start_adj[0].1, 1), (start_adj[1].1, 1)]);
    let mut curr = start_adj.map(|v| (v, 1));
    loop {
        if handle_next_node(&grid, &mut curr[0], &mut dist_map, gsize) == LoopControl::Break { break; }
        if handle_next_node(&grid, &mut curr[1], &mut dist_map, gsize) == LoopControl::Break { break; }
    }
    dist_map
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let dist_map = get_loop_dists(&contents);
    // println!("{:?}", dist_map);
    let m = dist_map.into_iter().max_by_key(|(_pos, dist)| *dist).unwrap();
    println!("Part 1: {} at {:?}", m.1, m.0);
    // TODO
}
