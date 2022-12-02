
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_elves() -> io::Result<()> {
    let file = File::open("src/afc2022/day1.txt")?;
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    //let mut highest = 0;
    let mut arr_highest: [u32; 3] = [0; 3];

    for line in reader.lines() {
        match line {
            Ok(valid) => {
                match valid.parse::<u32>() {
                    Ok(value) => {
                        sum += value;
                    },
                    Err(_) => {
                        arr_highest.sort();
                        for i in arr_highest.iter_mut() {
                            if sum > *i {
                                *i = sum;
                                break;
                            }
                        }
                        sum = 0;
                    }
                }
            }
            Err(err) => { println!("Error: {}", err);}
                       
        }
    }
    println!("{:?}", arr_highest);
    let highest: u32 = arr_highest.iter().sum();
    println!("Highest calories are: {}", highest);

    Ok(())
}

pub fn run() {
    match read_elves() {
        Err(err) => {println!("Error: {}", err); }
        _ => ()
    }
}
