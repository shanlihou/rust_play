/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */


pub struct Solution {

}

// @lc code=start
pub enum RegMatchRet {
    SAME,
    WRONG,
    COULD,
}
const STAR:u8 = '*' as u8;
const DOT:u8 = '.' as u8;


#[derive(Clone, Debug)]
pub struct RegPattern <'a>{
    head: usize,
    tail: usize,
    _slice: &'a [u8],
}
// pub struct RegPattern <'a>{
//     cur: Option<char>,
//     next: Option<char>,
//     pattern: std::str::Chars<'a>,
//     pos: i32,
// }

impl <'a> RegPattern <'a> {
    pub fn new (pattern: &'a String) -> Self{
        Self {
            head: 0,
            tail: pattern.len(),
            _slice: (&pattern).as_bytes(),
        }
    }

    pub fn shift_fore(&mut self) -> &mut Self{
        if self.head == self.tail {
            self
        }
        else {
            self.head += 1;
            if self.head == self.tail {
                self
            }
            else if self._slice[self.head] == STAR {
                self.head += 1;
                self
            }
            else {
                self
            }
        }
    }

    pub fn left(&self) -> u8{
        self._slice[self.head]
    }

    pub fn pure_right(&self) -> u8 {
        self._slice[self.tail - 1]
    }

    pub fn right(&self) -> u8 {
        if self._slice[self.tail - 1] == STAR {
            self._slice[self.tail - 2]
        }
        else {
            self._slice[self.tail - 1]
        }
    }

    pub fn is_left_star(&self) -> bool {
        self._slice[self.head + 1] == STAR
    }

    pub fn is_right_star(&self) -> bool {
        self._slice[self.tail - 1] == STAR
    }

    pub fn shift_back(&mut self) -> &mut Self {
        if self.head != self.tail {
            if self._slice[self.tail - 1] == STAR {
                self.head -= 2
            }
            else {
                self.head -= 1
            }
        }
        self
    }

    pub fn is_end(&self)-> bool {
        self.head == self.tail
    }
}

impl <'a, 'b> Solution {
    pub fn rec_match(s: &[u8], p: &[u8]) -> bool {
        let mut dp:Vec<Vec<bool>> = vec![vec![true]];
        let s_len = s.len();
        let p_len = p.len();

        for j in 0..p_len {
            let dp_0j = dp[0][j];
            if p[j] == STAR {
                dp[0].push(dp_0j);
                continue
            }

            if j + 1 < p_len && p[j + 1] == STAR {
                dp[0].push(dp_0j);
            }
            else {
                dp[0].push(false);
            }
        }

        for i in 0..s_len {
            dp.push(vec![false]);
            for j in 0..p_len {
                let dp_i1j = dp[i + 1][j];
                if p[j] == STAR {
                    dp[i + 1].push(dp_i1j);
                    continue
                }

                // if i == 3 {
                //     //println!("i:{}, j:{}, dp_i1j:{}, val:{}", i, j, dp_i1j, dp[i][j]);
                // }
                if j + 1 < p_len && p[j + 1] == STAR {


                    let mut val = dp[i][j] && (s[i] == p[j] || p[j] == DOT);
                    val = val || (dp[i][j + 1] && (s[i] == p[j] || p[j] == DOT));
                    val = val || dp_i1j;
                    //println!("i:{}, j:{}, dp_ij:{}, val:{}", i, j, dp_i1j, val);
                    dp[i + 1].push(val);
                }
                else {
                    let val = dp[i][j] && (s[i] == p[j] || p[j] == DOT);
                    dp[i + 1].push(val);
                }
            }
        }
        //println!("dp:{:?}\n", dp);
        dp[s_len][p_len]
    }

    pub fn is_match(s: String, p: String) -> bool {
        //let mut _s = RegPattern::new(&s);
        Solution::rec_match(s.as_bytes(), p.as_bytes())
    }
}
// @lc code=end

