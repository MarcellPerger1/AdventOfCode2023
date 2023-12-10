use itertools::Itertools;
use itertools::FoldWhile;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::fs;
use num::Integer;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Left,
    Right
}
// impl Instruction {
//     fn idx_tuple<T>(self, tp: (T, T)) -> T {
//         match self {
//             Self::Left => tp.0,
//             Self::Right => tp.1
//         }
//     }
// }

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

// fn is_all_end(names: &Vec<&String>) -> bool {
//     names.iter().all(|k| k.chars().last().expect("Expected non-null name") == 'Z')
// }

fn is_end(name: &String) -> bool {
    name.chars().last().expect("Expected non-null name") == 'Z'
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct NodeI {
//     out: (u16, u16)
// }
// impl NodeI {
//     fn new2(left: u16, right: u16) -> Self {
//         Self { out: (left, right) }
//     }
// }
// impl Default for NodeI {
//     fn default() -> Self {
//         Self { out: (u16::MAX, u16::MAX) }
//     }
// }

// fn make_inodes(nodes: HashMap<String, Node>) -> (Vec<NodeI>, HashMap<String, u16>) {
//     let mut str_to_i: HashMap<String, u16> = HashMap::new();
//     let mut next_i = 0_u16;
//     let mut get_or_insert_k = |s: String| -> u16 {
//         if let Some(i) = str_to_i.get(&s) {
//             return *i;
//         }
//         let curr_i = next_i;
//         str_to_i.insert(s, curr_i);
//         next_i += 1;
//         curr_i
//     };
//     let kv_pairs_vec = nodes.into_iter().map(|(k, v)|{
//         let k_new = get_or_insert_k(k);
//         let left = get_or_insert_k(v.out.0);
//         let right = get_or_insert_k(v.out.1);
//         (k_new, NodeI::new2(left, right))
//     }).collect_vec();
//     let mut v = [NodeI::default()].repeat(next_i as _);
//     for (i, r) in kv_pairs_vec {
//         v[i as usize] = r;
//     }
//     (v, str_to_i)
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct FlatNode {
//     next_i: u32
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct INodeWithIdx {
//     out: (u16, u16),
//     instr_i: u16
// }
// impl INodeWithIdx {
//     fn new(out: (u16, u16), instr_i: u16) -> Self {
//         Self { out, instr_i }
//     }

//     fn as_inode(self) -> NodeI {
//         NodeI { out: self.out }
//     }
// }

// fn find_chain(inodes: &Vec<NodeI>, instr_list: &Vec<Instruction>, start: usize) -> Vec<INodeWithIdx> {
//     let mut visited: HashSet<INodeWithIdx> = HashSet::new();
//     let mut nd_chain: Vec<INodeWithIdx> = vec![];
//     let (_repeated_nd, _cycle_len) = instr_list
//         .iter()
//         .enumerate()
//         .cycle()
//         .fold_while((inodes[start], 0_u64), |(curr_v, n), (ii, instr)| {
//             let new_nd = INodeWithIdx::new(curr_v.out, ii as u16);
//             if visited.contains(&new_nd) { return FoldWhile::Done((curr_v, n)); }
//             nd_chain.push(new_nd);
//             visited.insert(new_nd);
//             let next_nd = inodes[instr.idx_tuple(curr_v.out) as usize];
//             FoldWhile::Continue((next_nd, n+1))
//         }).into_inner();
//     nd_chain
// // }

// fn find_stop_indices(nd_chain: &Vec<INodeWithIdx>, stop_nodes: Vec<NodeI>) -> Vec<usize> {
//     nd_chain.iter().enumerate().filter_map(|(i, n2)| stop_nodes.contains(&n2.as_inode()).then_some(i)).collect_vec()
// }
// fn transform_to_inode_idxs(nodes: &Vec<&String>, str_to_idx: &HashMap<String, u16>) -> Vec<u16> {
//     nodes.iter().map(|s| *str_to_idx.get(*s).expect("Expected valid node")).collect_vec()
// }

// fn get_chains(inodes: &Vec<NodeI>, instr_list: &Vec<Instruction>, start_idxs: &Vec<u16>) -> Vec<Vec<INodeWithIdx>> {
//     start_idxs.iter().map(|start| find_chain(inodes, instr_list, *start as _)).collect_vec()
// }

fn part2() {
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
    let starting_nodes = nodes_kv.keys().filter(|k| k.chars().last() == Some('A')).collect_vec();
    // let end_nodes = nodes_kv.keys().filter(|k| k.chars().last().expect("Expected non-null name") == 'Z').collect_vec();
    // println!("{:?}", nodes_kv);
    // println!("{:?}", starting_nodes);
    // println!("{:?}", end_nodes);
    // println!("{:?}", is_all_end(&end_nodes));
    // let (inodes, str_to_idx) = make_inodes(nodes_kv.clone());
    // let end_idxs = transform_to_inode_idxs(&end_nodes, &str_to_idx);
    // let start_idxs = transform_to_inode_idxs(&starting_nodes, &str_to_idx);
    // println!("{:?}\n{:?}", start_idxs, end_idxs);
    // println!("{}", inodes.iter().map(|n| format!("{:?}", n)).join("\n"));
    // // let ch = find_chain(&inodes, &instructions, start_idxs[0] as _);
    // let ch_vec = get_chains(&inodes, &instructions, &start_idxs);
    // println!("{:?}", ch_vec.iter().map(|ch| ch.len()).collect_vec());
    // let mut states: HashSet<Vec<&String>> = HashSet::with_capacity(10_000);
    let amounts = starting_nodes.iter().map(|start| {
        let (_, amount) = instructions.iter().cycle().fold_while((start.to_owned(), 0_u64), |(curr_s, n), instr| {
            // println!("{:?}, {:?}", curr_s, instr);
            if is_end(curr_s) { return FoldWhile::Done((curr_s, n)); }
            FoldWhile::Continue((nodes_kv[curr_s].get_next_name(*instr), n+1))
        }).into_inner();
        amount
    });
    // NOTE: this 'lcm of each run' method will ONLY work for the specially crafted input 
    // that AoC gives us that has extra assumptions that are UNDOCUMENTED
    let lcm = amounts.reduce(|a, b| a.lcm(&b)).unwrap();
    println!("Part2: {}", lcm);
    // let (_, amount) = instructions.into_iter().cycle().fold_while((starting_nodes, 0_i64), |(curr_v, n), instr| {
    //     // println!("{:?}, {:?}", curr_s, instr);
    //     if is_all_end(&curr_v) { return FoldWhile::Done((vec![], n)); }
    //     // states.insert(curr_v.to_owned());
    //     if n%1_000_000 == 0 && n != 0 {println!("Done {} iterations", n);}
    //     if curr_v[0].chars().last() == Some('Z') && curr_v[1].chars().last() == Some('Z') && curr_v[2].chars().last() == Some('Z') {
    //         println!("{:?}", curr_v);
    //     }
    //     // println!("{:?}", curr_v);
    //     FoldWhile::Continue((curr_v.into_iter().map(|curr_s| nodes_kv[curr_s].get_next_name(instr)).collect_vec(), n+1))
    // }).into_inner();
    // println!("Part2: {}", amount);
}
