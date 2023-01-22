fn collatz(n: u64) -> u64 {
    if n<=1 {
        return n;
    } else if n%2==0 {
        return 1+collatz(n/2);
    } else {
        return 1+collatz(3*n+1);
    }
}

fn main() {
    let mut max_num: u64 = 0;
    let mut max_steps: u64 = 0;
    for i in 1..1000000 {
        let steps = collatz(i);

        if max_steps < steps {
            max_steps = steps;
            max_num = i;
        }
    }
    println!("Result num: {} in {} steps", max_num, max_steps);
}