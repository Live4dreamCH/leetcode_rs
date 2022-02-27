pub struct Solution {}

#[derive(Clone)]
struct DP {
    min_val: f64,
    max_val: f64,
    min_str: String,
    max_str: String,
}
impl DP {
    pub fn new() -> DP {
        DP {
            min_val: 1e9,
            max_val: -1.0,
            min_str: String::new(),
            max_str: String::new(),
        }
    }
}

pub fn optimal_division_dp(nums: Vec<i32>) -> String {
    let n = nums.len();
    let mut dp: Vec<Vec<DP>> = vec![vec![DP::new(); n]; n];
    for i in 0..n {
        dp[i][i].min_val = nums[i].into();
        dp[i][i].max_val = nums[i].into();
        dp[i][i].min_str = nums[i].to_string();
        dp[i][i].max_str = nums[i].to_string();
    }
    for b in 1..n {
        for i in 0..n - b {
            let j = i + b;
            for k in i..j {
                if dp[i][k].min_val / dp[k + 1][j].max_val < dp[i][j].min_val {
                    dp[i][j].min_val = dp[i][k].min_val / dp[k + 1][j].max_val;
                    if k + 1 == j {
                        dp[i][j].min_str = format!("{}/{}", dp[i][k].min_str, dp[k + 1][j].max_str)
                    } else {
                        dp[i][j].min_str =
                            format!("{}/({})", dp[i][k].min_str, dp[k + 1][j].max_str)
                    }
                }
                if dp[i][k].max_val / dp[k + 1][j].min_val > dp[i][j].max_val {
                    dp[i][j].max_val = dp[i][k].max_val / dp[k + 1][j].min_val;
                    if k + 1 == j {
                        dp[i][j].max_str = format!("{}/{}", dp[i][k].max_str, dp[k + 1][j].min_str)
                    } else {
                        dp[i][j].max_str =
                            format!("{}/({})", dp[i][k].max_str, dp[k + 1][j].min_str)
                    }
                }
            }
        }
    }
    return dp[0][n - 1].max_str.clone();
}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        // 举例: 设输入为[a,b,c,d,e]
        // 1. 除法可以转化为乘一个分数, 即a ÷ x = a * (1/x)
        // 2. 由于最左侧的除法本来优先级就最高, 在a左侧添加括号没有任何意义
        //    所以所有括号都在a的右侧
        // 3. 加括号会使括号内运算的优先级高于括号外
        //    从括号外来看, 括号隐藏了括号内部复杂的运算, 只向外提供一个数值, 也就是括号内运算的结果
        // 于是, 不管如何添加括号, 最终式子的形式总是a ÷ ? ÷ ? ÷ ... ÷ ?
        // 这些"?"既有可能是原数值, 如b; 也有可能是括号返回的数值, 如(c ÷ d)的值
        // 这个式子也可以化成a乘若干个分数的形式: a * (1/?) * (1/?) * ... * (1/?)
        // 继续, 合并分式: a * (1/(? * ? * ... *?))
        // 想要使这个式子的结果最大, 就要让?*?*...*?的连乘值最小
        // 最好的方法, 就是把a之后的所有数字用一对括号括起来
        // 形如: a ÷ (b ÷ c ÷ d ÷ e)
        // 这样做, 不仅使?的数目最少(只有一个?)
        // 也使得这个?的值最小(题目保证所有数字都大于1, b连除后面的所有数字, 越除越小)
        // 于是分母上的连乘值最小, 于是原式的值最大

        // 如果没有数字大于一的保证, 就需要使用动态规划

        // 网友的解释更简介
        // 由于数字都大于1, 乘号越多, 值越大
        // a ÷b ÷c ÷d 小于
        // a ÷ (b ÷ (c ÷ d)) = a ÷b *c ÷d 小于
        // a ÷ (b ÷ c ÷ d) = a ÷b *c *d

        // match nums.len() {
        //     1 => nums[0].to_string(),
        //     2 => format!("{}/{}", nums[0], nums[1]),
        //     n => {
        //         let mut res = nums[1].to_string();
        //         for i in 2..n {
        //             res.push_str(&format!("/{}", nums[i]))
        //         }
        //         format!("{}/({})", nums[0], res)
        //     }
        // }
        optimal_division_dp(nums)
    }
}

fn main() {
    let s = Solution::optimal_division(vec![1000, 100, 10, 2]);
    assert_eq!(s, "1000/(100/10/2)");
    dbg!(s);
}
