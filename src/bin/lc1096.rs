struct Solution;
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut ll1 = ll1::LL1::new();
        let parse_tree = ll1.parse(&expression);
        Vec::new()
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
    use std::collections::{BTreeSet, HashMap, HashSet};
    use std::fmt::Debug;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
    enum NonTerminal {
        Expression,
        OrExpr,
        OrNext,
        ConcatExpr,
        ConcatNext,
    }

    #[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone)]
    enum Terminal {
        Comma,
        OpenBrace,
        CloseBrace,
        Word(String),
        EOF,
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    enum Symbol {
        NonTerminal(NonTerminal),
        Terminal(Terminal),
        Epsilon,
    }

    #[derive(Clone)]
    struct Symbols {
        right: Vec<Symbol>,
        action: Rc<dyn Fn()>,
    }
    impl Debug for Symbols {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.right)
        }
    }
    impl PartialEq for Symbols {
        fn eq(&self, other: &Self) -> bool {
            self.right == other.right
        }
    }
    impl Eq for Symbols {}
    impl PartialOrd for Symbols {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.right.partial_cmp(&other.right)
        }
    }
    impl Ord for Symbols {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.right.partial_cmp(&other.right).unwrap()
        }
    }
    // impl Clone for Symbols {
    //     fn clone(&self) -> Self {
    //         Self {
    //             right: self.right.clone(),
    //             action: self.action.,
    //         }
    //     }
    // }

    #[derive(Debug)]
    struct Productions {
        production_set: HashMap<NonTerminal, BTreeSet<Symbols>>,
    }
    impl Productions {
        fn add(&mut self, non_terminal: NonTerminal, symbols: Vec<Symbol>, action: Rc<dyn Fn()>) {
            self.production_set
                .entry(non_terminal)
                .or_default()
                .insert(Symbols {
                    right: symbols,
                    action,
                });
        }
    }

    #[derive(Debug)]
    struct LL1Table {
        nonterminal_terminal_production_table: HashMap<NonTerminal, HashMap<Terminal, Symbols>>,
    }
    impl LL1Table {
        fn add(
            &mut self,
            non_terminal: NonTerminal,
            terminal: Terminal,
            symbols: Vec<Symbol>,
            action: Rc<dyn Fn()>,
        ) {
            self.nonterminal_terminal_production_table
                .entry(non_terminal)
                .or_default()
                .insert(
                    terminal,
                    Symbols {
                        right: symbols,
                        action,
                    },
                );
        }
        fn find(&self, non_terminal: &NonTerminal, terminal: &Terminal) -> Option<Symbols> {
            match self
                .nonterminal_terminal_production_table
                .get(non_terminal)?
                .get(terminal)
            {
                Some(s) => Some(s.clone()),
                None => None,
            }
        }
    }

    #[derive(Debug)]
    struct ParseTreeNode {
        symbol: Symbol,
        children: Vec<ParseTreeNode>,
    }

    #[derive(Debug)]
    struct ParseTree {
        root: ParseTreeNode,
    }

    #[derive(Debug)]
    pub struct LL1 {
        pos: usize,
        productions: Productions,
        parsing_table: LL1Table,
    }
    impl LL1 {
        pub fn new() -> LL1 {
            let mut p = LL1 {
                pos: 0,
                productions: Productions {
                    production_set: HashMap::new(),
                },
                parsing_table: LL1Table {
                    nonterminal_terminal_production_table: HashMap::new(),
                },
            };

            p.productions.add(
                NonTerminal::Expression,
                vec![
                    Symbol::NonTerminal(NonTerminal::OrExpr),
                    Symbol::NonTerminal(NonTerminal::OrNext),
                ],
                Rc::from(|| ()),
            );

            p.productions.add(
                NonTerminal::OrNext,
                vec![
                    Symbol::Terminal(Terminal::Comma),
                    Symbol::NonTerminal(NonTerminal::Expression),
                ],
                Rc::from(|| println!(" + ")),
            );
            p.productions
                .add(NonTerminal::OrNext, vec![Symbol::Epsilon], Rc::from(|| ()));

            p.productions.add(
                NonTerminal::OrExpr,
                vec![
                    Symbol::NonTerminal(NonTerminal::ConcatExpr),
                    Symbol::NonTerminal(NonTerminal::ConcatNext),
                ],
                Rc::from(|| ()),
            );

            p.productions.add(
                NonTerminal::ConcatNext,
                vec![Symbol::NonTerminal(NonTerminal::OrExpr)],
                Rc::from(|| println!(" * ")),
            );
            p.productions.add(
                NonTerminal::ConcatNext,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );

            p.productions.add(
                NonTerminal::ConcatExpr,
                vec![Symbol::Terminal(Terminal::Word(String::new()))],
                Rc::from(|| println!(" word ")),
            );
            p.productions.add(
                NonTerminal::ConcatExpr,
                vec![
                    Symbol::Terminal(Terminal::OpenBrace),
                    Symbol::NonTerminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::CloseBrace),
                ],
                Rc::from(|| println!(" into_expr ")),
            );

            p.parsing_table.add(
                NonTerminal::Expression,
                Terminal::Word(String::new()),
                vec![
                    Symbol::NonTerminal(NonTerminal::OrExpr),
                    Symbol::NonTerminal(NonTerminal::OrNext),
                ],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::Expression,
                Terminal::OpenBrace,
                vec![
                    Symbol::NonTerminal(NonTerminal::OrExpr),
                    Symbol::NonTerminal(NonTerminal::OrNext),
                ],
                Rc::from(|| ()),
            );

            p.parsing_table.add(
                NonTerminal::OrNext,
                Terminal::EOF,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::OrNext,
                Terminal::Comma,
                vec![
                    Symbol::Terminal(Terminal::Comma),
                    Symbol::NonTerminal(NonTerminal::Expression),
                ],
                Rc::from(|| println!(" + ")),
            );
            p.parsing_table.add(
                NonTerminal::OrNext,
                Terminal::CloseBrace,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );

            p.parsing_table.add(
                NonTerminal::OrExpr,
                Terminal::Word(String::new()),
                vec![
                    Symbol::NonTerminal(NonTerminal::ConcatExpr),
                    Symbol::NonTerminal(NonTerminal::ConcatNext),
                ],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::OrExpr,
                Terminal::OpenBrace,
                vec![
                    Symbol::NonTerminal(NonTerminal::ConcatExpr),
                    Symbol::NonTerminal(NonTerminal::ConcatNext),
                ],
                Rc::from(|| ()),
            );

            p.parsing_table.add(
                NonTerminal::ConcatNext,
                Terminal::EOF,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::ConcatNext,
                Terminal::Comma,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::ConcatNext,
                Terminal::OpenBrace,
                vec![Symbol::NonTerminal(NonTerminal::OrExpr)],
                Rc::from(|| println!(" * ")),
            );
            p.parsing_table.add(
                NonTerminal::ConcatNext,
                Terminal::CloseBrace,
                vec![Symbol::Epsilon],
                Rc::from(|| ()),
            );
            p.parsing_table.add(
                NonTerminal::ConcatNext,
                Terminal::Word(String::new()),
                vec![Symbol::NonTerminal(NonTerminal::OrExpr)],
                Rc::from(|| println!(" * ")),
            );

            p.parsing_table.add(
                NonTerminal::ConcatExpr,
                Terminal::Word(String::new()),
                vec![Symbol::Terminal(Terminal::Word(String::new()))],
                Rc::from(|| println!(" word ")),
            );
            p.parsing_table.add(
                NonTerminal::ConcatExpr,
                Terminal::OpenBrace,
                vec![
                    Symbol::Terminal(Terminal::OpenBrace),
                    Symbol::NonTerminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::CloseBrace),
                ],
                Rc::from(|| println!(" into_expr ")),
            );
        }

        pub fn parse(&mut self, str_ref: &str) -> ParseTree {
            let mut res = ParseTree {
                root: ParseTreeNode {
                    symbol: Symbol::NonTerminal(NonTerminal::Expression),
                    children: Vec::new(),
                },
            };
            self.recursive_decent(str_ref, &mut res.root);
            dbg!(res)
        }

        fn recursive_decent(&mut self, str_ref: &str, curr_node: &mut ParseTreeNode) {
            let ch_to_term = |ch: Option<&u8>| -> Terminal {
                match ch {
                    Some(ch) => match *ch {
                        b'{' => Terminal::OpenBrace,
                        b'}' => Terminal::CloseBrace,
                        b',' => Terminal::Comma,
                        _ => Terminal::Word(String::new()),
                    },
                    None => Terminal::EOF,
                }
            };
            match &mut curr_node.symbol {
                Symbol::NonTerminal(nonterminal) => {
                    let symbols = self
                        .parsing_table
                        .find(nonterminal, &ch_to_term(str_ref.as_bytes().get(self.pos)));
                    match symbols {
                        Some(symbols) => {
                            (symbols.action)();
                            for symbol in &symbols.right {
                                match symbol {
                                    Symbol::NonTerminal(_) => {
                                        println!("{:?} -> {:?}", curr_node.symbol, symbol);
                                        curr_node.children.push(ParseTreeNode {
                                            symbol: symbol.clone(),
                                            children: vec![],
                                        });
                                        self.recursive_decent(
                                            str_ref,
                                            curr_node.children.last_mut().unwrap(),
                                        )
                                    }
                                    Symbol::Terminal(ter) => {
                                        // todo
                                        let ch = str_ref.as_bytes().get(self.pos);
                                        let t = ch_to_term(ch);
                                        if t != *ter {
                                            panic!("unmatched char:{:?} and terminal:{:?}!", ch, t)
                                        }
                                        println!("{:?} -> {:?}", curr_node.symbol, symbol);
                                        curr_node.children.push(ParseTreeNode {
                                            symbol: symbol.clone(),
                                            children: vec![],
                                        });
                                        self.recursive_decent(
                                            str_ref,
                                            curr_node.children.last_mut().unwrap(),
                                        )
                                    }
                                    s => println!("find {:?} from table, ignore!", s),
                                }
                            }
                        }
                        None => {
                            panic!("entry not found in parsing table!")
                        }
                    }
                }
                Symbol::Terminal(ter) => {
                    let ch = str_ref.as_bytes().get(self.pos);
                    let t = ch_to_term(ch);
                    if t != *ter {
                        panic!("unmatched char:{:?} and terminal:{:?}!", ch, t)
                    }
                    match ter {
                        Terminal::Word(w) => {
                            w.push(*ch.unwrap() as char);
                            self.pos += 1;
                        }
                        Terminal::EOF => {
                            println!("EOF")
                        }
                        _ => self.pos += 1,
                    }
                }
                Symbol::Epsilon => panic!("Symbol::Epsilon in ParseTree"),
            };
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
    Solution::brace_expansion_ii(String::from("{{a,z},a{b,,c},{ab,z}}"));
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
