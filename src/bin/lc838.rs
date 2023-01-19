pub struct Solution {}

enum Last {
    Edge,
    R(usize),
    L(usize),
}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ano = Last::Edge;
        let mut res = dominoes.clone();
        for (i, c) in dominoes.chars().enumerate() {
            match c {
                'R' => {
                    if let Last::R(j) = ano {
                        for k in j + 1..i {
                            res.replace_range(k..k + 1, "R");
                        }
                    }
                    ano = Last::R(i);
                }
                'L' => {
                    match ano {
                        Last::Edge => {
                            // [0,i) = "L...L"
                            for k in 0..i {
                                res.replace_range(k..k + 1, "L");
                            }
                        }
                        Last::R(j) => {
                            // (j,i) = "R...L"
                            let mid = (j + i + 1) / 2;
                            for k in j + 1..mid {
                                res.replace_range(k..k + 1, "R");
                            }
                            let mid = (j + i) / 2;
                            for k in mid + 1..i {
                                res.replace_range(k..k + 1, "L");
                            }
                        }
                        Last::L(j) => {
                            // (j,i) = "L...L"
                            for k in j + 1..i {
                                res.replace_range(k..k + 1, "L");
                            }
                        }
                    }
                    ano = Last::L(i);
                }
                _ => (),
            }
        }
        if let Last::R(j) = ano {
            for k in j + 1..res.len() {
                res.replace_range(k..k + 1, "R");
            }
        }
        res
    }
}

fn main() {
    let s = Solution::push_dominoes(".L.R...LR..L..".to_string());
    assert_eq!(s, "LL.RR.LLRRLL..");
    println!("{}", s);
    let mut s = "R...R".to_string();
    s.replace_range(1..3, "L");
    println!("{}", s);
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
