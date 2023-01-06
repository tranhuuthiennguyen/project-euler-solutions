
// using dynamic programming to solve
fn main() {
    let mut a1 = 1;
    let mut a2 = 2;

    let mut sum = 0;
    while a1 <= 4000000 {
        println!("{a1} {a2}");
        if a1%2==0 {
            sum+=a1;
        }
        let temp = a1 + a2;
        a1 = a2;
        a2 = temp;
    }
    println!("{sum}");
}
