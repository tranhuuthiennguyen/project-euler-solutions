use std::{collections::HashMap, cmp};

fn prime_factor(mut n: u32) -> HashMap<u16, u16> {
    let mut map : HashMap<u16, u16> = HashMap::new();
    
    while n%2 == 0 {
        *map.entry(2).or_insert(0)+=1;
        n/=2;
    }

    for i in (3..f32::sqrt(n as f32) as u32 + 1).step_by(2) {
        while n%i == 0 {
            *map.entry(i as u16).or_insert(0)+=1;
            n/=i;
        }
    }

    if n > 2 {
        map.insert(n as u16, 1);
    }

    map
}

fn main() {
    let mut i = 1;
    let mut sum = i;
    let mut max = 1;
    while max > 0 {
        i+=1;
        sum+=i;
        let factors = prime_factor(sum);
        let mut divisors = 1;
        for (k, v) in &factors {
            divisors*=v+1;
        }
        max = cmp::max(max, divisors);
        if max >= 500 {
            break;
        }
    }

    println!("{}", sum);
}
