use std::fs::File;
use std::str::FromStr;
use std::io::{self, BufReader, Read};
use itertools::Itertools;

#[derive(Debug)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

#[derive(Debug)]
struct PacketErr;

impl FromStr for Packet {
    type Err = PacketErr;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [[[[],[0,2,4]],7,5,0]]
        // [ - Spawn new list
        // ] - Close current list
        // , - 
        let mut stack = vec![];
        let mut list = vec![];
        let mut value = None;


        for c in s.chars() {
            match c {
                '[' => { 
                    stack.push((std::mem::take(&mut list), value.take()))
                },

                ']' => {
                    if let Some(val) = value.take() {
                        list.push(Packet::Value(val))
                    }
                },
                '0'..='9' => {
                    value = Some(match value.take() {
                        None => c as i32,
                        Some(val) => val,
                    })
                },
                ',' => {
                    if let Some(val) = value.take() {
                    }
                },
                _   => panic!("Bad character in packet string"),
            };
        }
        Ok(Packet::Value(1))
    }
}

struct PacketPair {
    first: Packet,
    second: Packet,
}

fn packets() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day13/day13.txt")?)
        .read_to_string(&mut input)
        .unwrap();

    let test = input
        .split("\n\n")
        .flat_map(|input| input.lines())
        .map( |line| line.parse::<Packet>().unwrap())
        .collect_vec()
        ;



    println!("Part1:");
    println!("Part2:");

    Ok(())
}

pub fn run() {
    match packets() {
        _ => (),
    }
}

