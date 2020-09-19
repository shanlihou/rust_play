extern crate time;
pub mod exam;
pub mod stack;
pub mod question_420;
pub mod question_1044;
pub mod question_149;
pub mod question_878;
pub mod question_4;
pub mod question_10;
pub mod question_30;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::env;


fn old() {
    let a = read_data();
    //let a = vec![1, 2, 3];
//    let a = vec![1, 2, 3, 4];
    let start = time::now();
//    let ret = exam::Solution::shortest_subarray(a, 5);
    println!("start:{:?}", start);
    let ret = exam::Solution::shortest_subarray(a, 5837033);
    let end = time::now();
    println!("end:{:?}", end);
    println!("Hello, world! ret is {}, use:{}", ret, end - start);

    let str_s = "1010101010aaaB10101010".to_string();
    println!("s len:{:?}", str_s.len());
    let ret = question_420::Solution::strong_password_checker(str_s);
    println!("result:{:?}", ret);
    let str_s = "moplvidmaagmsiyyrkchbyhivlqwqsjcgtumqscmxrxrvwsnjjvygrelcbjgbpounhuyealllginkitfaiviraqcycjmskrozcdqylbuejrgfnquercvghppljmojfvylcxakyjxnampmakyjbqgwbyokaybcuklkaqzawageypfqhhasetugatdaxpvtevrigynxbqodiyioapgxqkndujeranxgebnpgsukybyowbxhgpkwjfdywfkpufcxzzqiuglkakibbkobonunnzwbjktykebfcbobxdflnyzngheatpcvnhdwkkhnlwnjdnrmjaevqopvinnzgacjkbhvsdsvuuwwhwesgtdzuctshytyfugdqswvxisyxcxoihfgzxnidnfadphwumtgdfmhjkaryjxvfquucltmuoosamjwqqzeleaiplwcbbxjxxvgsnonoivbnmiwbnijkzgoenohqncjqnckxbhpvreasdyvffrolobxzrmrbvwkpdbfvbwwyibydhndmpvqyfmqjwosclwxhgxmwjiksjvsnwupraojuatksjfqkvvfroqxsraskbdbgtppjrnzpfzabmcczlwynwomebvrihxugvjmtrkzdwuafozjcfqacenabmmxzcueyqwvbtslhjeiopgbrbvfbnpmvlnyexopoahgmwplwxnxqzhucdieyvbgtkfmdeocamzenecqlbhqmdfrvpsqyxvkkyfrbyolzvcpcbkdprttijkzcrgciidavsmrczbollxbkytqjwbiupvsorvkorfriajdtsowenhpmdtvamkoqacwwlkqfdzorjtepwlemunyrghwlvjgaxbzawmikfhtaniwviqiaeinbsqidetfsdbgsydkxgwoqyztaqmyeefaihmgrbxzyheoegawthcsyyrpyvnhysynoaikwtvmwathsomddhltxpeuxettpbeftmmyrqclnzwljlpxazrzzdosem".to_string();
    //let str_s = "akyjxakyjb".to_string();
    let ret = question_1044::Solution::longest_dup_substring(str_s);
    println!("result:{:?}", ret);
    //--------------------------149------------------------
    let mut points = vec![];
    let str_s = "[[1,1],[1,1],[2,2],[2,2]]";
    let mut ret = str_s[2..(str_s.len() - 2)].split("],[");
    for i in ret{
        let is:Vec<&str> = i.split(",").collect();
        let is:Vec<i32> = is.iter().map(|x| x.parse().unwrap()).collect();
        points.push(is);
    }
    let ret = question_149::Solution::max_points(points);
    println!("ret:{:?}", ret);
    let n = 887859796;
    let a = 29911;
    let b = 37516;
    let ret = question_878::Solution::nth_magical_number(n, a, b);
    println!("878:{:?}", ret);

    // 4 start--------------------------------------------------------
    let a = vec![1,2,3];
    let b = vec![4,5,6,7,8,9,10];

    let ret = question_4::Solution::find_median_sorted_arrays(a, b);
    println!("ret is :{}", ret);

    // 30 start -----------------------------------------------------
    let mut s_vec:Vec<std::string::String> = vec![
        "aa".to_string(),
        "aa".to_string(),
        "ab".to_string(),
        "aab".to_string(),
        "mississippi".to_string(),
        "mississippi".to_string(),
        "aaa".to_string(),
    ];
    let mut p_vec:Vec<std::string::String> = vec![
        "a".to_string(),
        "a*".to_string(),
        ".*".to_string(),
        "c*a*b".to_string(),
        "mis*is*p*.".to_string(),
        "mis*is*ip*.".to_string(),
        "ab*ac*a".to_string(),
    ];
    while s_vec.len() != 0{
        let s = s_vec.pop().unwrap();
        let p = p_vec.pop().unwrap();
        let ret = question_10::Solution::is_match(s.clone(), p.clone());
        println!("s:{}, p:{}, ret:{}", s, p, ret);
    }
}

fn main() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let ret = question_30::Solution::find_substring(s, words);
    println!("ret is :{:?}", ret);

    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()];
    let ret = question_30::Solution::find_substring(s, words);
    println!("ret is :{:?}", ret);
}

fn read_data() ->Vec<i32>{
    println!("cur path:{}", env::current_dir().unwrap().display());
    let path = Path::new("data.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let v: Vec<&str> = s.split(",").collect();
    let ret: Vec<i32> = v.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    ret
}