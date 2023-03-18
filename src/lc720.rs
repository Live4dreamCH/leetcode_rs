struct TrieNode<'a> {
    c: Option<char>,
    next: Vec<&'a mut TrieNode<'a>>,
}

impl<'a> TrieNode<'a> {
    fn new(c: Option<char>) -> TrieNode<'a> {
        TrieNode {
            c,
            next: Vec::new(),
        }
    }

    // fn new_with(c: Option<char>, prev: &mut TrieNode) -> TrieNode {}
}

struct Trie<'a> {
    root: TrieNode<'a>,
}

impl<'a> Trie<'a> {
    fn new() -> Trie<'a> {
        Trie {
            root: TrieNode::new(None),
        }
    }

    fn insert(&self, word: String) {
        let curr = self.root;
        for c in word.chars() {}
    }

    fn search(&self, word: String) -> bool {
        true
    }
    fn starts_with(&self, prefix: String) -> bool {
        true
    }
}

pub struct Solution {}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        "".to_string()
    }
}

fn main() {
    let s = Solution::longest_word(vec!["".to_string()]);
    assert_eq!(s, 59);
    dbg!(s);
}
