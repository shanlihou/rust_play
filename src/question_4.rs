/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
struct Solve <'a>{
    _nums1: &'a Vec<i32>,
    _nums2: &'a Vec<i32>
}


// @lc code=start
impl <'a> Solve <'a> {
    pub fn vio_get_mid(&self, mut left1: usize, mut right1: usize, mut left2: usize, mut right2: usize) -> f64 {
        let count = right1 - left1 + right2 - left2;
        if count == 1 {
            return if left1 == right1 {
                self._nums2[left2] as f64
            }
            else {
                self._nums1[left1] as f64
            }
        }

        if count == 2 {
            let mut sum = 0;
            for i in left1..right1 {
                sum += self._nums1[i]
            }

            for i in left2..right2 {
                sum += self._nums2[i]
            }

            return (sum as f64) / 2.0
        }

        if left1 == right1 {
            left2 += 1
        }
        else if left2 == right2 {
            left1 += 1
        }
        else if self._nums1[left1] < self._nums2[left2] {
            left1 += 1
        }
        else {
            left2 += 1
        }

        if left1 == right1 {
            right2 -=1
        }
        else if left2 == right2 {
            right1 -= 1
        }
        else if self._nums1[right1 - 1] > self._nums2[right2 - 1] {
            right1 -= 1
        }
        else {
            right2 -= 1
        }

        self.vio_get_mid(left1, right1, left2, right2)
    }

    pub fn get_vec_mid(&self, nums: &Vec<i32>, left: usize, right: usize) -> f64 {
        if (right - left) & 1 == 1 {
            nums[(right + left) / 2] as f64
        }
        else {
            let mid = (right + left) / 2;
            ((nums[mid] + nums[mid - 1]) as f64) / 2.0
        }
    }

    pub fn get_half(&self, left: usize, right: usize) -> usize {
        let whole = right - left;
        if whole & 1 == 1{
            whole / 2
        }
        else {
            whole / 2 - 1
        }
    }

    pub fn rec_deal(&self, left1: usize, right1: usize, left2: usize, right2: usize) -> f64{
        // println!("left1:{}, right1:{}, left2:{}, right2:{}", left1, right1, left2, right2);
        if left1 == right1 {
            return self.get_vec_mid(self._nums2, left2, right2)
        }
        else if left2 == right2 {
            return self.get_vec_mid(self._nums1, left1, right1)
        }

        let count = right1 - left1 + right2 - left2;
        if count <= 9 {
            return self.vio_get_mid(left1, right1, left2, right2)
        }

        let count1 = right1 - left1;
        if count1 <= 2 {
            let mut half2 = self.get_half(left2, right2);
            half2 -= count1;
            return self.rec_deal(left1, right1, left2 + half2, right2 - half2)
        }

        let count2 = right2 - left2;
        if count2 <= 2 {
            let mut half1 = self.get_half(left1, right1);
            half1 -= count2;
            return self.rec_deal(left1 + half1, right1 - half1, left2, right2)
        }

        let mid1 = self.get_vec_mid(self._nums1, left1, right1);
        let mid2 = self.get_vec_mid(self._nums2, left2, right2);
        let half1 = self.get_half(left1, right1);
        let half2 = self.get_half(left2, right2);
        let half = if half1 < half2 {half1} else {half2};
        if mid1 < mid2 {
            self.rec_deal(left1 + half, right1, left2, right2 - half)
        }
        else {
            self.rec_deal(left1, right1 - half, left2 + half, right2)
        }
    }
}
// @lc code=end
pub struct Solution {

}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let s = Solve {_nums1: &nums1, _nums2: &nums2};
        s.rec_deal(0, nums1.len(), 0, nums2.len())
    }
}