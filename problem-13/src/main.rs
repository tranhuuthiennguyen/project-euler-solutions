fn main() {
    let numbers = include_str!("input.txt");
    let sum: f64 = numbers
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .sum();
    println!("{}", sum.to_string().chars().take(10).collect::<String>());
}