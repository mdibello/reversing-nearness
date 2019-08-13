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

    pub fn evaluate(&self) -> u64 {
        let lower_bounds: Vec<u64> = vec![
            0, 10, 72, 816, 3800, 16902, 52528, 155840, 381672, 902550,
            1883244, 3813912, 7103408, 12958148, 22225500, 37474816,
            60291180, 95730984, 146469252, 221736200, 325763172,
            474261920, 673706892, 949783680, 1311600000, 1799572164,
            2425939956, 3252444776, 4294801980, 5643997650,
        ];
        let size = self.grid.len() as u32;
        let original = Solution::new(size);
        let mut sum = 0;
        for x1 in 0..size {
            for y1 in 0..size {
                for x2 in 0..size {
                    for y2 in 0..size {
                        sum += original.distance(x1, y1, x2, y2) *
                                self.distance(x1, y1, x2, y2);
                    }
                }
            }
        }
        return (sum as u64) - lower_bounds[(size - 1) as usize];
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
