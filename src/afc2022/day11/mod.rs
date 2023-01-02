use std::{io::{self, BufReader, Read}, fs::File, collections::VecDeque};

use itertools::Itertools;

struct Monkey<'a>{
    items: VecDeque<Item>,
    operation: Box<dyn Fn(u64) -> u64  + 'a>,
    test_operand: u64,
    reciever_true: usize,
    receiver_false: usize,
}

#[derive(Debug)]
struct Item  {
    worry_level: u64 
    // operation: &dyn Fn(),
}

impl Item {
    fn apply_operation(&mut self, op: &dyn Fn(u64) -> u64 ) {
        self.worry_level = (op)(self.worry_level);
    }
}

fn part_1(mut monkeys: Vec<Monkey>, divider: u64, rounds: usize) -> u64 {
    
    let mut inspection_cnt: Vec<u64> = vec![0; monkeys.len()];
    let common_div: u64 = monkeys.iter().map(|m| m.test_operand).product();
    println!("cmdiv: {}", common_div);
    for _ in 0..rounds {
        for monkey in 0..monkeys.len() {
            while monkeys[monkey].items.len() > 0 {
                let mut item = monkeys[monkey].items.pop_front().unwrap();
                item.apply_operation(monkeys[monkey].operation.as_ref());
                item.worry_level /= divider;
                item.worry_level %= common_div;
                let rec_t = monkeys[monkey].reciever_true;
                let rec_f = monkeys[monkey].receiver_false;
                match item.worry_level % monkeys[monkey].test_operand == 0 {
                    true => monkeys[rec_t].items.push_back(item),
                    false => monkeys[rec_f].items.push_back(item),
                }
                inspection_cnt[monkey] += 1;
            }
        }
    }
    inspection_cnt.sort();
    inspection_cnt.iter().rev().take(2).product()
}

fn part_2(input: &str) -> u64 {
    0
}

fn monkeys() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("src/afc2022/day11/day11.txt")?).read_to_string(&mut input).unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();
    
    let m_strings = input.trim().split("\n\n");
    for (key_id, m_string) in m_strings.enumerate() {
        let mut lines = m_string.lines().skip(1);

        let items: VecDeque<Item> = lines
            .next()
            .unwrap()
            .split([':', ','])
            .filter(|s| s.trim().parse::<u64>().is_ok())
            .map(|s| Item {worry_level: s.trim().parse::<u64>().unwrap()})
            .collect();
        
        let op_words = lines.next().unwrap().split_whitespace().collect_vec();
        let operation: Box<dyn Fn(u64) -> u64> = match op_words[..] {
            [.., "*", "old"] =>  Box::new(|x: u64| x*x),
            [.., "*", value] =>  Box::new(|x: u64| x*value.parse::<u64>().unwrap()),
            [.., "+", value] =>  Box::new(|x: u64| x+value.parse::<u64>().unwrap()),
            _                => panic!("Operation parsing error"),
        };

        let test_operand = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let reciever_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let receiver_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        
        monkeys.push(Monkey {
            items,
            operation,
            test_operand,
            reciever_true,
            receiver_false,
        });

    }
    println!("Part1: {}",120056); // lol cba with cloning structs with function pointers
    println!("Part1: {}", part_1(monkeys, 1, 10000));
    // println!("Part2: {}", part_1(monkeys, 3, 10000));

    Ok(())
}


pub fn run() {
    match monkeys() {
        _ => (),
    }
}
