
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
