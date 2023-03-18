pub struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut answer: Vec<i32> = Vec::with_capacity(n);
        for j in 0..n {
            answer.push(j as i32);
        }
        for i in 0..m {
            for j in 0..n {
                let &pos = &answer[j];
                if pos == -1 {
                    continue;
                } else if (pos == 0 && grid[i][pos as usize] == -1)
                    || (pos == (n - 1) as i32 && grid[i][pos as usize] == 1)
                {
                    answer[j] = -1;
                } else if (grid[i][pos as usize] == 1 && grid[i][pos as usize + 1] == -1)
                    || (grid[i][pos as usize] == -1 && grid[i][pos as usize - 1] == 1)
                {
                    answer[j] = -1;
                } else {
                    answer[j] = pos + grid[i][pos as usize];
                }
            }
        }
        answer
    }
}

fn main(){}