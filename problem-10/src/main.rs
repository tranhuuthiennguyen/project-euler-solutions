fn is_prime(n: &u32) -> bool {
    if *n == 2 {
        return true;
    }

    if *n%2 == 0 {
        return false;
    }

    for i in (3..f32::sqrt(*n as f32) as u32 + 1).step_by(2) {
        if n%i == 0 {
            return false;
        }
    }
    true
}

fn prime_sum_up_to(n: u32) -> u64 {
    let mut sum: u64 = 0;
    let mut i = 2;
    while i < n {
        if is_prime(&i) {
            //println!("{}", i);
            sum+=i as u64;
        }
        i+=1;
    }
    sum
}

fn main() {
    let output = prime_sum_up_to(2000000);
    println!("{}", output);
}
