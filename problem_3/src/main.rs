use std::fs;
use std::iter;
use itertools::Itertools;


fn main() {
    part1();
    part2();
}


fn part1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0).collect();
    let maxlen = lines.iter().map(|ln| ln.len()).max().expect("Expected >= 1 line");
    let empty_line = ".".repeat(maxlen);
    let lines_with_padding: Vec<_> = iter::once(empty_line.as_str())
        .chain(lines.iter().map(|refref| *refref))
        .chain(iter::once(empty_line.as_str())).collect();
    let sums = lines_with_padding.windows(3).map(|lns| {
        if let [prev, curr, next] = lns {
            let numeric_groups = curr
                .char_indices()
                .group_by(|(_i, c)| c.is_numeric());
            let nums_on_this_line = numeric_groups.into_iter()
                .filter_map(|v| if v.0 { Some(v.1) } else { None })
                .filter_map(|groups| {
                    let groupsv = groups.collect_vec();
                    // println!("{:?}", groupsv);
                    let first_idx = groupsv.first().unwrap().0;
                    let last_idx = groupsv.last().unwrap().0;
                    let num_str = groupsv.iter().map(|x| x.1).join("");
                    let num = num_str.parse::<u32>().expect("Invalid num format");
                    let top_adj = (first_idx.saturating_sub(1)..=last_idx+1)
                        .map(|i| prev.chars().nth(i).unwrap_or('.'));  // incl. diagonal
                    let bot_adj = (first_idx.saturating_sub(1)..=last_idx+1)
                        .map(|i| next.chars().nth(i).unwrap_or('.'));
                    let mut adj = top_adj
                        .chain(iter::once(if first_idx == 0 {'.'} else { curr.chars().nth(first_idx.saturating_sub(1)).unwrap_or('.')}))
                        .chain(iter::once(curr.chars().nth(last_idx + 1).unwrap_or('.')))
                        .chain(bot_adj);
                    let mut d: Vec<_> = adj.clone().collect();
                    // println!("{:?}", d);
                    if adj.any(|c| c != '.') {
                        // num should be counted
                        Some(num)
                    } else {
                        None
                    }
                });
            let x = nums_on_this_line.sum::<u32>();
            // println!("x={x}");
            x
            // ...
        } else {
            unreachable!();
        }
    });
    let s = sums.sum::<u32>();
    println!("Part1: sum={s}");
}

fn intersect_ranges(r0: (usize, usize), r1: (usize, usize)) -> bool {
    // (both incl.)
    // not intersecting <=> r0.0 <= r0.1 < r1.0 <= r1.1 || r1.0 <= r1.1 < r0.0 <= r0.1
    //                  <=> r0.1 < r.1 || r1.1 < r0.0
    let is_disjoint: bool = r0.1 < r1.0 || r1.1 < r0.0;
    !is_disjoint
}

fn find_intersecting_on_line(num_idx_list: &Vec<Vec<(usize, usize, u32)>>, line_i: usize, idx_range: (usize, usize)) -> Vec<u32> {
    num_idx_list[line_i].iter().filter_map(|(start, end, num)| {
        // include num if top area intersects it
        intersect_ranges(idx_range, (*start, *end)).then_some(*num)
    }).collect()
}


fn part2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines: Vec<_> = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0).collect();
    // println!("{:#?}", triples);
    let num_idx_list: Vec<_> = lines.iter().map(|ln| {
        let numeric_groups = ln
            .char_indices()
            .group_by(|(_i, c)| c.is_numeric());
        let nums_on_this_line = numeric_groups.into_iter()
            .filter_map(|(is_num, val)| if is_num { Some(val) } else { None })
            .map(|groups| {
                let groupsv = groups.collect_vec();
                // println!("{:?}", groupsv);
                let first_idx = groupsv.first().unwrap().0;
                let last_idx = groupsv.last().unwrap().0;
                let num_str = groupsv.iter().map(|x| x.1).join("");
                let num = num_str.parse::<u32>().expect("Invalid num format");
                (first_idx, last_idx, num)
            });
        nums_on_this_line.collect::<Vec<_>>()
    }).collect();
    // println!("{:#?}", num_idx_list);
    let ratios_on_lines = lines.iter().enumerate().map(|(li, ln)| {
        let star_indices = ln
            .char_indices()
            .filter_map(|(i, c)| if c == '*' { Some(i) } else { None });
        let ratios_list = star_indices.filter_map(|i| {
            // println!("Star: {}:{}", li, i);
            let idx_range_top = if i == 0 { (i, i+1) } else { (i-1, i+1) };
            let idx_range_bot = idx_range_top;
            let top_nums = if let Some(prev_i) = li.checked_sub(1) {
                find_intersecting_on_line(&num_idx_list, prev_i, idx_range_top)
            } else { Vec::new() };
            let prev_nums = if i != 0 {
                find_intersecting_on_line(&num_idx_list, li, (i-1, i-1)) 
            } else { Vec::new() };
            let next_nums = if i != ln.len()-1 {
                find_intersecting_on_line(&num_idx_list, li, (i+1, i+1))
            } else { Vec::new() };
            let bot_nums = if li < lines.len()-1 {
                find_intersecting_on_line(&num_idx_list, li+1, idx_range_bot)
            } else { Vec::new() };
            let adj_nums = top_nums.into_iter().chain(prev_nums.into_iter()).chain(next_nums.into_iter()).chain(bot_nums.into_iter()).collect_vec();
            // println!("  {adj_nums:?}");
            (adj_nums.len() == 2).then(|| adj_nums[0] * adj_nums[1])
        });
        ratios_list.sum::<u32>()
    });
    let s: u32 = ratios_on_lines.sum();
    println!("Part 2: {s}");
}
