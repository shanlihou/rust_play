pub struct Solution {

}

//enum LastOpr {
//    Nothing,
//    Modify,
//    Delete,
//    Add,
//}
#[derive(Default, Debug)]
pub struct RetObj {
    whole_step:i32,
    add_step:i32,
    mod_step:i32,
    del_step:i32,
    s_len:i32,
}

impl RetObj {
    pub fn new(add_step:i32, mod_step:i32, del_step:i32, s_len:i32)->Self {
        RetObj{whole_step:add_step + mod_step + del_step, add_step, mod_step, del_step, s_len}
    }
}

impl Solution {

    pub fn without_del(num_ori:&mut Vec<i32>, mut s_len:i32, mut add_step:i32, mut mod_step:i32, del_step:i32) -> RetObj{
//        let mut s_len = s_len;
//        let mut mod_step = mod_step;
//        let mut add_step = add_step;
//        let mut del_step = del_step;
        let mut num_vec = num_ori.clone();
        while num_vec.len() > 0{
            for num in num_vec.iter_mut() {
                if *num > 2 {
                    if s_len < 6 {
                        s_len += 1;
                        *num -= 2;
                        add_step += 1;
                    }
                    else {
                        mod_step += 1;
                        *num -= 3;
                    }
                }
            }

            num_vec.retain(|x| *x > 2);
        }
        RetObj::new(add_step, mod_step, del_step, s_len)
    }

    pub fn rec(num_vec:&mut Vec<i32>, s_len:i32, add_step:i32, mod_step:i32, del_step:i32) -> RetObj{
        if s_len > 20 {
            let mut min_step = 99999;
            let mut min_obj= RetObj::default();
            for index in 0..num_vec.len() {
                if num_vec[index] > 2 {
                    {
                        if let Some(num) = num_vec.get_mut(index) {
                            *num -= 1;
                        }
                    }
                    let tmp_obj:RetObj = Solution::rec(num_vec, s_len - 1, add_step, mod_step, del_step + 1);
                    if min_step > tmp_obj.whole_step {
                        min_step = tmp_obj.whole_step;
                        min_obj = tmp_obj;
                    }
                    {
                        if let Some(num) = num_vec.get_mut(index) {
                            *num += 1;
                        }
                    }
                }
            }
            if min_step != 99999 {
                min_obj
            }
            else {
                Solution::without_del(num_vec, s_len, add_step, mod_step, del_step)
            }
        }
        else {
            Solution::without_del(num_vec, s_len, add_step, mod_step, del_step)
        }
    }

    pub fn strong_password_checker(s: String) -> i32 {
        let mut mask:u32 = 0;
        let mut last_alpha = char::from(0);
        let mut same_count = 0;
        let s_len = s.len() as i32;
        let add_step = 0;
        let modify_step = 0;
        let delete_step = 0;
        let mut num_vec:Vec<i32> = Vec::new();
        for i in s.chars() {
            if i.is_ascii_lowercase() {
                mask |= 1
            }
            else if i.is_ascii_uppercase() {
                mask |= 2
            }
            else if i.is_ascii_digit() {
                mask |= 4
            }

            if last_alpha == char::from(0) {
                same_count = 1
            }
            else {
                if last_alpha == i {
                    same_count += 1;
                }
                else {
                    if same_count > 2 {
                        num_vec.push(same_count);
                    }
                    same_count = 1;
                }
            }

            last_alpha = i;
        }

        if same_count > 2 {
            num_vec.push(same_count);
        }
        num_vec.sort();

        println!("num vec:{:?}", num_vec);
        let ret = Solution::rec(&mut num_vec, s_len, add_step, modify_step, delete_step);
        println!("num ret:{:?}", ret);


        let mut mask_num = i32::from(0);
        for i in 0..3 {
            if (mask & (1 << i)) != 0 {
                mask_num += 1;
            }
        }

        mask_num = 3 - mask_num;
        mask_num -= ret.add_step;
        mask_num -= ret.mod_step;

        if ret.s_len < 6 {
            mask_num -= (6 - ret.s_len) as i32;
        }

        mask_num = if mask_num < 0 {0} else {mask_num};

        let mut other_step = i32::from(0);
        if ret.s_len < 6 {
            other_step = (6 - ret.s_len) as i32
        }
        else if ret.s_len > 20 {
            other_step = (ret.s_len - 20) as i32
        }
        println!("s_len is {:?}, other:{:?}, add_step:{:?}, modify_step:{:?}, delete_step:{:?}, mask_num:{:?}", s_len, other_step, add_step, modify_step, delete_step, mask_num);

        ret.add_step + ret.mod_step + ret.del_step + other_step + mask_num
    }
//
//    pub fn strong_password_checker_old(s: String) -> i32 {
//        let mut mask:u32 = 0;
//        let mut last_alpha = char::from(0);
//        let mut same_count = 0;
//        let mut s_len = s.len();
//        let mut add_step = 0;
//        let mut modify_step = 0;
//        let mut delete_step = 0;
//        let mut just_modify = char::from(0);
//        let mut last_opr = LastOpr::Nothing;
//        for i in s.chars() {
//            if i.is_ascii_lowercase() {
//                mask |= 1
//            }
//            else if i.is_ascii_uppercase() {
//                mask |= 2
//            }
//            else if i.is_ascii_digit() {
//                mask |= 4
//            }
//
//            if just_modify != char::from(0) && i != just_modify && delete_step > 0 {
//                delete_step -= 1;
//            }
//
//            just_modify = char::from(0);
//            if last_alpha == char::from(0) {
//                last_alpha = i;
//                last_opr = LastOpr::Nothing;
//            }
//            else {
//                same_count = if i == last_alpha {same_count + 1} else {0};
//                last_alpha = i;
//
//                if same_count > 1 {
//                    if s_len > 20 {
//                        s_len -= 1;
//                        delete_step += 1;
//                        same_count -= 1;
//                        last_opr = LastOpr::Delete;
//                    }
//                    else if s_len < 6 {
//                        s_len += 1;
//                        add_step += 1;
//                        last_alpha = char::from(0);
//                        same_count = 0;
//                        last_opr = LastOpr::Add;
//                    }
//                    else {
//                        modify_step += 1;
//                        last_alpha = char::from(0);
//                        same_count = 0;
//                        just_modify = match last_opr {
//                            LastOpr::Delete => just_modify,
//                            _ => i
//                        };
//                        last_opr = LastOpr::Modify;
//                    }
//                }
//                else {
//                    last_opr = LastOpr::Nothing;
//                }
//            }
//        }
//
//        let mut mask_num = i32::from(0);
//        for i in 0..3 {
//            if (mask & (1 << i)) != 0 {
//                mask_num += 1;
//            }
//        }
//
//        mask_num = 3 - mask_num;
//        mask_num -= add_step;
//        mask_num -= modify_step;
//
//        if s_len < 6 {
//            mask_num -= (6 - s_len) as i32;
//        }
//
//        mask_num = if mask_num < 0 {0} else {mask_num};
//
//        let mut other_step = i32::from(0);
//        if s_len < 6 {
//            other_step = (6 - s_len) as i32
//        }
//        else if s_len > 20 {
//            other_step = (s_len - 20) as i32
//        }
//        println!("s_len is {:?}, other:{:?}, add_step:{:?}, modify_step:{:?}, delete_step:{:?}, mask_num:{:?}", s_len, other_step, add_step, modify_step, delete_step, mask_num);
//
//        add_step + modify_step + delete_step + other_step + mask_num
//    }
}