use std::{io::{self, BufReader, Read}, fs::File};
use std::collections::HashSet;
use std::fmt;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Command{
    dir: Direction,
    steps: usize,
}


#[derive(Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y

    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        write!(f, "{}, {}", self.x, self.y)
    }
}


impl Position {
    fn new() -> Self{
        Position {
            x: 0,
            y: 0,
        }
    }
    fn from_values(x: i32, y: i32) -> Self {
        Position {
            x,
            y,
        }
    }
}

struct Rope {
    head: Position,
    tail: Position,
    knots: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    fn new(nr_knots: usize) -> Self {
        Rope {
            head: Position::new(), 
            tail: Position::new(), // obsolete after part 2
            knots: vec![Position::new();nr_knots - 1], // head is separate  
            visited: HashSet::from([Position::new()]),

        }
    }

    fn go(&mut self, cmds: Vec<Command>) {
        for cmd in cmds {
            for _ in 0..cmd.steps {
                self.move_head(&cmd.dir);
                self.tail_follow();
            }
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }
    }

    fn tail_follow(&mut self) {
        let mut current = &self.head;

        for knot in self.knots.iter_mut() {

            let (dx, dy) = (current.x - knot.x, current.y - knot.y);
            if dx.abs() > 1 || dy.abs() > 1 {
                knot.x += dx.signum();
                knot.y += dy.signum();
                //self.visited.insert(Position::from_values(knot.x, knot.y));
            }
            current = knot;
            // let (dx, dy) = (self.head.x - self.tail.x, self.head.y - self.tail.y);
            // if dx.abs() > 1 || dy.abs() > 1 {
            //     self.tail.x += dx.signum();
            //     self.tail.y += dy.signum();
            //     self.visited.insert(Position::from_values(self.tail.x, self.tail.y));
            // }
    
        }
        self.visited.insert(Position::from_values(self.knots.last().unwrap().x, self.knots.last().unwrap().y));
    }
}

fn part_1(input: &str) -> u32 {
    
    let mut rope = Rope::new(2);
    let cmds: Vec<Command> = input
        .lines()
        .map(|line| line.split_once(' '))
        .map(|split| match split {
            Some((d, s)) => {
                let dir = match d {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Panic, bad direction")
                };
                let steps = s.parse::<usize>().unwrap();
                Command {dir,steps}
            }
            None => panic!("Panic, bad split"),
        }
        ).collect();
    rope.go(cmds);
    rope.visited.len() as u32
}

fn part_2(input: &str) -> u32 {
    let mut rope = Rope::new(10);
    let cmds: Vec<Command> = input
        .lines()
        .map(|line| line.split_once(' '))
        .map(|split| match split {
            Some((d, s)) => {
                let dir = match d {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Panic, bad direction")
                };
                let steps = s.parse::<usize>().unwrap();
                Command {dir,steps}
            }
            None => panic!("Panic, bad split"),
        }
        ).collect();
    rope.go(cmds);
    rope.visited.len() as u32
}

fn ropes() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day09/day09.txt")?).read_to_string(&mut input).unwrap();
     println!("Part1: {}", part_1(input.as_str()));
     println!("Part2: {}", part_2(input.as_str()));
    Ok(())
}


pub fn run() {
    match ropes() {
        _ => (),
    }
}

