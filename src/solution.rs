use rand::distributions::{Distribution, Uniform};
use std::cmp::Ordering;

#[derive(Clone, Eq)]
pub struct Solution {
    grid: Vec<u32>,
    eval: u64,
}

impl Solution {
    pub fn new(size: u32) -> Solution {
        let mut s: Solution = Solution {
            grid: Vec::new(),
            eval: 0,
        };
        for i in 0..size.pow(2) {
            s.grid.push(i as u32);
        }
        s.eval = s.clone().evaluate();
        return s;
    }

    pub fn load(strs: Vec<&str>) -> Solution {
        let mut s: Solution = Solution {
            grid: Vec::new(),
            eval: 0,
        };
        let size = (strs.len() as f64).sqrt() as u32;
        for i in 0..strs.len() {
            s.grid.push(name_to_index(strs[i], size));
        }
        s.eval = s.clone().evaluate();
        return s;
    }

    pub fn evaluate(mut self) -> u64 {
        let lower_bounds: Vec<u64> = vec![
            0, 10, 72, 816, 3800, 16902, 52528, 155840, 381672, 902550,
            1883244, 3813912, 7103408, 12958148, 22225500, 37474816,
            60291180, 95730984, 146469252, 221736200, 325763172,
            474261920, 673706892, 949783680, 1311600000, 1799572164,
            2425939956, 3252444776, 4294801980, 5643997650,
        ];
        let size = (self.grid.len() as f64).sqrt() as u32;
        let mut sum: u64 = 0;
        for idx_a in 0..self.grid.len() {
            for idx_b in idx_a..self.grid.len() {
                let orig_idx_1 = self.grid[idx_a as usize];
                let orig_x_a = orig_idx_1 % size;
                let orig_y_a = orig_idx_1 / size;
                let orig_idx_2 = self.grid[idx_b as usize];
                let orig_x_b = orig_idx_2 % size;
                let orig_y_b = orig_idx_2 / size;
                let new_x_a = (idx_a as u32) % size;
                let new_y_a = (idx_a as u32) / size;
                let new_x_b = (idx_b as u32) % size;
                let new_y_b = (idx_b as u32) / size;
                sum += distance(orig_x_a as i32, orig_y_a as i32, orig_x_b as i32, orig_y_b as i32, size as i32) * 
                        distance(new_x_a as i32, new_y_a as i32, new_x_b as i32, new_y_b as i32, size as i32);
            }
        }
        self.eval = sum - lower_bounds[(size - 1) as usize];
        return self.eval;
    }

    pub fn mutate(&mut self, n: u32) {
        let step = Uniform::new(0, self.grid.len());
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let idx_a = step.sample(&mut rng);
            let idx_b = step.sample(&mut rng);
            let temp = self.grid[idx_a];
            self.grid[idx_a] = self.grid[idx_b];
            self.grid[idx_b] = temp;
        }
        self.eval = self.clone().evaluate();
    }

    pub fn generate(&mut self, children: u32, mutations: u32) -> Vec<Solution> {
        let mut new_gen: Vec<Solution> = Vec::new();
        for _ in 0..children+1 {
            let mut child = self.clone();
            child.mutate(mutations);
            new_gen.push(child);
        }
        return new_gen;
    }

    pub fn size(self) -> usize {
        return (self.grid.len() as f64).sqrt() as usize;
    }

    pub fn eval(self) -> u64 {
        return self.eval;
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        self.eval.cmp(&other.eval)
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        self.eval == other.eval
    }
}

fn distance(x_a: i32, y_a: i32, x_b: i32, y_b: i32, size: i32) -> u64 {
    let dx;
    if (x_b - x_a).abs() <= size / 2 {
        dx = (x_b - x_a).abs();
    } else if x_a > x_b {
        dx = x_b + size - x_a;
    } else {
        dx = x_a + size - x_b;
    }

    let dy;
    if (y_b - y_a).abs() <= size / 2 {
        dy = (y_b - y_a).abs();
    } else if y_a > y_b {
        dy = y_b + size - y_a;
    } else {
        dy = y_a + size - y_b;
    }

    return (dx as u64).pow(2) + (dy as u64).pow(2);
}

fn index_to_name(n: u32, size: u32) -> String {
    let alphabet: Vec<&str> = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
        "U", "V", "W", "X", "Y", "Z", "1", "2", "3", "4",
    ];
    return String::from(alphabet[(n%size) as usize]) +
           &String::from(alphabet[(n/size) as usize]);
}

fn name_to_index(name: &str, size: u32) -> u32 {
    let alphabet: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4',
    ];
    return (alphabet.iter().position(|&r| r == name.chars().nth(1).unwrap()).unwrap() as u32 * size) +
            alphabet.iter().position(|&r| r == name.chars().nth(0).unwrap()).unwrap() as u32;
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = (self.grid.len() as f64).sqrt() as u32;
        let mut _cell_count = 0;
        for cell in &self.grid {
            if _cell_count % size == 0 {
                write!(f, "(");
            }
            write!(f, "{}", index_to_name(*cell, size));
            if (_cell_count + 1) % size != 0 {
                write!(f, ", ");
            } else {
                write!(f, ")");
                if _cell_count != (self.grid.len() - 1) as u32 {
                    write!(f, ",");
                }
                write!(f, "\n");
            }
            _cell_count += 1;
        }
        return write!(f, "");
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_distance_size_3() {
        assert_eq!(distance(0, 0, 1, 0, 3), 1);
        assert_eq!(distance(0, 0, 2, 0, 3), 1);
        assert_eq!(distance(0, 0, 0, 1, 3), 1);
        assert_eq!(distance(0, 0, 0, 2, 3), 1);
        assert_eq!(distance(0, 0, 1, 1, 3), 2);
        assert_eq!(distance(0, 0, 2, 2, 3), 2);
        assert_eq!(distance(2, 1, 1, 2, 3), 2);

        assert_eq!(distance(1, 0, 0, 0, 3), 1);
        assert_eq!(distance(2, 0, 0, 0, 3), 1);
        assert_eq!(distance(0, 1, 0, 0, 3), 1);
        assert_eq!(distance(0, 2, 0, 0, 3), 1);
        assert_eq!(distance(1, 1, 0, 0, 3), 2);
        assert_eq!(distance(2, 2, 0, 0, 3), 2);
        assert_eq!(distance(1, 2, 2, 1, 3), 2);
    }

    #[test]
    fn test_distance_size_5() {
        assert_eq!(distance(0, 0, 1, 0, 5), 1);
        assert_eq!(distance(0, 0, 2, 0, 5), 4);
        assert_eq!(distance(0, 0, 3, 0, 5), 4);
        assert_eq!(distance(0, 0, 4, 4, 5), 2);
        assert_eq!(distance(1, 0, 2, 0, 5), 1);
        assert_eq!(distance(1, 0, 3, 0, 5), 4);
        assert_eq!(distance(1, 0, 4, 0, 5), 4);
        assert_eq!(distance(1, 0, 4, 4, 5), 5);
        assert_eq!(distance(2, 0, 3, 0, 5), 1);
        assert_eq!(distance(2, 0, 4, 0, 5), 4);
        assert_eq!(distance(2, 0, 0, 1, 5), 5);
        assert_eq!(distance(2, 0, 4, 4, 5), 5);
        assert_eq!(distance(3, 4, 4, 4, 5), 1);
        
        assert_eq!(distance(1, 0, 0, 0, 5), 1);
        assert_eq!(distance(2, 0, 0, 0, 5), 4);
        assert_eq!(distance(3, 0, 0, 0, 5), 4);
        assert_eq!(distance(4, 4, 0, 0, 5), 2);
        assert_eq!(distance(2, 0, 1, 0, 5), 1);
        assert_eq!(distance(3, 0, 1, 0, 5), 4);
        assert_eq!(distance(4, 0, 1, 0, 5), 4);
        assert_eq!(distance(4, 4, 1, 0, 5), 5);
        assert_eq!(distance(3, 0, 2, 0, 5), 1);
        assert_eq!(distance(4, 0, 2, 0, 5), 4);
        assert_eq!(distance(0, 1, 2, 0, 5), 5);
        assert_eq!(distance(4, 4, 2, 0, 5), 5);
        assert_eq!(distance(4, 4, 3, 4, 5), 1);
    }

    #[test]
    fn test_evaluate_1() {
        let s: Solution = Solution {
            grid: vec![
                4, 16, 2, 23, 10,
                13, 8, 5, 12, 6,
                0, 22, 18, 3, 9,
                19, 15, 20, 17, 11,
                21, 1, 24, 14, 7,
            ],
            eval: 0,
        };
        assert_eq!(s.evaluate(), 1400);
    }

    #[test]
    fn test_evaluate_2() {
        let s: Solution = Solution {
            grid: vec![
                12, 3, 24, 10, 18,
                0, 4, 11, 13, 1,
                14, 2, 16, 23, 21,
                15, 19, 7, 9, 8,
                5, 17, 6, 20, 22,
            ],
            eval: 0,
        };
        assert_eq!(s.evaluate(), 1050);
    }

    #[test]
    fn test_index_to_name() {
        assert_eq!(index_to_name(0, 3), String::from("AA"));
        assert_eq!(index_to_name(1, 3), String::from("BA"));
        assert_eq!(index_to_name(2, 3), String::from("CA"));
        assert_eq!(index_to_name(3, 3), String::from("AB"));
        assert_eq!(index_to_name(4, 3), String::from("BB"));
        assert_eq!(index_to_name(5, 3), String::from("CB"));
        assert_eq!(index_to_name(6, 3), String::from("AC"));
        assert_eq!(index_to_name(7, 3), String::from("BC"));
        assert_eq!(index_to_name(8, 3), String::from("CC"));
        assert_eq!(index_to_name(18, 5), String::from("DD"));
        assert_eq!(index_to_name(0, 30), String::from("AA"));
        assert_eq!(index_to_name(899, 30), String::from("44"));
    }

    #[test]
    fn test_name_to_index() {
        assert_eq!(name_to_index("AA", 3), 0);
        assert_eq!(name_to_index("BA", 3), 1);
        assert_eq!(name_to_index("CA", 3), 2);
        assert_eq!(name_to_index("AB", 3), 3);
        assert_eq!(name_to_index("BB", 3), 4);
        assert_eq!(name_to_index("CB", 3), 5);
        assert_eq!(name_to_index("AC", 3), 6);
        assert_eq!(name_to_index("BC", 3), 7);
        assert_eq!(name_to_index("CC", 3), 8);
        assert_eq!(name_to_index("DD", 5), 18);
        assert_eq!(name_to_index("AA", 30), 0);
        assert_eq!(name_to_index("44", 30), 899);
    }
}
