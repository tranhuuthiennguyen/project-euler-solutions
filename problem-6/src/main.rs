fn sum_from_one(n: u32) -> u32 {
    (1 .. n + 1).fold(0, |a, b| a + b)
}

fn main() {
    let n = 100;
    let sum = sum_from_one(n);
    let sum2 = n*(n+1)*(2*n+1)/6;
    let output = sum.pow(2) - sum2;
    println!("{}", output);
}
