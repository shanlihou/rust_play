#[derive(Debug)]
pub struct CalcRank {
    a: i64,
    b: i64,
    gcd: i64,
    lcm: i64,
    range: i64,
}

static G_MOD:i64 = 1000000007;

impl CalcRank {
    pub fn calc_gcd(a:i64, b:i64) -> i64 {
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

    pub fn calc_rate_by_num(&self, num:i64) -> i64 {
        if num >= self.lcm {
            return -1
        }

        num / self.a + num / self.b
    }

    pub fn calc_num_in_range(&self, rate:i64) -> i64{
        println!("self:{:?}, rate:{:?}", self, rate);
        let _b = (rate * (self.a / self.gcd) / self.range) * self.b;
        let _a = (rate * (self.b / self.gcd) / self.range) * self.a;
        println!("_a:{:?}, _b:{:?}", _a, _b);
        if self.calc_rate_by_num(_b) == rate {
            _b
        }
        else if self.calc_rate_by_num(_b + self.b) == rate {
            _b + self.b
        }
        else if self.calc_rate_by_num(_a) == rate {
            _a
        }
        else {
            _a + self.a
        }
    }

    pub fn get_nth(&self, n: i64) -> i64 {
        let times = n / self.range;
        let rate = n % self.range;
        let mod_num = self.calc_num_in_range(rate);
        ((times as u64 * self.lcm as u64) as i64 + mod_num) % G_MOD
    }

    pub fn new(a:i64, b:i64) -> Self {
        let gcd = CalcRank::calc_gcd(a, b);
        let lcm = a * b / gcd;
        let range = (a + b - gcd) / gcd;
        CalcRank{a, b, gcd, lcm, range}
    }
}

pub struct Solution {

}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let calc_rank = CalcRank::new(a as i64, b as i64);
        calc_rank.get_nth(n as i64) as i32
    }
}