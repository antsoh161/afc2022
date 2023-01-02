#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::{self, BufReader, Read}, fs::File};
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<u32>>,
    visibility_set: HashSet<(usize, usize)>,
}

impl Grid {
    pub fn new(input: &str) -> Self{
        let data = input
            .lines().map(|line| {
                let mut v: Vec<u32> = vec![];
                for c in line.chars() {
                    v.push(c.to_digit(10).unwrap());
                }
                v
            }).collect_vec();

        let visibility_set: HashSet<(usize, usize)> = HashSet::new();

        Grid{ data,
              visibility_set,
        }
    }

    fn is_edge(&self, x: usize, y: usize ) -> bool {
        match (x,y) {
            (0,_) => true,
            (max,_) if max == self.data.len()-1 => true,
            (_,0) => true,
            (_,max) if max == self.data[0].len()-1 => true,
            _ =>  false,
        }
    }

    fn not_blocked(&self, x: usize, y: usize) -> bool {
        // check left
        if self.data[x][..y].iter().max().unwrap() < &self.data[x][y] {
            return true;
        }
        // check right
        if self.data[x][y+1..].iter().max().unwrap() < &self.data[x][y] {
            return true;
        }    
        //check up
        if self.data[..x].iter().map(|row| row[y]).collect::<Vec<u32>>().iter().max().unwrap() < &self.data[x][y] {
            return true;
        }
        //check down
        if self.data[x+1..].iter().map(|row| row[y]).collect::<Vec<u32>>().iter().max().unwrap() < &self.data[x][y] {
            return true;
        }
        false
    }

    fn populate_visibility(&mut self) {
        for (x, row) in self.data.iter().enumerate() {
            for (y, height) in row.iter().enumerate() {
                // println!("Checking: {}, {} = {}", x,y, self.data[x][y]);
                if self.is_edge(x,y) 
                   || self.not_blocked(x, y)
                {
                    self.visibility_set.insert((x,y));
                }
            }
        }
    }


    fn get_score(&self, x: usize, y: usize) -> u32 {
        let left = self.data[x][..y].iter().rev().find_position(|first| **first >= self.data[x][y]);
        let right = self.data[x][y+1..].iter().find_position(|first| **first >= self.data[x][y]);
        let up_v = self.data[..x].iter().map(|row| row[y]).collect::<Vec<u32>>(); 
        let up  = up_v.iter().rev().find_position(|first| **first >= self.data[x][y]);
        let down_v = self.data[x+1..].iter().map(|row| row[y]).collect::<Vec<u32>>();
        let down = down_v.iter().find_position(|first| **first >= self.data[x][y]);

        let mut score = 1;
        match left {
            None => {
                if !self.is_edge(x, y) {
                    score *= y as u32;
                }
            }, 
            Some((pos, val)) => {score *= pos as u32 + 1; }, // Tree
        }
        match right {
            None => {
                if !self.is_edge(x, y){
                    score *= (self.data[x].len() - y - 1) as u32;
                }
            },
            Some((pos, val)) => {score *= pos as u32 + 1; }
        }
        match up {
            None => {
                if !self.is_edge(x, y){
                    score *= x as u32;
                }
            },
            Some((pos, val)) => { score *= pos as u32 + 1; }
        }
        match down {
            None => { 
                if !self.is_edge(x, y){
                    score *= (self.data.len() - x - 1) as u32;
                }
            }
            Some((pos, val)) => { score *= pos as u32 + 1;},
        }
        score
        
    }

    fn calculate_scenic_score(&self) -> u32 {
        let mut highest_score = 0;
        for (x, row) in self.data.iter().enumerate() {
            for (y, height) in row.iter().enumerate() {
                let score = self.get_score(x,y);
                if score > highest_score {
                    highest_score = score;
                }
            }
        }
        highest_score
    }

    fn print_grid(&self){
        for row in self.data.iter() {
            for col in row {
                print!("{}", col);
            }
            println!("");
        }
    }
} 
   

fn part_1(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    // grid.print_grid();
    grid.populate_visibility();
    grid.visibility_set.len() as u32   
}

fn part_2(input: &str) -> u32 {
    let grid = Grid::new(input);
    grid.calculate_scenic_score()
}

fn intervals() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day08/day08.txt")?).read_to_string(&mut input).unwrap();
     println!("Part1: {}", part_1(input.as_str()));
     println!("Part2: {}", part_2(input.as_str()));
    Ok(())
}


pub fn run() {
    match intervals() {
        _ => (),
    }
}

