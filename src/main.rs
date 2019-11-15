extern crate time;
pub mod exam;
pub mod list;
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
    list::test();

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