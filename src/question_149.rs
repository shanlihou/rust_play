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
        println!("points :{:?}", points);
        let a:HashMap<LineVec, i32> = HashMap::new();
        let start = 1;
        for i in 0..points.len() {
            if i == start {
                continue
            }

            let _vec = Solution::calc_rate(&points[start], &points[i]);
            println!("_vec:{:?}", _vec);
        }
        0
    }
}