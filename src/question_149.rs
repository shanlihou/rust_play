use std::collections::HashMap;

pub struct Solution {

}

struct Point<'a> {
    pos: &'a Vec<i32>,

}

#[derive(Debug)]
struct LineVec {
    x: f32,
    y: f32
}

impl std::cmp::PartialEq for LineVec {
    fn eq(&self, other:&Self) -> bool {
        false
    }
}

impl std::cmp::Eq for LineVec {
}

impl Solution {
    pub fn calc_rate(a:Vec<i32>, b:Vec<i32>) -> LineVec {
        let diff_x = (b[0] - a[0]) as f32;
        let diff_y = (b[1] - a[1]) as f32;
        let diff_mod = (diff_x * diff_x + diff_y * diff_y).sqrt();
        LineVec{x:diff_x / diff_mod, y:diff_y / diff_mod}
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        println!("points :{:?}", points);
        let a:HashMap<LineVec, i32> = HashMap::new();
        let start = 1;
        for i in 0..points.len() {
            if i == start {
                continue
            }

            let _vec = Solution::calc_rate(points[start], points[i]);
            println!("_vec:{:?}", _vec);
        }
        0
    }
}