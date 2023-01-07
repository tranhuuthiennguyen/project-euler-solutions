use std::collections::HashMap;

fn prime_factor_of(n: &mut u32) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    while *n%2 == 0 {
        *map.entry(2).or_insert(0)+=1;
        *n/=2;
    }

    for i in (3..(f32::sqrt(*n as f32) as u32 + 1)).step_by(2) {
        while *n%i == 0 {
            *map.entry(i).or_insert(0)+=1;
            *n/=i;
        }
    }
    if *n > 2 {
        map.insert(*n, 1);
    }
    map
}

fn smallest_multiple(n: &u32) -> u32 {
    let mut res = 1;
    let mut prime_factor_map : HashMap<u32, u32> = HashMap::new();
    for mut i in 2..(*n + 1) {
        let map = prime_factor_of(&mut i);
        for (k, v) in map {
            if prime_factor_map.contains_key(&k) {
                if *prime_factor_map.get(&k).unwrap() < v {
                    prime_factor_map.insert(k, v);   
                }
            } else {
                prime_factor_map.insert(k, v);
            }
        }
    }
    for (k, v) in prime_factor_map {
        res *= k.pow(v);
    }
    res
}

fn main() {
    let output = smallest_multiple(&20);
    println!("{}",output);
}
