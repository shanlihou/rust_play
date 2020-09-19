/*
 * @lc app=leetcode.cn id=30 lang=rust
 *
 * [30] 串联所有单词的子串
 */

// @lc code=start
use std::marker::Copy;
pub struct Solution {

}

const ALPHA_a: u8 = 'a' as u8;


#[derive(Debug)]
pub struct Node {
    son: Vec<Option<Box<Node>>>
}

impl Node {
    pub fn new() -> Self {
        let mut _son:Vec<Option<Box<Node>>> = vec![];
        for i in 0..26 {
            _son.push(None)
        }
        Self {
            son: _son
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Option<Box<Node>>
}

impl Trie {
    pub fn new(words: &Vec<String>) -> Self {
        let mut _root = Node::new();
        for word in words {
            let mut _cur_node = &mut _root;
            for c in word.as_bytes() {
                let alpha = (c - ALPHA_a) as usize;
                //println!("alpha:{:?}, c:{:?}\n", alpha, c);
                _cur_node.son[alpha] = Some(Box::new(Node::new()));
                let mut _cur_node = &mut _cur_node.son[alpha];
            }
        }
        println!("trie:{:?}", _root);
        Self {
            root: Some(Box::new(_root))
        }
    }

    pub fn is_str_in(&self, word: String) -> bool {
        match &self.root {
            Some(_root) => {
                let mut cur_root = _root;
                for c in word.as_bytes() {
                    let alpha = (c - ALPHA_a) as usize;
                    if let Some(_cur) = cur_root {

                    }
                    else {

                    }
                }
            },
            None => {

            }
        }
        true
    }
}



impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let t = Trie::new(&words);
        vec![]
    }
}
// @lc code=end

