use rand::distributions::{Distribution, Uniform};

pub struct Solution {
    grid: Vec<Vec<u32>>,
    eval: u64,
}

impl Solution {
    pub fn new(size: u32) -> Solution {
        let mut s: Solution = Solution {
            grid: Vec::new(),
            eval: 0,
        };
        for i in 0..size.pow(2) {
            if i % size == 0 {
                s.grid.push(Vec::new());
            }
            s.grid[(i/size) as usize].push(i as u32);
        }
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
        let size = self.grid.len() as u32;
        let mut sum: u64 = 0;
        // TODO: fix so that we're not doing twice the necessary work
        for x1 in 0..size {
            for y1 in 0..size {
                for x2 in 0..size {
                    for y2 in 0..size {
                        let orig_idx_1 = self.grid[x1 as usize][y1 as usize];
                        let x_a = orig_idx_1 % size;
                        let y_a = orig_idx_1 / size;
                        let orig_idx_2 = self.grid[x2 as usize][y2 as usize];
                        let x_b = orig_idx_2 % size;
                        let y_b = orig_idx_2 / size;
                        sum += distance(x1 as i32, y1 as i32, x2 as i32, y2 as i32, size as i32) * 
                                distance(x_a as i32, y_a as i32, x_b as i32, y_b as i32, size as i32);
                    }
                }
            }
        }
        self.eval = (sum / 2) - lower_bounds[(size - 1) as usize];
        return self.eval;
    }

    pub fn mutate(&mut self, n: u32) {
        let step = Uniform::new(0, self.grid.len());
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x1 = step.sample(&mut rng);
            let y1 = step.sample(&mut rng);
            let x2 = step.sample(&mut rng);
            let y2 = step.sample(&mut rng);
            let temp = self.grid[x1][y1];
            self.grid[x1][y1] = self.grid[x2][y2];
            self.grid[x2][y2] = temp;
        }
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

    //println!("dx: {} / dy: {}", dx, dy);
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

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut _row_count = 0;
        for row in &self.grid {
            let mut _cell_count = 0;
            _row_count += 1;
            write!(f, "(");
            for cell in row {
                _cell_count += 1;
                write!(f, "{}", index_to_name(*cell, row.len() as u32));
                if _cell_count < row.len() {
                    write!(f, ", ");
                }
            }
            write!(f, ")");
            if _row_count < self.grid.len() {
                write!(f, ",\n");
            }
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
                vec![4, 16, 2, 23, 10],
                vec![13, 8, 5, 12, 6],
                vec![0, 22, 18, 3, 9],
                vec![19, 15, 20, 17, 11],
                vec![21, 1, 24, 14, 7],
            ],
            eval: 0,
        };
        assert_eq!(s.evaluate(), 1400);
    }

    #[test]
    fn test_evaluate_2() {
        let s: Solution = Solution {
            grid: vec![
                vec![12, 3, 24, 10, 18],
                vec![0, 4, 11, 13, 1],
                vec![14, 2, 16, 23, 21],
                vec![15, 19, 7, 9, 8],
                vec![5, 17, 6, 20, 22],
            ],
            eval: 0,
        };
        assert_eq!(s.evaluate(), 1050);
    }
}
