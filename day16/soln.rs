// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 16.

extern crate aoc;

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

pub fn checksum(src: String) -> String {
    let mut sum = src.clone();
    loop {
        let mut new_sum = String::new();
        let sum_chars = sum.chars().collect::<Vec<char>>();
        let nchars = sum_chars.len();
        assert!(nchars >= 2 && nchars & 1 == 0);
        for i in 0..nchars >> 1 {
            let c = match (sum_chars[i << 1], sum_chars[(i << 1) + 1]) {
                ('0', '0') => '1',
                ('0', '1') => '0',
                ('1', '0') => '0',
                ('1', '1') => '1',
                _ => panic!("unexpected pair in sum")
            };
            new_sum.push(c);
        };
        sum = new_sum;
        if sum.len() & 1 == 1 {
            break;
        }
    };
    sum 
}

#[test]
fn test_checksum() {
    assert!(checksum("110010110100".to_string()) == "100".to_string());
}

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert!(args.len() == 3);
    let fill: usize = args[1].parse().expect("could not parse fill");
    let mut data = args[2].to_string();
    while data.len() < fill {
        data = fill_step(data);
    };
    let data = &data[0..fill].to_string();
    print!("{}\n", checksum(data.to_string()));
}
