struct Solution;
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        nfa::NFA::from(&expression).exec_all().into_iter().collect()
    }

    // fn parse(expression: &str, nfa: &mut HashMap<u32, HashMap<String, u32>>) {
    //     nfa.insert(0, HashMap::new());
    //     let mut pos = 0;
    //     let mut new_state = 2;
    //     // let
    //     if expression.is_empty() {
    //         return;
    //     }
    //     Solution::bracket(0, expression, &mut new_state)
    // }

    // fn bracket(curr_state: i32, expression: &str, new_state: &mut i32) {
    //     match *expression.as_bytes().first().unwrap() {
    //         b'{' => 1,
    //         b',' => 1,
    //         b'}' => 1,
    //         _ => 1,
    //     };
    // }

    // fn exec(nfa: &HashMap<u32, HashMap<String, u32>>, res_set: &mut HashSet<String>) {}
}

pub mod nfa {
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct NFA {
        pos: isize,
        new_state: u32,
        curr_state: u32,
        nfa: HashMap<u32, HashMap<String, HashSet<u32>>>,
    }

    impl NFA {
        pub fn from(str_ref: &str) -> NFA {
            let mut p = NFA {
                pos: -1,
                curr_state: 0,
                new_state: 2,
                nfa: HashMap::new(),
            };
            p.parse(str_ref, 0, 1);
            dbg!(p)
        }

        fn parse(&mut self, str_ref: &str, from_state: u32, to_state: u32) {
            let mut arm_str = String::new();
            loop {
                self.pos += 1;
                match str_ref.as_bytes().get(self.pos as usize) {
                    None | Some(b'}') => {
                        if !arm_str.is_empty() {
                            self.nfa
                                .entry(from_state)
                                .or_default()
                                .entry(arm_str)
                                .or_default()
                                .insert(to_state);
                        }
                        return;
                    }
                    Some(b'{') => {
                        self.nfa
                            // .entry(self.curr_state)
                            .entry(from_state)
                            .or_default()
                            .entry(String::new())
                            .or_default()
                            .insert(self.new_state);
                        self.nfa
                            .entry(self.new_state + 1)
                            .or_default()
                            .entry(String::new())
                            .or_default()
                            .insert(to_state);
                        self.new_state += 2;
                        // dbg!(&self);
                        self.parse(str_ref, self.new_state - 2, self.new_state - 1);
                    }
                    Some(b',') => {
                        if !arm_str.is_empty() {
                            self.nfa
                                .entry(from_state)
                                .or_default()
                                .entry(arm_str.clone())
                                .or_default()
                                .insert(to_state);
                            arm_str.clear();
                        }
                    }
                    Some(ch) => arm_str.push(*ch as char),
                };
            }
        }

        pub fn exec_all(&self) -> HashSet<String> {
            HashSet::new()
        }
    }
}

pub mod ll1 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    enum NonTerminal {
        Expression,
        OrExpr,
        OrNext,
        ConcatExpr,
        ConcatNext,
    }

    enum Terminal {
        Comma,
        OpenBrace,
        CloseBrace,
        Word,
        EOF,
    }

    enum Symbol {
        NonTerminal,
        Terminal,
        Epsilon,
    }

    struct Production {
        left: NonTerminal,
        right: Vec<Symbol>,
    }

    struct Action {
        production: Production,
        action: Box<dyn Fn()>,
    }

    struct LL1Table {
        nonterminal_terminal_production_table: HashMap<NonTerminal, HashMap<Terminal, Action>>,
    }
    struct ParseTreeNode {
        symbol: Symbol,
        children: Vec<ParseTreeNode>,
    }
    struct ParseTree {
        root: ParseTreeNode,
    }

    #[derive(Debug)]
    pub struct LL1 {
        pos: isize,
        new_state: u32,
        curr_state: u32,
        nfa: HashMap<u32, HashMap<String, HashSet<u32>>>,
    }
    impl LL1 {
        pub fn from(str_ref: &str) -> LL1 {
            let mut p = LL1 {
                pos: -1,
                curr_state: 0,
                new_state: 2,
                nfa: HashMap::new(),
            };
            p.parse(str_ref, 0, 1);
            dbg!(p)
        }

        fn parse(&mut self, str_ref: &str, from_state: u32, to_state: u32) {
            let mut arm_str = String::new();
            loop {
                self.pos += 1;
                match str_ref.as_bytes().get(self.pos as usize) {
                    None | Some(b'}') => {
                        if !arm_str.is_empty() {
                            self.nfa
                                .entry(from_state)
                                .or_default()
                                .entry(arm_str)
                                .or_default()
                                .insert(to_state);
                        }
                        return;
                    }
                    Some(b'{') => {
                        self.nfa
                            // .entry(self.curr_state)
                            .entry(from_state)
                            .or_default()
                            .entry(String::new())
                            .or_default()
                            .insert(self.new_state);
                        self.nfa
                            .entry(self.new_state + 1)
                            .or_default()
                            .entry(String::new())
                            .or_default()
                            .insert(to_state);
                        self.new_state += 2;
                        // dbg!(&self);
                        self.parse(str_ref, self.new_state - 2, self.new_state - 1);
                    }
                    Some(b',') => {
                        if !arm_str.is_empty() {
                            self.nfa
                                .entry(from_state)
                                .or_default()
                                .entry(arm_str.clone())
                                .or_default()
                                .insert(to_state);
                            arm_str.clear();
                        }
                    }
                    Some(ch) => arm_str.push(*ch as char),
                };
            }
        }
    }
}

fn main() {
    Solution::brace_expansion_ii(String::from("a"));
    Solution::brace_expansion_ii(String::from("{a,b,c}"));
    Solution::brace_expansion_ii(String::from("{{a,b},{b,c}}"));
    Solution::brace_expansion_ii(String::from("{a,b}{c,d}"));
    Solution::brace_expansion_ii(String::from("a{b,c,d}"));
    Solution::brace_expansion_ii(String::from("a{b,c}{d,e}f{g,h}"));
    Solution::brace_expansion_ii(String::from("{a,b}{c,{d,e}}"));
    Solution::brace_expansion_ii(String::from("{{a,z},a{b,c},{ab,z}}"));
    // assert_eq!(vec!["a"], Solution::brace_expansion_ii(String::from("a")));
    // assert_eq!(
    //     vec!["a", "b", "c"],
    //     Solution::brace_expansion_ii(String::from("{a,b,c}"))
    // );
    // assert_eq!(
    //     vec!["a", "b", "c"],
    //     Solution::brace_expansion_ii(String::from("{{a,b},{b,c}}"))
    // );
    // assert_eq!(
    //     vec!["ac", "ad", "bc", "bd"],
    //     Solution::brace_expansion_ii(String::from("{a,b}{c,d}"))
    // );
    // assert_eq!(
    //     vec!["ab", "ac", "ad"],
    //     Solution::brace_expansion_ii(String::from("a{b,c,d}"))
    // );
    // assert_eq!(
    //     vec!["abdfg", "abdfh", "abefg", "abefh", "acdfg", "acdfh", "acefg", "acefh"],
    //     Solution::brace_expansion_ii(String::from("a{b,c}{d,e}f{g,h}"))
    // );
    // assert_eq!(
    //     vec!["ac", "ad", "ae", "bc", "bd", "be"],
    //     Solution::brace_expansion_ii(String::from("{a,b}{c,{d,e}}"))
    // );
    // assert_eq!(
    //     vec!["a", "ab", "ac", "z"],
    //     Solution::brace_expansion_ii(String::from("{{a,z},a{b,c},{ab,z}}"))
    // );
}
