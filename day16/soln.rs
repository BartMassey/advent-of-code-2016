// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 16.

extern crate aoc;

/// Run one dragon-curve step and return the resulting
/// string.
fn fill_step(src: String) -> String {
    let mut dst = src.clone();
    dst.push('0');
    let src_chars = src.chars().collect::<Vec<char>>();
    for i in (0..src_chars.len()).rev() {
        let c = match src_chars[i] {
            '0' => '1',
            '1' => '0',
            _ => panic!("unexpected char in data")
        };
        dst.push(c);
    };
    dst
}

#[test]
fn test_fill_step() {
    let tests = [
        ("1", "100"),
        ("0", "001"),
        ("11111", "11111000000"),
        ("111100001010", "1111000010100101011110000") ];
    for &(src, dst) in tests.iter() {
        assert!(fill_step(src.to_string()) == dst.to_string());
    };
}

/// Return the "checksum" of some data.
pub fn checksum(src: String) -> String {
    let mut sum = src.clone();
    while sum.len() & 1 == 0 {
        let mut new_sum = String::new();
        let sum_chars = sum.chars().collect::<Vec<char>>();
        let nchars = sum_chars.len();
        assert!(nchars >= 2 && nchars & 1 == 0);
        for i in 0..nchars >> 1 {
            let c = match (sum_chars[i << 1], sum_chars[(i << 1) + 1]) {
                ('0', '0') | ('1', '1') => '1',
                ('0', '1') | ('1', '0') => '0',
                _ => panic!("unexpected pair in sum")
            };
            new_sum.push(c);
        };
        sum = new_sum;
    };
    sum 
}

#[test]
fn test_checksum() {
    assert!(checksum("110010110100".to_string()) == "100".to_string());
}

/// Run the fill-and-checksum on the given inputs.
pub fn main() {
    let args = aoc::get_args();
    assert!(args.len() == 2);
    let fill: usize = args[0].parse().expect("could not parse fill");
    let mut data = args[1].to_string();

    // Fill to specified size or more.
    while data.len() < fill {
        data = fill_step(data);
    };

    // Truncate the fill to specified size.
    let data = &data[0..fill].to_string();

    // Checksum.
    println!("{}", checksum(data.to_string()));
}
