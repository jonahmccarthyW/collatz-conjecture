use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

#[inline]
fn get_peak(mut n: u64) -> u64 {
    let original_n = n;
    let mut max_val = n;
    
    loop {
        n = 3 * n + 1;
        
        if n > max_val {
            max_val = n;
        }
        
        n >>= n.trailing_zeros();

        if n < original_n {
            break;
        }
    }
    
    max_val
}

fn generate_valid_residues(m: u64) -> Vec<u64> {
    let mut allowed = vec![false; m as usize];

    for i in (1..m).step_by(2) {
        let mut a = m;
        let mut b = i;
        let mut drops = false;

        while a % 2 == 0 {
            if b % 2 == 0 {
                a /= 2;
                b /= 2;
            } else {
                a *= 3;
                b = 3 * b + 1;
            }
            
            if a < m {
                drops = true;
                break;
            }
        }
        if !drops {
            allowed[i as usize] = true;
        }
    }

    allowed
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_valid)| if is_valid { Some(i as u64) } else { None })
        .collect()
}

fn main() {
    let start_time = Instant::now();
    let limit: u64 = 10_000_000_000; //pb: 3.7623415s
    let m: u64 = 1 << 20; //check other values
    let valid_residues = generate_valid_residues(m);

    let (max_peak_num, max_peak) = (0..=(limit / m))
        .into_par_iter()
        .fold(
            || (0, 0),
            |mut local_best, k| {
                let base = k * m;
                
                for &r in &valid_residues {
                    let n = base + r;
                    
                    if n > limit {
                        continue;
                    }
                    if n < 10 {
                        continue;
                    }

                    let p = get_peak(n);
                    if p > local_best.1 {
                        local_best = (n, p);
                    }
                }
                local_best
            },
        )
        .reduce(
            || (0, 0),
            |best, current| {
                if current.1 > best.1 {
                    current
                } else if current.1 == best.1 {
                    if best.0 == 0 || current.0 < best.0 { current } else { best }
                } else {
                    best
                }
            },
        );

    let duration = start_time.elapsed();

    println!(
        "\nUp to {}:
- Highest peak: {}, n = {}
- Time taken: {:?}\n",
        limit.separate_with_commas(),
        max_peak.separate_with_commas(),
        max_peak_num.separate_with_commas(),
        duration
    );
}