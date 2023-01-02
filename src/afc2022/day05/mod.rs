#![allow(dead_code)]
#![allow(unused_variables)]
use std::{io::{self, BufReader, Read}, fs::File};


struct Stack {
    stack: Vec<Vec<char>>,
}

impl Stack {
    pub fn new(stack_string: &str) -> Self {
        let nr_columns = stack_string
            .lines()
            .last()
            .map(|line| line.split_whitespace())
            .unwrap()
            .map(|nr| nr.parse::<usize>().unwrap())
            .max()
            .unwrap();
        
        let mut stack: Vec<Vec<char>> = vec![vec![]; nr_columns ];

        for line in stack_string.lines().rev().skip(1) {
            for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c.is_alphabetic() {
                    stack[idx].push(c);
                }
            }
        }

        Stack{ stack }
    }

    fn move_crates(&mut self, nr: usize, from: usize, to: usize) {
        for n in 0..nr {
            let b = self.stack[from].pop();
            self.stack[to].push(b.unwrap());
        }
    }

    fn move_multiple_crates(&mut self, nr: usize, from: usize, to: usize) {
        let idx = self.stack[from].len()-nr;
        let mut moving = self.stack[from].split_off(idx);
        self.stack[to].append(&mut moving); 
    }

    fn print_stack(&self) {
        for (idx, col) in self.stack.iter().enumerate() {
            print!("{}| ", idx);
            for c in col {
                print!("{} ", c);
            }
            println!("");
        }
    }
    fn top_crates(&self) -> String {
        let mut s: String = String::new();
        for col in self.stack.iter() {
            s.push(col.last().unwrap().to_owned());
        }
        s
    }
}


fn part_1(input: &str) -> String {

    let (stack_str, cmd_str) = input.split_once("\n\n").unwrap();
    
    let mut stack = Stack::new(stack_str);

    let cmds = cmd_str
        .lines()
        .map(|line| line.split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok()) // Filter for numbers
            .collect::<Vec<usize>>() // collect vector of [nr, from, to]
        ).map(|cmd_v| (cmd_v[0], cmd_v[1] - 1, cmd_v[2] - 1) ); // make tuple and fix index

    for (nr, from, to) in cmds {
        stack.move_crates(nr, from, to);
    }
    stack.top_crates()
}

fn part_2(input: &str) -> String{
    let (stack_str, cmd_str) = input.split_once("\n\n").unwrap();
    
    let mut stack = Stack::new(stack_str);

    let cmds = cmd_str
        .lines()
        .map(|line| line.split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok()) // Filter for numbers
            .collect::<Vec<usize>>() // collect vector of [nr, from, to]
        ).map(|cmd_v| (cmd_v[0], cmd_v[1] - 1, cmd_v[2] - 1) ); // make tuple and fix index


    for (nr, from, to) in cmds {
        stack.move_multiple_crates(nr, from, to);
    }
    stack.top_crates()
}

fn intervals() -> io::Result<()> {
    
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day05/day05.txt")?).read_to_string(&mut input).unwrap();
    println!("Part1: {}", part_1(input.as_str()));
    println!("Part2: {}", part_2(input.as_str()));
    Ok(())
}


pub fn run() {
    match intervals() {
        _ => (),
    }
}
