extern crate rand;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod solution;
use solution::*;

fn main() -> std::io::Result<()> {

    let mut solutions: Vec<Solution> = Vec::new();

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open("solution.txt")
    //     .unwrap();

    // for i in 6..31 {
    //     writeln!(file, "{}\n", Solution::new(i));
    // }

    let mut f = File::open("solution.txt")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let saved_solutions = buffer.split("\n\n").collect::<Vec<&str>>();

    for s in saved_solutions {
        if s.len() > 0 {
            let temp: String = s.replace("\n", "").replace(" ", "").replace("(", "").replace(")", "");
            let strs: Vec::<&str> = temp.split(",").collect::<Vec<&str>>();
            println!("{}\n", Solution::load(strs.clone()));
            solutions.push(Solution::load(strs));
        }
    }

    Ok(())
}
