
#![allow(dead_code)]
use std::{io::{self, BufReader, BufRead}, fs::File};

const UPPERCASE_OFFSET: u32 = 27;
const LOWERCASE_OFFSET: u32 = 1;

fn get_prio(item: char) -> u32{
    if item.is_lowercase() {
        (item as u8 - b'a') as u32 + LOWERCASE_OFFSET
    }
    else {
        (item as u8 - b'A') as u32 + UPPERCASE_OFFSET
    }
}

fn prioritize_part1() -> io::Result<()> {
    let lines = BufReader::new(File::open("./src/afc2022/day3.txt")?).lines().map(|lines| lines.unwrap());
    let sum: u32 = lines.map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        a.chars()
        .find(|c| b.contains(*c))
        .map(get_prio)
        .unwrap()
    }).sum();

    println!("{sum}");
    Ok(())
}

pub fn run() {
    match prioritize_part1() {
        Err(err) => {!panic!("{}", err);},
        _ => (),
    }
}



