struct Solution {
    grid: Vec<Vec<u32>>,
}

impl Solution {
    fn new(size: u32) -> Solution {
        let mut s: Solution = Solution {
            grid: Vec::new(),
        };
        for i in 0..size.pow(2) {
            if i % size == 0 {
                s.grid.push(Vec::new());
            }
            s.grid[(i/size) as usize].push(i as u32);
        }
        return s;
    }
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

fn main() {
    println!("{}", Solution::new(5));
}
