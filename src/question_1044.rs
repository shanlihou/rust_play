use std::collections::HashMap;

pub struct Solution {

}

struct RabinKarp {
    weight:i32,
    hash:i32,
}

static G_E:i32 = 26;
static G_MOD:i32 = 1000007;
static G_A:u8 = 'a' as u8;

impl RabinKarp {
    pub fn new(s: String) -> RabinKarp {
        let mut _weight = 1;
        let mut _hash:i32 = 0;
        let mut is_first = true;
        for c in s.bytes() {
            if is_first {
                is_first = false;
            }
            else {
                _weight = _weight * G_E % G_MOD;
            }
            _hash = (_hash * G_E + RabinKarp::get_byte_num(c)) % G_MOD;
        }
        RabinKarp{weight:_weight, hash:_hash}
    }

    pub fn shift(&mut self, head:u8, tail:u8) ->i32 {
        self.hash = ((self.hash - RabinKarp::get_byte_num(head) * self.weight) * G_E + RabinKarp::get_byte_num(tail)) % G_MOD;
        if self.hash < 0 {
            self.hash += G_MOD
        }
        self.hash
    }

    pub fn get_byte_num(c: u8) -> i32 {
        (c - G_A) as i32
    }
}

#[derive(Debug)]
struct HashObj {
    first_pos: usize,
    times: usize,
}

#[derive(Debug)]
struct DupObj<'a> {
    origin_str: &'a String,
    hash_obj_vec: Vec<HashObj>,
    match_len: usize,
}

impl<'a> DupObj<'a> {
    pub fn new(pos:usize, ori_str: &'a String, match_len:usize) -> DupObj {
        let mut dup = DupObj{origin_str:ori_str, hash_obj_vec: vec![], match_len};
        dup.hash_obj_vec.push(HashObj{first_pos: pos, times: 1});
        dup
    }

    pub fn push(&mut self, pos:usize) ->usize {
        for hash_obj in self.hash_obj_vec.iter_mut() {
            if self.origin_str[hash_obj.first_pos..(hash_obj.first_pos + self.match_len)].cmp(&self.origin_str[pos..(pos + self.match_len)]) == std::cmp::Ordering::Equal {
                hash_obj.times += 1;
                return hash_obj.times;
            }
        }

        self.hash_obj_vec.push(HashObj{first_pos:pos, times:1});
        1
    }
}

impl Solution {

    pub fn get_dup_by_len(l: usize, s: &String) -> i32 {
        if l == 0 {
            return -1;
        }

        let mut dup_dic = HashMap::new();
        let mut rk = RabinKarp::new(s[0..l].to_string());
        let mut first_pos:i32 = -1;
        let mut max_len = 0;
        dup_dic.insert(rk.hash, DupObj::new(0, s, l));
        for start in 0..s.len() {
            let end = start + l;
            if end >= s.len() {
                break;
            }

            let hash = rk.shift(s.as_bytes()[start] as u8, s.as_bytes()[end] as u8);
            if dup_dic.contains_key(&hash) {
                let times = dup_dic.get_mut(&hash).unwrap().push(start + 1);
                if times != 1 && times > max_len {
                    max_len = times;
                    first_pos = (start + 1) as i32;
                }
            }
            else {
                dup_dic.insert(hash, DupObj::new(start + 1, s, l));
            }

        }
        // println!("l:{:?}, dup:{:?}", l, dup_dic);
        first_pos
    }

    pub fn longest_dup_substring(s: String) -> String {
        let mut left = 0;
        let mut right = s.len();
        let mut final_pos = -1;
        while left < right {
            let mid = (left + right) / 2;
            let pos = Solution::get_dup_by_len(mid, &s);
            if mid == 4 {
            }
            if pos == -1 {
                right = mid
            }
            else {
                left = mid + 1;
                final_pos = pos
            }
        }
        let ret = left as i32 - 1;
        if ret <= 0 {
            "".to_string()
        }
        else {
            let final_pos = final_pos as usize;
            s[final_pos..(final_pos + ret as usize)].to_string()
        }

    }
}