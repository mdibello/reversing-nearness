extern crate rand;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::collections::HashMap;

mod solution;
use solution::*;

#[derive(Hash, PartialEq, Eq)]
enum MutationLevel {
    LOW,
    NORMAL,
    HIGH,
}

fn main() -> std::io::Result<()> {

    // RUNTIME PARAMETERS
    let num_gens = 1000;
    let gen_size = 200;
    let gen_dist = vec![0.2, 0.2, 0.1, 0.1, 0.1, 0.1, 0.05, 0.05, 0.05, 0.05];
    let stag_lim = 100;
    let mut mut_exps: HashMap<MutationLevel, u32> = HashMap::new();
    mut_exps.insert(MutationLevel::LOW, 0);
    mut_exps.insert(MutationLevel::NORMAL, 1);
    mut_exps.insert(MutationLevel::HIGH, 2);

    let mut solutions: Vec<Solution> = Vec::new();
    let mut buffer = String::new();

    {
        let mut f = File::open("solution.txt")?;
        f.read_to_string(&mut buffer)?;
    }

    let saved_solutions = buffer.split("\n;\n").collect::<Vec<&str>>();

    for s in saved_solutions {
        if s.len() > 0 {
            let temp: String = s.replace("\n", "").replace(" ", "").replace("(", "").replace(")", "");
            let strs: Vec::<&str> = temp.split(",").collect::<Vec<&str>>();
            solutions.push(Solution::load(strs));
        }
    }

    loop {
        for i in 0..25 {
            print!("\nBEGINNING WORK ON GRID SIZE {} ", i+6);
            let grid_size = (solutions[i].clone().size() as f64).sqrt() as u32;
            let mut mut_lvl = MutationLevel::NORMAL;
            let mut num_mutations =  grid_size.pow(mut_exps[&mut_lvl]);
            let mut stagnant_count = 0;
            let mut generation: Vec<Solution> = solutions[i].generate(gen_size, num_mutations);
            generation.sort();
            for _ in 0..num_gens+1 {
                if stagnant_count > stag_lim {
                    if mut_lvl == MutationLevel::NORMAL {
                        mut_lvl = MutationLevel::LOW;
                        num_mutations = grid_size.pow(mut_exps[&mut_lvl]);
                        stagnant_count = 0;
                    } else if mut_lvl == MutationLevel::LOW {
                        mut_lvl = MutationLevel::HIGH;
                        num_mutations = grid_size.pow(mut_exps[&mut_lvl]);
                        stagnant_count = 0;
                    } else if mut_lvl == MutationLevel::HIGH {
                        mut_lvl = MutationLevel::NORMAL;
                        num_mutations = grid_size.pow(mut_exps[&mut_lvl]);
                        stagnant_count = 0;
                    }
                }
                match mut_lvl {
                    MutationLevel::LOW => print!("_"),
                    MutationLevel::NORMAL => print!("-"),
                    MutationLevel::HIGH => print!("^"),
                }
                std::io::stdout().flush()?;
                let mut new_generation: Vec<Solution> = Vec::new();
                for j in 0..gen_dist.len() {
                    let num_children = (gen_dist[j] * (gen_size as f64)) as u32 - 1;
                    new_generation.push(generation[j].clone());
                    new_generation.append(&mut generation[j].generate(num_children, num_mutations));
                }
                generation = new_generation;
                generation.sort();
                if generation[0] < solutions[i] {
                    println!("\n>>>Found better solution for grid size {}: {} -> {}", i+6, solutions[i].clone().eval(), generation[0].clone().eval());
                    solutions[i] = generation[0].clone();
                    update(solutions.clone())?;
                    stagnant_count = 0;
                } else {
                    stagnant_count += 1;
                }
            }
        }
    }

    Ok(())
}

fn update(solutions: Vec<Solution>) -> std::io::Result<()> {
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("new_solution.txt")
            .unwrap();

        let size = solutions.clone().len();
        let mut i = 0;
        for s in solutions {
            write!(file, "{}", s);
            if i != size - 1 {
                write!(file, ";");
            }
            write!(file, "\n");
            i += 1;
        }
    }

    fs::rename("new_solution.txt", "solution.txt")?;

    Ok(())
}
