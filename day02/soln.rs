// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 2.

extern crate aoc;

/// Take a board, a starting point and a line of traversal
/// instructions and return the ending position.
fn walk_line(board: &[Vec<char>], n_start: (usize, usize),
             insns: &str) -> (usize, usize) {
    let mut n = n_start;

    // Walk over the instructions trying moves.
    for c in insns.chars() {
        // Find next position from direction.
        let next_n = match c {
            'U' => (n.0, n.1 - 1),
            'D' => (n.0, n.1 + 1),
            'L' => (n.0 - 1, n.1),
            'R' => (n.0 + 1, n.1),
            _ => panic!("bad direction")
        };

        // Skip steps that would walk off the keypad.
        if board[next_n.1][next_n.0] != '.' {
            n = next_n;
        }
    };
    n
}

/// Print the bathroom code for the instance.
pub fn print_code(board: &[Vec<char>]) {
    // Set up state.
    let c = board.len() / 2 + 1;
    let mut n = (c, c);

    // Read strings from the input file and process them.
    for line in aoc::input_lines() {
        n = walk_line(board, n, &line);
        print!("{}", board[n.1][n.0]);
    }
    println!("");
}

/// Read the board for the part, returning a nested
/// vector of the board surrounded by stops (`.`).
pub fn read_board(part: usize) -> Vec<Vec<char>> {
    // Set up state.
    let mut result = Vec::new();
    // Read each line of the board file.
    let board_lines =
        aoc::input_file_lines(&format!("board{}.txt", part))
        .expect("could not open board file");
    for line in board_lines {
        // Start a new row, add left boundary.
        let mut row = Vec::new();
        row.push('.');

        // Wrap up the characters of the row.
        let chars = line.chars().collect::<Vec<char>>();
        row.extend_from_slice(&chars);

        // Push the right boundary and save the row.
        row.push('.');
        result.push(row);
    };

    // Add the top and bottom boundaries.
    if result.is_empty() {
        panic!("empty board");
    };
    let nrow = result[0].len();
    let mut stops = Vec::with_capacity(nrow);
    stops.resize(nrow, '.');
    result.insert(0, stops.clone());
    result.push(stops.clone());

    // Return the result.
    result
}

/// Read the appropriate board, then print its code.
pub fn main() {
    let part = aoc::get_part();
    let board = read_board(part);
    print_code(&board);
}
