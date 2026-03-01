use std::collections::HashMap;

pub fn factors(n: u64) -> HashMap<u64, usize> {
    let max = (n as f64).sqrt().floor() as u64;
    let primes = primes_below(max);

    let mut result = HashMap::new();
    let mut i = 0;
    let mut m = n;

    loop {
        let p = primes[i];
        let mut div = true;
        while div {
            div = m % p == 0;
            if div {
                *result.entry(p).or_insert(0) += 1;
                m = m / p;
            }
        }
        i += 1;
        if i == primes.len() {
            if m > 1 {
                result.insert(m, 1);
            }
            break;
        }
    }
    result
}

fn primes_below(n: u64) -> Vec<u64> {
    let mut is_prime = vec![true; n as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let m = (n as f64).sqrt().floor() as u64;

    for i in 2..=m {
        let mut j = 2;
        while i * j < n {
            is_prime[(i * j) as usize] = false;
            j += 1;
        }
    }

    is_prime
        .into_iter()
        .enumerate()
        .filter(|&(_, b)| b)
        .map(|(i, _)| i as u64)
        .collect()
}
