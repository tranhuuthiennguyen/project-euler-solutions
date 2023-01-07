fn get_digit(i: &u32, n: &u32) -> u32 {
    let div = 10_i32.pow(*i) as u32;
    (n%div)/(div/10)
}

fn is_palindromic(n: &u32) -> bool {
    let length = (*n as f32).log10() as u32 + 1;
    
    for i in 1..(length/2 + 1) {
        if get_digit(&i, &n) != get_digit(&(length + 1 - i), &n) {
            return false;
        }
    }
    
    true
}

fn main() {
    let mut max = 0;
    for i in (0..1000).rev() {
        for j in (0..1000).rev() {
            let product = i * j;
            if is_palindromic(&product) {
                if product > max {
                    max = product;
                }
            }
        }
    }

    println!("{}", max);
}
