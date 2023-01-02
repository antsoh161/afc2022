
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{io::{self, BufReader, Read}, fs::File};

use itertools::Itertools;


fn find_marker(input: &str, window_size: usize) -> usize{
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .position(|win| {
             let uniques = win.iter().unique();
             win.len() == uniques.count()
            } 
        ).unwrap()
    + window_size
    
}

fn find_markers() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day06/day06.txt")?).read_to_string(&mut input).unwrap();
    println!("Part1: {}", find_marker(input.as_str(), 4));
    println!("Part2: {}", find_marker(input.as_str(), 14));
    Ok(())
}


pub fn run() {
    match find_markers() {
        _ => (),
    }
}
