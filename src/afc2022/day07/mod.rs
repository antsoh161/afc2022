#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{io::{self, BufReader, Read}, fs::File};
    
use std::collections::HashMap;
use std::str::FromStr;

struct AFCFile {
    size: u32,
}

fn part_1() {

}


fn intervals() -> io::Result<()> {
    
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day07/day07.txt")?).read_to_string(&mut input).unwrap();
    // println!("Part1: {}", part_1(input.as_str()));
    // println!("Part2: {}", part_2(input.as_str()));
    Ok(())
}


pub fn run() {
    match intervals() {
        _ => (),
    }
}
