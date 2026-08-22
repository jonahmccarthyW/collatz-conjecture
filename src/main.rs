use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[inline(always)]
fn get_peak(mut n: u64) -> u64 {
    let original_n = n;
    
    n = 3 * n + 1;
    let mut max_val = n;
    n >>= n.trailing_zeros();

    while n >= original_n {
        n = 3 * n + 1;
        
        if n > max_val {
            max_val = n;
        }
        
        n >>= n.trailing_zeros();
    }
    
    max_val
}

fn generate_residues(m: u64) -> Vec<u64> {
    (0..m / 4)
        .into_par_iter()
        .map(|k| k * 4 | 3)
        .filter(|&i| {
            let mut a = m * 3;
            let mut b = 3 * i + 1;
            
            loop {
                let tz = (a | b).trailing_zeros();
                a >>= tz;
                b >>= tz;
                
                if a < m {
                    return false;
                }
                if (a & 1) != 0 {
                    return true;
                }

                a *= 3;
                b = 3 * b + 1;
            }
        })
        .collect()
}

fn main() {
    let start_time = Instant::now();
    let limit: u64 = 10_000_000_000; //pb: 932.5828ms
    let m: u64 = 1 << 24;
    let residues = generate_residues(m);
    
    let full_chunks = limit / m;

    let (mut peak_num, mut peak) = (0..full_chunks)
        .into_par_iter()
        .fold(
            || (0, 0),
            |mut local_best, k| {
                let base = k * m;
                
                for &r in &residues {
                    let n = base | r;
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
                    current
                } else {
                    best
                }
            },
        );

    let final_base = full_chunks * m;
    for &r in &residues {
        let n = final_base | r;
        
        if n > limit {
            break;
        }
        
        let p = get_peak(n);
        if p > peak {
            peak = p;
            peak_num = n;
        }
    }

    let duration = start_time.elapsed();

    println!(
        "\nUp to {}:
- Highest peak: {}, n = {}
- Time taken: {:?}\n",
        limit.separate_with_commas(),
        peak.separate_with_commas(),
        peak_num.separate_with_commas(),
        duration
    );
}