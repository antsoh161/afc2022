#![allow(dead_code)]
#![allow(unused_variables)]

pub mod afc2022;

use crate::afc2022::*;

fn main() {
    let fvec = make_days();

    for (i, day) in fvec.iter().enumerate() {
        println!("---- Advent of Code - Day {} ----", i+1);
        day();
        println!();
    }
    
}
fn make_days() -> Vec<&'static dyn Fn()> {
    // let mut fvec: Vec<&dyn Fn()> = vec![];
    let mut fvec: Vec<&dyn Fn()> = Vec::new();
    fvec.push(&day01::run);
    fvec.push(&day02::run);
    fvec.push(&day03::run);
    fvec.push(&day04::run);
    fvec.push(&day05::run);
    fvec.push(&day06::run);
    fvec.push(&day07::run);
    fvec.push(&day08::run);
    fvec.push(&day09::run);
    fvec.push(&day10::run);
    fvec.push(&day11::run);
    fvec.push(&day12::run);
    fvec.push(&day13::run);
    fvec
}
