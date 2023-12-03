use std::fs;
use std::iter;
use itertools::Itertools;


fn main() {
    part1();
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
            println!("x={x}");
            x
            // ...
        } else {
            unreachable!();
        }
    });
    let s = sums.sum::<u32>();
    println!("Part1: sum={s}");
}
