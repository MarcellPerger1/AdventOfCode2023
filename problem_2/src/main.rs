use std::fs;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, Default)]
struct CubeCount {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl CubeCount {
    pub fn just_red(red: u32) -> CubeCount {
        CubeCount {
            red,
            green: 0,
            blue: 0,
        }
    }
    pub fn just_green(green: u32) -> CubeCount {
        CubeCount {
            red: 0,
            green,
            blue: 0,
        }
    }
    pub fn just_blue(blue: u32) -> CubeCount {
        CubeCount {
            red: 0,
            green: 0,
            blue,
        }
    }
    pub fn add(self, other: Self) -> Self {
        CubeCount {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
    pub fn le(self, other: Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
    pub fn max_cubes(self, other: Self) -> Self {
        CubeCount {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn part1() {
    let cubes_in_bag = CubeCount {
        red: 12,
        green: 13,
        blue: 14,
    };
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let ids = lines.filter_map(|ln| -> Option<usize> {
        let ln_without_prefix = ln
            .strip_prefix("Game ")
            .expect("Bad format - Expected 'Game N'");
        let (id_str, rest) = ln_without_prefix
            .split_once(":")
            .expect("Bad format - Expected ':'");
        let id = id_str
            .trim()
            .parse::<usize>()
            .expect("Bad format - invalid game id number");
        let rounds = rest.trim().split_terminator(";");
        let cube_counts = rounds.map(cubes_in_round);
        let count = cube_counts
            .reduce(CubeCount::max_cubes)
            .expect("Game should not be empty");
        println!("{ln} => {count:?}");
        if count.le(cubes_in_bag) {
            Some(id)
        } else {
            None
        }
    });
    let s: usize = ids.sum();
    println!("Part1: sum={s}");
}

fn cubes_in_round(round: &str) -> CubeCount {
    let items = round.trim().split(",").map(|item| item.trim());
    let cube_counts = items.map(|item_s| {
        let v: Vec<_> = item_s.split(" ").collect();
        assert_eq!(v.len(), 2, "Expected each item to be <amount> <color>");
        let amount = v[0].parse::<u32>().unwrap();
        match v[1] {
            "red" => CubeCount::just_red(amount),
            "green" => CubeCount::just_green(amount),
            "blue" => CubeCount::just_blue(amount),
            _ => panic!("Unknown color"),
        }
    });
    cube_counts
        .reduce(CubeCount::add)
        .expect("Expected at least 1 cube")
}

fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let powers = lines.map(|ln| -> u32 {
        let ln_without_prefix = ln
            .strip_prefix("Game ")
            .expect("Bad format - Expected 'Game N'");
        let (_id_str, rest) = ln_without_prefix
            .split_once(":")
            .expect("Bad format - Expected ':'");
        // let id = id_str.trim().parse::<usize>().expect("Bad format - invalid game id number");
        let rounds = rest.trim().split_terminator(";");
        let cube_counts = rounds.map(cubes_in_round);
        let required_cubes = cube_counts
            .reduce(CubeCount::max_cubes)
            .expect("Game should not be empty");
        let power = required_cubes.power();
        println!("{ln} => {required_cubes:?}, power={power}");
        power
    });
    let s: u32 = powers.sum();
    println!("Part2: sum={s}");
}
