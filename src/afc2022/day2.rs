

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

use Hand::*;

impl Hand {
    fn new(c: &str) -> Self {
        match c {
            "A" | "X" => {Rock},
            "B" | "Y" => {Paper},
            "C" | "Z" => {Scissor},
            _ => {panic!("Invalid input into hand::new()");}

        }
    }

    fn pick_by_marker(o: Self, m: &str) -> Self {
        match m {
            "X" => o.beats(),
            "Y" => o,
            "Z" => o.beaten_by(),
            _ => {panic!("Invalid input into hand::new8)");}
        }
    }

    fn beats(&self) -> Self {
        match *self {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match *self {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        }
    }

    fn handtype_points(&self) -> u32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
        
    }
    
}

fn play_hand(opponent: &Hand, me: &Hand) -> u32 {
    let mut points = 0;

    match (me.beats(), opponent.beats()) {
        _ if me.beats() == *opponent => points += 6,
        _ if opponent.beats() == *me => points += 0,
        _ => points += 3, //Draw
    };

    return me.handtype_points() + points;
}


fn rock_paper_scissors() -> io::Result<()> {
    let file = File::open("src/afc2022/day2.txt")?;
    let reader = BufReader::new(file);

    let mut part1_points: u32 = 0;
    let mut part2_points: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(valid) => {
                if let Some((opponent, me)) = valid.split_once(' ') {
                    part1_points += play_hand(&Hand::new(opponent), &Hand::new(me));
                    part2_points += play_hand(&Hand::new(opponent), &Hand::pick_by_marker(Hand::new(opponent), me));
                }
            }
            Err(err) => { println!("Error: {}", err);}
        }
    }
    println!("Part1 Points: {}", part1_points);
    println!("Part2 Points: {}", part2_points);
    Ok(())
}

pub fn run() {
    match rock_paper_scissors() {
        Err(err) => {println!("Error: {}", err); }
        _ => ()
    }
}
