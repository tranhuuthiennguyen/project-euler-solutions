use std::{
    io::{self, BufRead}, 
    fs::File, 
    path::Path,
    cmp
};

fn max_horizontal(lines: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for line in lines {
        for i in 0..17 {
            let product = &line[i..i+4].into_iter().map(|&x| x as u32).product::<u32>();
            max = cmp::max(max, *product);
        }
    }
    max
}

fn max_vertical(lines: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for i in 0..20 {
        for j in 0..17 {
            let product = &lines[j..j+4].iter().map(|x| x[i] as u32).product::<u32>();
            max = cmp::max(max, *product);
        }
    }
    max
}

fn max_diagonal(lines: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for i in (4..20).rev() {
        for j in 0..17 {
            let mut product = 1;
            for k in 0..4 {
                product *= lines[i-k][j+k] as u32;
            }
            max = cmp::max(max, product);
        }
    }
    max
}

fn main () {
    let lines = read_lines("grid.txt");
    let horizon = max_horizontal(&lines);
    let vertical = max_vertical(&lines);
    let diagonal = max_diagonal(&lines);
    println!("{horizon} {vertical} {diagonal}");
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("No such file");
    let br = io::BufReader::new(file);
    br.lines()
        .map(|line| line.unwrap()
                    .split_whitespace()
                    .into_iter()
                    .map(|c| c.parse::<u8>()
                                    .unwrap())
                                    .collect())
        .collect()
}