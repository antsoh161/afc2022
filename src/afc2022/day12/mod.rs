use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::ops::Add;

// use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn assign(&mut self, x: i32, y: i32) {
        self.0 = x;
        self.1 = y;
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

struct Graph {
    data: HashMap<Pos, u8>,
    adjacency_data: HashMap<Pos, Vec<Pos>>,
    start: Pos,
    goal: Pos,
}

impl Graph {
    fn new(data: HashMap<Pos, u8>, start: Pos, goal: Pos) -> Self {
        Graph {
            data: HashMap::new(),
            adjacency_data: HashMap::new(),
            start,
            goal,
        }
    }
    fn print_graph(&self) {
        for (pos, adjs) in self.adjacency_data.iter() {
            print!("Pos: ({}, {}): Adjacents: ", pos.0, pos.1);
            for adj in adjs.iter() {
                print!("({}, {})", adj.0, adj.1);
            }
            println!();
        }
    }
}

fn bfs(graph: &Graph) -> Option<usize> {
    
    let mut queue: VecDeque<(Pos,usize)> = VecDeque::new();
    queue.push_front((graph.start, 0));
    
    let mut visited = HashSet::new();
    visited.insert(graph.start);

    while let Some((node, cost)) = queue.pop_front() {
        if node == graph.goal {
            return Some(cost);
        }
        for next in graph.adjacency_data[&node].iter() {
            if visited.contains(&next){
                continue;
            }
            queue.push_back((*next, cost+1));
            visited.insert(*next);
        }
    }
    None
}

fn make_graph(input: &str) -> Graph {
    // let n: i32 = input.lines().map(|line| line.chars().count()).collect_vec().iter().sum();

    let mut map: HashMap<Pos, u8> = HashMap::new();
    let mut start = Pos(0, 0);
    let mut goal = Pos(0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start.assign(i as i32, j as i32);
                    map.insert(Pos(i as i32, j as i32), 'a' as u8)
                }
                'E' => {
                    goal.assign(i as i32, j as i32);
                    map.insert(Pos(i as i32, j as i32), 'z' as u8)
                }
                _ => map.insert(Pos(i as i32, j as i32), c as u8),
            };
        }
    }

    let mut adj_map: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let  possible_adjacents = [Pos(1, 0), Pos(0, 1), Pos(-1, 0), Pos(0, -1)];
    let mut tmp_vec: Vec<Pos> = vec![];
    for pos in map.iter() {
        tmp_vec.clear();
        for p_adj in possible_adjacents.iter() {
            let check_idx = *pos.0 + *p_adj;
            if let Some(matched) = map.get(&check_idx){
                if (*matched as i32 - map[pos.0] as i32) <= 1 { // If 1 elevation above maximum
                    tmp_vec.push(check_idx);
                }
            }
        }
        adj_map.insert(pos.0.clone(), tmp_vec.clone());
    }

    Graph {
        data: map,
        adjacency_data: adj_map,
        start,
        goal,
    }
}

fn path_finder() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day12/day12.txt")?)
        .read_to_string(&mut input)
        .unwrap();
    let mut graph = make_graph(input.as_str());
    // graph.print_graph();
    println!("Part1: {}", bfs(&graph).unwrap());

    let mut lowest_cost: usize = std::usize::MAX;

    for (pos, height) in graph.data.iter() {
        if *height as char == 'a' {
            graph.start = *pos;
            if let Some(cost) = bfs(&graph)  {
                if cost < lowest_cost {
                    lowest_cost = cost;
                }
            }
        }
    }
    println!("Part2: {}", lowest_cost);

    Ok(())
}

pub fn run() {
    match path_finder() {
        _ => (),
    }
}
