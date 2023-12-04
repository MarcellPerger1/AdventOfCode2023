use std::collections::HashSet;
use std::fs;
// use std::iter;
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn parse_num_list(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.trim().split_whitespace().map(|num_s| {
        num_s.parse::<u32>().expect("item should be u32")
    })
}

fn part1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let won_amounts = lines.map(|ln| {
        let without_card = ln.strip_prefix("Card ").expect("bad format: line should start with 'Card '").trim();
        let (_id_str, main_line) = without_card.split_once(':').expect("bad format: line should have ':'");
        let main_line = main_line.trim();
        let (winning_s, chosen_s) = main_line.split_once('|').expect("Line should have '|'");
        let win_set = parse_num_list(winning_s).collect::<HashSet<_>>();
        let chosen_set = parse_num_list(chosen_s).collect::<HashSet<_>>();
        let nums_in_both = win_set.intersection(&chosen_set).map(ToOwned::to_owned).collect_vec();
        if nums_in_both.len() == 0 { 0 } else { 1<<(nums_in_both.len()-1) }
    });
    let s: u32 = won_amounts.sum();
    println!("Part1: {s}");
}

fn part2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let n_matching = lines.map(|ln| {
        let without_card = ln.strip_prefix("Card ").expect("bad format: line should start with 'Card '").trim();
        let (_id_str, main_line) = without_card.split_once(':').expect("bad format: line should have ':'");
        let main_line = main_line.trim();
        let (winning_s, chosen_s) = main_line.split_once('|').expect("Line should have '|'");
        let win_set = parse_num_list(winning_s).collect::<HashSet<_>>();
        let chosen_set = parse_num_list(chosen_s).collect::<HashSet<_>>();
        let nums_in_both = win_set.intersection(&chosen_set).map(ToOwned::to_owned).collect_vec();
        nums_in_both.len()
    }).collect_vec();
    let n_lines = n_matching.len();
    let mut amounts = [1].repeat(n_lines);
    for i in 0..n_lines {
        let curr_amount = amounts[i];
        let curr_matching = n_matching[i];
        for j in i+1..i+1+curr_matching {  // the next <amount matched> cards...
            // ... each get +1*<amount we have curr card>
            amounts[j] += curr_amount;
        }
    }
    let s: u32 = amounts.iter().sum();
    println!("Part2: {s}");
}
