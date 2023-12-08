use itertools::Itertools;
use itertools::FoldWhile;
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::cmp;

fn main() {
    part1();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Left,
    Right
}

fn parse_instructions(ln: &str) -> Vec<Instruction> {
    ln.trim().chars().map(|c| {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown char")
        }
    }).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    out: (String, String)
}
impl Node {
    fn new_from_str(name: &str, out: (&str, &str)) -> Self {
        Self { name: name.to_string(), out: (out.0.to_string(), out.1.to_string()) }
    }
    fn get_next_name(&self, instr: Instruction) -> &String {
        match instr {
            Instruction::Left => &self.out.0,
            Instruction::Right => &self.out.1,
        }
    }
}

fn parse_node_line(ln: &str) -> Node {
    let (name_s, right) = ln.split_once('=').expect("Expected format of node: 'name = ...'");
    let name = name_s.trim();
    let out = parse_node_tuple(right.trim());
    Node::new_from_str(name, out)
}
fn parse_node_tuple(right: &str) -> (&str, &str) {
    let inner = right.strip_prefix('(').and_then(|v| v.strip_suffix(')')).expect("Node value should be in parens");
    let (out_l, out_r) = inner.split_once(',').expect("Left and rigt should be comma-separated");
    (out_l.trim(), out_r.trim())
}

fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines = contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect_vec();
    let instructions = parse_instructions(lines[0]);
    let nodes = lines[1..].iter().map(|ln| parse_node_line(ln));
    let nodes_kv: HashMap<_, _> = nodes.map(|node| (node.name.clone(), node)).collect();
    // println!("{:?}", nodes_kv);
    let (_, amount) = instructions.into_iter().cycle().fold_while(("AAA".to_string(), 0), |(curr_s, n), instr| {
        // println!("{:?}, {:?}", curr_s, instr);
        if curr_s == "ZZZ" { return FoldWhile::Done(("ZZZ".to_string(), n)); }
        FoldWhile::Continue((nodes_kv[&curr_s].get_next_name(instr).clone(), n+1))
    }).into_inner();
    println!("Part1: {}", amount);
}
