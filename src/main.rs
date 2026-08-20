use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

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

fn main() {
    let start_time = Instant::now();

    // Goal: 10 Billion, 27.8454517s
    let limit: u64 = 10_000_000_000; 

    let (max_peak_num, max_peak) = (10..=(limit + 1) / 2)
        .into_par_iter()
        .map(|i| {
            let odd_num = i * 2 - 1;
            (odd_num, get_peak(odd_num))
        })
        .reduce(
            || (0, 0),
            |best_so_far, current| {
                if current.1 > best_so_far.1 {
                    current
                } else {
                    best_so_far
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