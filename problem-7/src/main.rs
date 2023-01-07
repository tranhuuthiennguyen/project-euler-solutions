fn is_prime(n: &u32) -> bool {
    if *n == 2 { return true; }

    for i in 3..(f32::sqrt(*n as f32)) as u32 + 1 {
        if n%i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut prime = 3;
    let mut i = 2;
    while i < 10001 {
        prime+=2;
        if is_prime(&prime) {
            i+=1;
        }
    }
    println!("{}", prime);
}
