use itertools::Itertools;
use std::fs;
// use std::iter;
use std::cmp;

fn main() {
    part1();
    part2();
}

fn parse_num_list(s: &str) -> impl Iterator<Item = u64> + '_ {
    s.trim()
        .split_whitespace()
        .map(|num_s| num_s.parse::<u64>().expect("item should be u64"))
}

fn get_race_range((time, record_dist): (u64, u64)) -> (u64, u64) {
    // T = `time` = time available for race
    // S_r = `record_dist`
    // s = distance travelled
    // t_h = time held
    // s = time left * speed accumulated
    // s = (T - t_h) * t_h
    // s = (T - t_h) * t_h > S_r
    // T*t_h - t_h^2 > S_r
    // t_h^2 - T*t_h + S_r < 0
    // t_h >= 0 and
    // [ --T - sqrt((-T)^2 - 4*1*S_r) ] / 2*1 < t_h < [ --T + sqrt((-T)^2 - 4*1*S_r) ] / 2*1
    // [T - sqrt(T^2 - 4*S_r)] / 2 < t_h < [T + sqrt(T^2 - 4*S_r)]
    if let Some(discriminant) = (time * time).checked_sub(4 * record_dist) {
        let d_f64 = discriminant as f64;
        let time_f64 = time as f64;
        let sqrt_d = d_f64.sqrt();
        let lo_f64 = (time_f64 - sqrt_d) / 2.0;
        let lo_excl = cmp::max(lo_f64.floor() as i64, 0) as u64;
        let lo_incl = lo_excl + 1;
        let hi_f64 = (time_f64 + sqrt_d) / 2.0;
        let hi_excl = cmp::max(hi_f64.ceil() as i64, 0) as u64;
        let hi_incl = hi_excl - 1;
        assert!(hi_incl >= lo_incl, "No positive solution to quadratic");
        (lo_incl, hi_incl)
    } else {
        // discr < 0 => no real solutions - this problem should always have solutions so PANIC
        panic!("Quadratic has no solutions");
    }
}

fn get_race_moe(race: (u64, u64)) -> u64 {
    let (lo_incl, hi_incl) = get_race_range(race);
    let hi_excl = hi_incl + 1;
    hi_excl - lo_incl
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    // let contents =
    //     fs::read_to_string("./src/example.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect_vec();
    let times = parse_num_list(
        lines[0]
            .strip_prefix("Time: ")
            .expect("line 1 should be 'Time: '"),
    );
    let dists = parse_num_list(
        lines[1]
            .strip_prefix("Distance: ")
            .expect("line 2 should be 'Distance: '"),
    );
    let td_vec = times.zip(dists).collect_vec();
    let moe = td_vec.iter().map(|r| get_race_moe(*r)).collect_vec();
    // println!("{:#?};\n {:#?}", td_vec, moe);
    let v = moe.iter().product::<u64>();
    println!("Part1: {}", v);
}

fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect_vec();
    let time_s = lines[0]
        .strip_prefix("Time: ")
        .expect("line 1 should be 'Time: '")
        .replace(" ", "");
    let dist_s = lines[1]
        .strip_prefix("Distance: ")
        .expect("line 2 should be 'Distance: '")
        .replace(" ", "");
    let time = time_s
        .parse::<u64>()
        .expect("Line 1 should only contain numbers");
    let dist = dist_s
        .parse::<u64>()
        .expect("Line 2 should only contain numbers");
    let v = get_race_moe((time, dist));
    println!("Part1: {}", v);
}
