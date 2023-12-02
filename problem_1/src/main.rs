use std::fs;
use std::iter;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let nums_as_str = lines.map(|ln| {
        let nums: Vec<_> = ln.chars().filter(|c| c.is_numeric()).collect();
        format!(
            "{}{}",
            nums.first().expect("Expected it to have 1st char"),
            nums.last().expect("Expected it to have 1st char")
        )
    });
    let nums = nums_as_str.map(|s| s.parse::<u32>().unwrap());
    let s: u32 = nums.sum();
    println!("Part1: The sum is {s}");
}

const NUM_WORDS: &[&str] = &[
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parsed_tup(tup: &(usize, char)) -> (usize, i32) {
    (tup.0, tup.1.to_string().parse().unwrap())
}
fn to_signed_tup(tup: &(usize, i32)) -> (isize, i32) {
    (tup.0 as isize, tup.1)
}

fn part2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let nums = lines.map(|ln| {
        let nums: Vec<_> = ln
            .chars()
            .enumerate()
            .filter(|(_i, c)| c.is_numeric())
            .collect();
        let first_digit = nums.first().and_then(|tup| Some(parsed_tup(tup)));
        let last_digit = nums
            .last()
            .and_then(|tup| Some(to_signed_tup(&parsed_tup(tup))));
        let first = (1..=9)
            .map(|num| {
                ln.find(NUM_WORDS[num])
                    .and_then(|idx| Some((idx, num as i32)))
            })
            .chain(iter::once(first_digit))
            .filter_map(|x| x)
            .min_by_key(|(i, _value)| i.to_owned())
            .expect("Expected first value")
            .1;
        let last = (1..=9)
            .map(|num| {
                ln.rfind(NUM_WORDS[num])
                    .and_then(|i| Some(((i + NUM_WORDS[num].len() - 1) as isize, num as i32)))
            })
            .chain(iter::once(last_digit))
            .filter_map(|x| x)
            .max_by_key(|(i, _value)| i.to_owned())
            .expect("Expected last value")
            .1;
        return 10 * first + last;
    });
    let res: i32 = nums.sum();
    println!("Part2: The sum is {res}");
}
