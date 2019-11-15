use std::collections::VecDeque;

pub struct Solution {
    a:Vec<i32>,
    k:i32,
}

impl Solution {
    fn calc(&self, sum:i32, start:usize, end:usize) -> i32{
        if start >= end {
            return -1;
        }

        let left = self.calc(sum - self.a[end - 1], start, end - 1);
        let right = self.calc(sum - self.a[start], start + 1, end);

        let mut cur:i32 = -1;
        if sum >= self.k {
            cur = (end - start) as i32;
        }

        if left == -1 {
            if right < left {
                return right;
            }
            else {
                return cur;
            }
        }
        else if right == -1 {
            if left < right {
                return left;
            }
            else {
                return cur;
            }
        }
        else if left < right{
            return left;
        }
        else {
            return right;
        }
    }

    pub fn shortest_subarray_old(a: Vec<i32>, k: i32) -> i32 {
        let sum:i32 = a.iter().sum();
        let sol = Solution{a, k};
        sol.calc(sum, 0, sol.a.len())
    }
}

impl Solution {
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        let mut obj_list: VecDeque<MinObj> = VecDeque::new();
        let mut cur_sum = 0;
        let mut cur_count = 0;
        let mut min_count = 999999;
        for i in a {
            cur_count += 1;
            cur_sum += i;
            while obj_list.len() != 0{
                if obj_list[obj_list.len() - 1].sum >= cur_sum {
                    obj_list.pop_back();
                }
                else {
                    break;
                }
            }

            while obj_list.len() != 0 {
                if cur_sum - obj_list[0].sum >= k {
                    if min_count > cur_count - obj_list[0].count {
                        min_count = cur_count - obj_list[0].count;
                    }

                    obj_list.pop_front();
                }
                else {
                    break;
                }
            }
            obj_list.push_back(MinObj{sum:cur_sum, count:cur_count});

            if cur_sum >= k && min_count > cur_count{
                min_count = cur_count;
            }
        }

        if min_count == 999999 {-1} else {min_count}
    }
}

struct MinObj{
    sum: i32,
    count: i32,
}