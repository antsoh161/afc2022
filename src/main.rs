pub mod afc2022;

use crate::afc2022::*;

fn main() {
    let fvec = make_days();

    for (i, day) in fvec[0..].iter().enumerate() {
        println!("---- Advent of Code - Day {} ----", i+1);
        day();
        println!();
    }

}

fn make_days() -> Vec<&'static dyn Fn()> {
    // let mut fvec: Vec<&dyn Fn()> = vec![];
    let mut fvec: Vec<&dyn Fn()> = Vec::new();
    fvec.push(&day1::run);
    fvec.push(&day2::run);
    fvec.push(&day3::run);
    // fvec.push(&day4::run);
    // fvec.push(&day5::run);
    // fvec.push(&day6::run);
    // fvec.push(&day7::run);
    fvec
}
