
#![allow(dead_code)]
use std::{io::{self, BufReader, BufRead, Lines}, fs::File};
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


fn part_1(lines_in: Lines<BufReader<File>>) -> u32 {
    lines_in.map(|lines| lines.unwrap())
        .map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        a.chars()
        .find(|c| b.contains(*c))
        .map(get_prio)
        .unwrap()
    }).sum::<u32>()
}

fn part_2(lines_in: Lines<BufReader<File>>) -> u32 {
    lines_in.map(|lines| lines.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunks| chunks[0]
            .chars()
            .find(|badge| chunks[1].contains(*badge) && chunks[2].contains(*badge))
            .map(get_prio)
            .unwrap()
            )
            .sum::<u32>()
}

fn prioritize() -> io::Result<()> {
    
    println!("Part1: {:?}", part_1(BufReader::new(File::open("src/afc2022/day03/day03.txt")?).lines()));
    println!("Part1: {:?}", part_2(BufReader::new(File::open("src/afc2022/day03/day03.txt")?).lines()));
    
    Ok(())
}


pub fn run() {

    match prioritize() {
        _ => (),
    }
}



