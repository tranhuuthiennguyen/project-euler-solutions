use std::vec::Vec;

fn main() {
    let mut v : Vec<i64> = Vec::new();
    let mut num = 600851475143;
    if num%2==0 {
        v.push(2);
        while num%2 == 0 {
            num/=2;
        }
    }

    for i in (3..(f64::sqrt(num as f64) as i64 + 1)).step_by(2) {
        if num%i == 0 {
            v.push(i);
            while num%i == 0 {
                num/=i;
            }
        }
    }
    for n in v.iter() {
        println!("{n}");
    }
}
