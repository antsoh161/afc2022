#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::{self, BufReader, Read}, fs::File};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cost(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

struct CPU {
    program: Vec<Instruction>,
    register_x: i32,
    cycles: u32,
    watchpoints: Vec<u32>,
    signal_strength_sum: i32,
    renderer: bool,
}

impl CPU { 
    fn new() -> Self {
        CPU {
            program: vec![],
            register_x: 1,
            cycles: 0,
            watchpoints: vec![],
            signal_strength_sum: 0,
            renderer: false,
        }
    }

    fn load_program(&mut self, input: &str) {
        self.program = input
            .lines()
            .map(|line| line.split_whitespace())
            .map(|mut words| {
                let inst = words.next().unwrap();
                match inst {
                    "noop" => Instruction::Noop,
                    "addx" => Instruction::Addx(words.next().unwrap().parse::<i32>().unwrap()),
                    _ => panic!("Bad instruction")
                }}
            ).collect();
    }

    fn set_watchpoints(&mut self, wp: Vec<u32>) {
        self.watchpoints = wp;
        self.watchpoints.sort();
        self.watchpoints.reverse();
    }

    fn spin(&mut self, n: u32) {
        for i in 0..n {
            if self.renderer {
                if ((self.cycles % 40) as i32 - self.register_x).abs() <= 1 {
                    print!("#");
                }
                else {
                    print!(".");
                }
            } 
            self.cycles += 1;
            if self.watchpoints.last().is_some() && self.cycles == *self.watchpoints.last().unwrap() {
                if self.renderer {
                    println!();
                }
                self.signal_strength_sum += self.cycles as i32 * self.register_x;
                self.watchpoints.pop();
            }
        }
    }

    fn toggle_renderer(&mut self) {
        self.renderer = !self.renderer;
    }

    fn execute_program(&mut self){
        self.program.reverse();
        for _ in 0..self.program.len() {
            let inst = self.program.pop().unwrap();
            match inst {
                Instruction::Noop => self.spin(inst.cost()),
                Instruction::Addx(value) => {
                    self.spin(inst.cost());
                    self.register_x += value;
                    // println!("{}", self.register_x);
                },
            }
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut cpu = CPU::new();
    cpu.load_program(input);
    cpu.set_watchpoints(vec![20, 60, 100, 140, 180, 220]);
    cpu.execute_program();
    cpu.signal_strength_sum    
}

fn part_2(input: &str) -> i32 {
    println!("Part2: ");
    let mut cpu = CPU::new();
    cpu.load_program(input);
    cpu.set_watchpoints(vec![40, 80, 120, 160, 200, 240]);
    cpu.toggle_renderer();
    cpu.execute_program();
    0
}

fn cpu_tasks() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day10/day10.txt")?).read_to_string(&mut input).unwrap();
    

     println!("Part1: {}", part_1(input.as_str()));
     part_2(input.as_str());
    Ok(())
}


pub fn run() {
    match cpu_tasks() {
        _ => (),
    }
}
