use std::collections::HashMap;

pub struct Solution {

}

struct RabinKarp {
    weight:u32,
    hash:u32
}

static G_E:u32 = 26;
static G_MOD:u32 = 99999721;
static G_A:u8 = 'a' as u8;

impl RabinKarp {
    pub fn new(s: String) -> RabinKarp {
        let mut _weight = 1;
        let mut _hash:u32 = 0;
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

    pub fn shift(&mut self, head:u8, tail:u8) ->u32 {
        self.hash = (self.hash - RabinKarp::get_byte_num(head) * self.weight) * G_E + RabinKarp::get_byte_num(tail);
        self.hash
    }

    pub fn get_byte_num(c: u8) -> u32 {
        (c - G_A + 1) as u32
    }
}

struct HashObj {
    first_pos: usize,
    times: usize,
}

struct DupObj {
    hash_obj_vec: Vec<HashObj>
}

#derive
impl DupObj {
    pub fn new(pos:usize) -> DupObj {
        let mut dup = DupObj{hash_obj_vec: vec![]};
        dup.hash_obj_vec.push(HashObj{first_pos: pos, times: 1});
        dup
    }
}

impl Solution {

    pub fn get_dup_by_len(l: usize) -> String {
        let mut dup_dic = HashMap::new();
        let mut rk = RabinKarp::new(s[0..3].to_string());
        dup_dic.insert(rk.hash, DupObj::new(0));
        for start in 0..s.len() {
            let end = start + l;
            if end >= s.len() {
                break;
            }

            let hash = rk.shift(s.as_bytes()[start] as u8, s.as_bytes()[end] as u8);

            println!("start:{:?}, end:{:?}, hash:{:?}", start, end, hash)
        }
        "a"
    }

    pub fn longest_dup_substring(s: String) -> String {
        println!("s:{:?}", s);


//        for start in 0..s.len() {
//
//
//            for end in (start + 1)..(s.len() + 1){
////                println!("start:{:?}, end:{:?}", start, end);
//                let cur_str = s[start..end].to_string();
//                if end < s.len() {
//                    for i in end..s.len() {
//                        let _end = end - start + i;
//                        if _end > s.len() {
//                            break;
//                        }
//                        println!("i:{:?}, end:{:?}", i, _end);
//                        let ret = s[i.._end].cmp(cur_str.as_str());
//                        match ret {
//                            std::cmp::Ordering::Equal => {
//
//                            },
//                            _ => None
//                        }
//                        println!("ret:{:?}", ret);
//                    }
//                }
//            }
//        }
        "abc".to_string()
    }
}