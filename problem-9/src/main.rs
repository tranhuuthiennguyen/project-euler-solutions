fn pygathorean_triplet(n : u32) -> u32 {
    let mut product = 1;
    for i in 1..n/3+1 {
        for j in i+1..n/2+1 {
            let k = n - i - j;
            if i.pow(2) + j.pow(2) == k.pow(2) {
                product*=i*j*k;
            }
        }
    }
    product
}

fn main() {
    let output = pygathorean_triplet(1000);
    println!("{}", output);
}
