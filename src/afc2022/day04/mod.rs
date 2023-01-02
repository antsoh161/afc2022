use std::{io::{self, BufReader, Read}, fs::File};

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap()) // maps to left, right
        .map(|(left, right)| (left.split_once('-').unwrap(), right.split_once('-').unwrap())) // maps to ((ls, le), (rs, re))
        .map(|((ls, le),( rs, re))| (ls.parse::<u32>().unwrap(), le.parse::<u32>().unwrap(), rs.parse::<u32>().unwrap(), re.parse::<u32>().unwrap())) // maps to <u32>(ls, le, rs, re)
        .map(|(ls, le, rs, re)| (ls <= rs && le >= re) || (rs <= ls && re >= le) )
        .filter(|is_true| *is_true)
        .count()
    
}

fn is_overlapping((ls, le, rs, re): (u32, u32, u32, u32)) -> bool {
    (ls >= rs && ls <= re
        || le >= rs && le <= re)
    || (rs >= ls && rs <= le
        || re >= ls && re <= le)
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap()) // maps to left, right
        .map(|(left, right)| (left.split_once('-').unwrap(), right.split_once('-').unwrap())) // maps to ((ls, le), (rs, re))
        .map(|((ls, le),( rs, re))| (ls.parse::<u32>().unwrap(), le.parse::<u32>().unwrap(), rs.parse::<u32>().unwrap(), re.parse::<u32>().unwrap())) // maps to <u32>(ls, le, rs, re)
        .map(is_overlapping)
        .filter(|is_true| *is_true)
        .count()
}

fn intervals() -> io::Result<()> {
    
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day04/day04.txt")?).read_to_string(&mut input).unwrap();
    println!("Part1: {}", part_1(input.as_str()));
    println!("Part2: {}", part_2(input.as_str()));
    Ok(())
}


pub fn run() {
    match intervals() {
        _ => (),
    }
}
