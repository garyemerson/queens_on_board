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
        // println!("\x1b[1mTEST CASE {}\x1b[0m\n", test_case_num + 1);
        // let soln = solve(info.0, info.1, &info.2);
        let soln = solve2(info.0, info.1, &info.2);
        // for (i, placements) in soln.iter().enumerate() {
        //     println!("{} solns with {} queen{}", placements.len(), i + 1, if i > 0 { "s" } else { "" });
        //     // print_placement(info.0, info.1, &HashSet::from_iter(placements.clone()), &info.2);
        //     for plc in placements {
        //         print_placement(info.0, info.1, plc, &info.2);
        //         println!("---");
        //     }
        //     println!();
        // }

        // brute force
        // println!(
        //     "{} total solutions\n\n",
        //     soln.len());
        // soln_counts.push(
        //     soln.len());

        // optimal
        // println!(
        //     "{} total solutions\n\n",
        //     soln/*.iter().fold(0, |acc, v| acc + v.len())*/);
        soln_counts.push(
            soln/*.iter().fold(0, |acc, v| acc + v.len())*/);
    }

    for c in &soln_counts {
        println!("{}", c);
    }
}

fn solve(rows: i32, cols: i32, blocked_positions: &HashSet<(i32, i32)>) -> usize {
    let mut all_placements: Vec<Vec<HashSet<(i32, i32)>>> = Vec::new();
    let mut new_placements: Vec<(HashSet<(i32, i32)>, i32)> = get_placements_from_basis(rows, cols, vec![(HashSet::new(), -1)], blocked_positions);
    let mut count = new_placements.len();
    while new_placements.len() > 0 {
        new_placements = get_placements_from_basis(rows, cols, new_placements, blocked_positions);
        count += new_placements.len();
    }
    count
}

fn solve2(rows: i32, cols: i32, blocked_positions: &HashSet<(i32, i32)>) -> i32 {
    num_placements_from_basis(
        rows,
        cols,
        -1,
        &mut HashSet::new(),
        blocked_positions)
}

fn num_placements_from_basis(
    rows: i32,
    cols: i32,
    max_position_index: i32,
    basis_placement: &mut HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> i32
{
    let mut num_placements = 0;
    for position_index in (max_position_index + 1)..(rows * cols) {
        let row = position_index / cols;
        let col = position_index % cols;
        if can_add_pos((row, col), rows, cols, basis_placement, blocked_positions) {
            num_placements += 1;
            basis_placement.insert((row, col));
            num_placements += num_placements_from_basis(rows, cols, position_index, basis_placement, blocked_positions);
            basis_placement.remove(&(row, col));
        }
    }
    num_placements
}

fn print_in_order(rows: i32, cols: i32, placement: &HashSet<(i32, i32)>) {
    let mut clone = placement.clone();
    let mut pairs: Vec<(i32, i32)> = clone.drain().collect();
    pairs.sort_by_key(|p| p.0 * cols + cols);
    for p in pairs {
        print!("({}, {}) ", p.0, p.1);
    }
    println!();
}

fn get_placements_from_basis(
    rows: i32,
    cols: i32,
    basis_placements: Vec<(HashSet<(i32, i32)>, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> Vec<(HashSet<(i32, i32)>, i32)>
{
    let mut placements = Vec::new();
    println!("iterating over {} basis placements", basis_placements.len());
    for plc in basis_placements {
        for row in (plc.1 / cols)..rows {
            for col in 0..cols {
                if !plc.0.contains(&(row, col)) && row * cols + col > plc.1  {
                    let mut new_placement: (HashSet<(i32, i32)>, i32) = plc.clone();
                    if can_add_pos((row, col), rows, cols, &new_placement.0, blocked_positions) {
                        new_placement.0.insert((row, col));
                        new_placement.1 = row * cols + col;
                        placements.push(new_placement);
                    }
                }
            }
        }
    }
    placements
}

fn can_add_pos(pos: (i32, i32), rows: i32, cols: i32, placement: &HashSet<(i32, i32)>, blocked_positions: &HashSet<(i32, i32)>) -> bool {
    let positions_set: HashSet<(i32, i32)> = placement.clone();
    is_right_horizontal_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_down_horizontal_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_left_horizontal_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_up_horizontal_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_up_right_diag_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_down_right_diag_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_down_left_diag_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        is_up_left_diag_valid(pos, rows, cols, &positions_set, blocked_positions) &&
        !blocked_positions.contains(&pos)
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
        if !(is_right_horizontal_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_down_horizontal_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_left_horizontal_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_up_horizontal_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_up_right_diag_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_down_right_diag_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_down_left_diag_valid(*pos, rows, cols, &positions_set, blocked_positions) &&
            is_up_left_diag_valid(*pos, rows, cols, &positions_set, blocked_positions)) ||
            blocked_positions.contains(pos)
        {
            return false
        }
    }
    true
}

fn is_right_horizontal_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0, pos.1 + 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false
        } else if blocked_positions.contains(&curr) {
            return true
        }
        curr = (curr.0, curr.1 + 1);
    }
    true
}

fn is_down_horizontal_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 + 1, pos.1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 + 1, curr.1);
    }
    true
}

fn is_left_horizontal_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0, pos.1 - 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0, curr.1 - 1);
    }
    true
}

fn is_up_horizontal_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 - 1, pos.1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 - 1, curr.1);
    }
    true
}

fn is_up_right_diag_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 - 1, pos.1 + 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 - 1, curr.1 + 1);
    }
    true
}

fn is_down_right_diag_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 + 1, pos.1 + 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 + 1, curr.1 + 1);
    }
    true
}

fn is_down_left_diag_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 + 1, pos.1 - 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 + 1, curr.1 - 1);
    }
    true
}

fn is_up_left_diag_valid(
    pos: (i32, i32),
    rows: i32,
    cols: i32,
    queen_positions: &HashSet<(i32, i32)>,
    blocked_positions: &HashSet<(i32, i32)>) -> bool
{
    let mut curr = (pos.0 - 1, pos.1 - 1);
    while curr.0 >= 0 && curr.0 < rows && curr.1 >= 0 && curr.1 < cols {
        if queen_positions.contains(&curr) {
            return false;
        } else if blocked_positions.contains(&curr) {
            return true;
        }
        curr = (curr.0 - 1, curr.1 - 1);
    }
    true
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
