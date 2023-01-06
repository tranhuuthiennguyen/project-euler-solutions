fn main() {
    let mut sum = 0;
    let num = 1000;
    for i in 0..num {
        if i%3 == 0 || i%5==0 {
            sum+=i;
        }
    }
    println!("{sum}");
}
