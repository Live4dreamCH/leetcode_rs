use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        // let mut ll1 = ll1::LL1::new();
        // let parse_tree = ll1.parse(&expression);
        let mut tokenizer = Lexer::from(expression.as_bytes().iter());
        for token in tokenizer {
            dbg!(token);
        }
        Vec::new()
    }
}

/// interface between lexer and parser
#[derive(Debug)]
pub enum Token {
    OpenBrace,
    Word(String),
    Comma,
    CloseBrace,
}

/// comsume string, and provide next token and lookahead function
pub struct Lexer<'a> {
    str_iter: core::slice::Iter<'a, u8>,
}

impl<'a> Lexer<'a> {
    pub fn from(str_iter: core::slice::Iter<'a, u8>) -> Lexer {
        Lexer { str_iter }
    }

    pub fn lookahead(&mut self, n: u32) -> Vec<Token> {
        if 0 == n {
            return Vec::new();
        }
        let new_self = Lexer::from(self.str_iter.clone());
        new_self.take(n as usize).collect()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let mut word = String::new();
        match self.str_iter.next()? {
            b'{' => Some(OpenBrace),
            b',' => Some(Comma),
            b'}' => Some(CloseBrace),
            ch @ b'a'..=b'z' => {
                word.push(*ch as char);
                loop {
                    let next_ch = self.str_iter.clone().next();
                    if let Some(ch @ b'a'..=b'z') = next_ch {
                        word.push(*ch as char);
                        self.str_iter.next();
                        continue;
                    } else {
                        return Some(Word(word));
                    }
                }
            }
            error => panic!("meet char {}", error),
        }
    }
}

enum Symbol {
    Expr,
    Union,
    Concat,
    Id(String),
}
// use petgraph::graph::DiGraph;

pub struct ParseTreeNode {
    curr: Symbol,
    children: Vec<Symbol>,
}

pub struct ParseTree {
    root: ParseTreeNode,
}

impl ParseTree {
    pub fn from(root: ParseTreeNode) -> ParseTree {
        ParseTree { root }
    }
}
// pub struct AST{}

pub struct RecursiveDecent {
    visited_path: VecDeque<(Symbol, bool)>,
}
impl RecursiveDecent {
    pub fn new() -> RecursiveDecent {
        RecursiveDecent {
            visited_path: VecDeque::new(),
        }
    }

    pub fn parse(&self, lexer: &mut Lexer) -> Result<ParseTree, String> {
        let mut root = self.expr(lexer)?;
        Ok(ParseTree::from(root))
    }

    // expr ->union | union,expr
    fn expr(&self, lexer: &mut Lexer) -> Result<ParseTreeNode, String> {
        let mut node = ParseTreeNode {
            curr: Symbol::Expr,
            children: Vec::new(),
        };

        // let mut err_vec = Vec::new();
        //  self.union() ?
        Ok(node)
    }

    // union->concat | concat union
    fn union(&self, lexer: &mut Lexer) -> Result<ParseTreeNode, String> {
        Err(String::new())
    }

    // concat->word | {expr}
    fn concat(&self, lexer: &mut Lexer) -> Result<ParseTreeNode, String> {
        Err(String::new())
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
