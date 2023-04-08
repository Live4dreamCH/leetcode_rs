struct Solution;
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        // let mut ll1 = ll1::LL1::new();
        // let parse_tree = ll1.parse(&expression);
        let mut tokenizer = Tokenizer::from(expression.as_bytes().iter());
        for token in tokenizer {
            dbg!(token);
        }
        Vec::new()
    }
}

#[derive(Debug)]
pub enum Token {
    OpenBrace,
    Word(String),
    Comma,
    CloseBrace,
}
pub struct Tokenizer<'a> {
    str_iter: core::slice::Iter<'a, u8>,
}
impl<'a> Tokenizer<'a> {
    pub fn from(str_iter: core::slice::Iter<'a, u8>) -> Tokenizer {
        Tokenizer { str_iter }
    }
}
impl Iterator for Tokenizer<'_> {
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

pub struct RecursiveDecent {}
impl RecursiveDecent {
    pub fn from() -> RecursiveDecent {
        RecursiveDecent {}
    }
    pub fn parse(&self, tokenizer: &mut Tokenizer) -> Result<ParseTree, String> {
        let mut root = self.expr()?;
        Ok(ParseTree::from(root))
    }
    // expr ->union | union,expr
    fn expr(&self) -> Result<ParseTreeNode, String> {
        let mut node = ParseTreeNode {
            curr: Symbol::Expr,
            children: Vec::new(),
        };
        let mut err_vec = Vec::new();
        match self.union() {
            Ok(subtree) => ,
            Err(_) => todo!(),
        }
        node
    }
    // union->concat | concat union
    fn union(&self) -> Result<ParseTreeNode, String> {}
    // concat->word | {expr}
    fn concat(&self) -> Result<ParseTreeNode, String> {}
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
