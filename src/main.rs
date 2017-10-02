use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::min;

fn main() {
    let num_test_cases = get_line().trim().parse::<i32>().unwrap();
    // println!("{} test cases", num_test_cases);
    let mut soln_counts = Vec::new();
    for test_case_num in 0..num_test_cases {
        let info: (i32, i32, HashSet<(i32, i32)>) = read_test_case();
        // let soln = solve_brute_force(info.0, info.1, &info.2);
        println!("\x1b[1mTEST CASE {}\x1b[0m\n", test_case_num + 1);
        let soln = solve(info.0, info.1, &info.2);
        for (i, placements) in soln.iter().enumerate() {
            println!("{} solns with {} queen{}", placements.len(), i + 1, if i > 0 { "s" } else { "" });
            // for plc in placements {
            //     print_placement(info.0, info.1, plc, &info.2);
            //     println!("---");
            // }
            // println!();
        }
        println!(
            "{} total solutions\n\n",
            soln.iter().fold(0, |acc, v| acc + v.len()));
        soln_counts.push(
            soln.iter().fold(0, |acc, v| acc + v.len()));
    }

    for c in &soln_counts {
        println!("{}", c);
    }
}

fn solve(rows: i32, cols: i32, blocked_positions: &HashSet<(i32, i32)>) -> Vec<Vec<HashSet<(i32, i32)>>> {
    let mut all_placements: Vec<Vec<HashSet<(i32, i32)>>> = Vec::new();
    let mut new_placements: Vec<HashSet<(i32, i32)>> = get_placements_from_basis(rows, cols, &vec![HashSet::new()], blocked_positions);
    loop {
        if new_placements.len() > 0 {
            all_placements.push(new_placements.clone());
        } else {
            break;
        }
        new_placements = get_placements_from_basis(rows, cols, &new_placements, blocked_positions);
    }
    all_placements
}

fn get_placements_from_basis(
    rows: i32,
    cols: i32,
    basis_placements: &Vec<HashSet<(i32, i32)>>,
    blocked_positions: &HashSet<(i32, i32)>) -> Vec<HashSet<(i32, i32)>>
{
    let mut placements = Vec::new();
    println!("iterating over {} basis placements", basis_placements.len());
    for plc in basis_placements {
        let taken_rows: HashSet<i32> = HashSet::from_iter(plc.iter().map(|pair| pair.0));
        let taken_cols: HashSet<i32> = HashSet::from_iter(plc.iter().map(|pair| pair.1));
        let blocked_rows: HashSet<i32> = HashSet::from_iter(blocked_positions.iter().map(|pair| pair.0));
        let blocked_cols: HashSet<i32> = HashSet::from_iter(blocked_positions.iter().map(|pair| pair.1));
        for row in (0..rows).filter(|r| !taken_rows.contains(r) || blocked_rows.contains(r)) {
            for col in (0..cols).filter(|c| !taken_cols.contains(c) || blocked_cols.contains(c)) {
                if !plc.contains(&(row, col)) {
                    let mut new_placement: HashSet<(i32, i32)> = plc.clone();
                    new_placement.insert((row, col));
                    if is_valid_placement(
                        rows,
                        cols,
                        &new_placement.clone().into_iter().collect::<Vec<(i32, i32)>>(),
                        blocked_positions)
                    {
                        if !placements.contains(&new_placement) {
                            placements.push(new_placement);
                        }
                    }
                }
            }
        }
    }
    placements
}

fn print_placement(rows: i32, cols: i32, occupied_positions: &HashSet<(i32, i32)>, blocked_positions: &HashSet<(i32, i32)>) {
    for row in 0..rows {
        for col in 0..cols {
            if occupied_positions.contains(&(row, col)) {
                print!("Q ");
            } else if blocked_positions.contains(&(row, col)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn solve_brute_force(rows: i32, cols: i32, blocked_positions: &HashSet<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let max_queens = min(rows, cols);
    let placements: Vec<Vec<(i32, i32)>> = enumerate_placements(rows, cols, max_queens);
    placements
        .into_iter()
        .filter(|positions|
            is_valid_placement(rows, cols, &positions.to_vec(), blocked_positions))
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn is_valid_placement(rows: i32, cols: i32, positions: &Vec<(i32, i32)>, blocked_positions: &HashSet<(i32, i32)>) -> bool {
    let positions_set: HashSet<(i32, i32)> = HashSet::from_iter(positions.clone());
    for pos in &positions_set {
        if !(is_valid_path(get_right_horizontal(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_down_horizontal(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_left_horizontal(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_up_horizontal(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_up_right_diag(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_down_right_diag(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_down_left_diag(*pos, rows, cols), &positions_set, blocked_positions) &&
            is_valid_path(get_up_left_diag(*pos, rows, cols), &positions_set, blocked_positions)) ||
            blocked_positions.contains(pos)
        {
            return false
        }
    }
    true
}

fn get_right_horizontal(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0, curr.1 + 1);
    }
    positions
}

fn get_down_horizontal(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 + 1, curr.1);
    }
    positions
}

fn get_left_horizontal(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0, curr.1 - 1);
    }
    positions
}

fn get_up_horizontal(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 - 1, curr.1);
    }
    positions
}

fn get_up_right_diag(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 - 1, curr.1 + 1);
    }
    positions
}

fn get_down_right_diag(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 + 1, curr.1 + 1);
    }
    positions
}

fn get_down_left_diag(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 + 1, curr.1 - 1);
    }
    positions
}

fn get_up_left_diag(pos: (i32, i32), rows: i32, cols: i32) -> Vec<(i32, i32)> {
let mut curr = pos;
    let mut positions = Vec::new();
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        positions.push(curr);
        curr = (curr.0 - 1, curr.1 - 1);
    }
    positions
}

fn is_valid_path(path: Vec<(i32, i32)>, occupied_positions: &HashSet<(i32, i32)>, blocked_positions: &HashSet<(i32, i32)>) -> bool {
    for pos in path.iter().skip(1) {
        if blocked_positions.contains(&pos) {
            return true;
        } else if occupied_positions.contains(&pos) {
            return false;
        }
    }
    true
}

fn enumerate_placements(rows: i32, cols: i32, max_queens: i32) -> Vec<Vec<(i32, i32)>> {
    let mut placements = Vec::new();
    for num_queens in 1..(max_queens + 1) {
        let mut plcs =
            get_choose_set(0, (rows * cols) - 1, num_queens)
                .into_iter()
                .map(|placement|
                    placement
                        .into_iter()
                        .map(|num| num_to_coord(cols, num))
                        .collect::<Vec<(i32, i32)>>())
                .collect::<Vec<Vec<(i32, i32)>>>();
        placements.append(&mut plcs);
    }
    placements
}

fn num_to_coord(cols: i32, num: i32) -> (i32, i32) {
    (num / cols, num % cols)
}

fn get_choose_set(start: i32, end: i32, num: i32) -> Vec<Vec<i32>> {
    if num == 1 {
        (start..(end + 1))
            .map(|x| vec![x])
            .collect::<Vec<Vec<i32>>>()
    } else {
        let mut sets = Vec::new();
        for i in start..(end - num + 2) {
            let mut smaller_sets = get_choose_set(i + 1, end, num - 1);
            for set in &mut smaller_sets {
                set.push(i);
            }
            sets.append(&mut smaller_sets);
        }
        sets
    }
}

fn read_test_case() -> (i32, i32, HashSet<(i32, i32)>) {
    let row_col: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let (rows, cols) = (row_col[0], row_col[1]);

    let mut blocked_positions = HashSet::new();
    for row in 0..rows {
        let line = get_line();
        let chars = line.chars();

        for (col, ch) in chars.enumerate() {
            if ch == '#' {
                blocked_positions.insert((row, col as i32));
            }
        }
    }

    (rows, cols, blocked_positions)
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
