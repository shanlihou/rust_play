extern crate time;
pub mod exam;
pub mod stack;
pub mod question_420;
pub mod question_1044;
pub mod question_149;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::env;


fn main() {
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
    let str_s = "[[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]";
    let mut ret = str_s[2..(str_s.len() - 2)].split("],[");
    for i in ret{
        let is:Vec<&str> = i.split(",").collect();
        let is:Vec<i32> = is.iter().map(|x| x.parse().unwrap()).collect();
        points.push(is);
    }
    question_149::Solution::max_points(points);
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