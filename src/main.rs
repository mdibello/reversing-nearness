extern crate rand;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

mod solution;
use solution::*;

fn main() -> std::io::Result<()> {

    // RUNTIME PARAMETERS
    let num_gens = 1000;
    let gen_size = 200;
    let mut_coef = 10;
    let gen_dist = vec![0.2, 0.2, 0.1, 0.1, 0.1, 0.1, 0.05, 0.05, 0.05, 0.05];

    let mut solutions: Vec<Solution> = Vec::new();
    let mut buffer = String::new();

    {
        let mut f = File::open("solution.txt")?;
        f.read_to_string(&mut buffer)?;
    }

    let saved_solutions = buffer.split("\n\n").collect::<Vec<&str>>();

    for s in saved_solutions {
        if s.len() > 0 {
            let temp: String = s.replace("\n", "").replace(" ", "").replace("(", "").replace(")", "").replace(";", "");
            let strs: Vec::<&str> = temp.split(",").collect::<Vec<&str>>();
            solutions.push(Solution::load(strs));
        }
    }

    loop {
        for i in 0..25 {
            print!("\nBEGINNING WORK ON GRID SIZE {} ", i+6);
            let num_mutations = solutions[i].clone().size() as u32 * mut_coef;
            let mut generation: Vec<Solution> = solutions[i].generate(gen_size, num_mutations);
            generation.sort();
            for _ in 0..num_gens+1 {
                print!("."); std::io::stdout().flush()?;
                let mut new_generation: Vec<Solution> = Vec::new();
                for j in 0..gen_dist.len() {
                    let num_children = (gen_dist[j] * (gen_size as f64)) as u32 - 1;
                    let idx = generation.len() - (j + 1);
                    new_generation.push(generation[idx].clone());
                    new_generation.append(&mut generation[idx].generate(num_children, num_mutations));
                }
                generation = new_generation;
                generation.sort();
                if generation[0] < solutions[i] {
                    println!("\n>>>Found better solution for grid size {}: {} -> {}", i+6, solutions[i].clone().eval(), generation[0].clone().eval());
                    solutions[i] = generation[0].clone();
                    update(solutions.clone())?;
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
            write!(file, "\n\n");
            i += 1;
        }
    }

    fs::rename("new_solution.txt", "solution.txt")?;

    Ok(())
}
