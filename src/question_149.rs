use std::collections::HashMap;

pub struct Solution {

}

struct Point<'a> {
    pos: &'a Vec<i32>,

}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct LineVec {
    x: i32,
    y: i32
}

macro_rules! abs {
    ($x:expr) => {
        if $x > 0 {
            $x
        }
        else {
            -$x
        }
    }
}

impl Solution {
    pub fn gcd(a:i32, b:i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = 0;
        while b != 0 {
            c = a % b;
            a = b;
            b = c;
        }
        a
    }

    pub fn calc_rate(a:&Vec<i32>, b:&Vec<i32>) -> LineVec {
        let diff_x = b[0] - a[0];
        let diff_y = b[1] - a[1];
        if diff_x == 0 {
            return LineVec{x:0, y:1}
        }

        let _gcd = Solution::gcd(abs!(diff_x), abs!(diff_y));
        if (diff_y as i64) * (diff_x as i64) < 0 {
            LineVec{x: abs!(diff_x) / _gcd, y: -abs!(diff_y) / _gcd}
        }
        else {
            LineVec{x: abs!(diff_x) / _gcd, y: abs!(diff_y) / _gcd}
        }

    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let start = 1;
        let mut max_num = 0;
        for i in 0..points.len() {
            let mut rate_map:HashMap<LineVec, i32> = HashMap::new();
            let mut max_inner = 1;
            let mut same_pos = 0;
            for j in (i + 1)..points.len() {
                if points[i][0] == points[j][0] && points[i][1] == points[j][1] {
                    same_pos += 1;
                    continue
                }

                let rate = Solution::calc_rate(&points[i], &points[j]);
                if rate_map.contains_key(&rate) {
                    if let x = rate_map.get_mut(&rate).unwrap() {
                        *x += 1;
                        if *x > max_inner {
                            max_inner = *x;
                        }
                    }
                }
                else {
                    rate_map.insert(rate, 1);
                }
            }

            max_inner  += same_pos;
            if max_num < max_inner {
                max_num = max_inner;
            }
        }
        max_num += 1;
        if max_num <= points.len() as i32 {max_num} else {points.len() as i32}
    }
}