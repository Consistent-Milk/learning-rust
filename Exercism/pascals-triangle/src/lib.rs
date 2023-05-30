pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
 
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => return vec![],
            _ => {
                let mut v: Vec<Vec<u32>> = vec![];
                let n: u32 = self.row_count;
               
                let f = |x: u32| {
                    let mut p: u32 = 1;
                    for i in 1..=x {
                        p *= i;
                    }
                    p
                };
               
                let combination = |n: u32, r: u32| {
                    f(n) / (f(r) * f(n - r))
                };

                for i in 0..n {
                    let mut inner_vec: Vec<u32> = Vec::new();
                    
                    for j in 0..=i {
                        inner_vec.push(combination(i, j));
                    }
                    
                    v.push(inner_vec);
                }
                return v;
            }
        }
    }
}
