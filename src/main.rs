use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

fn get_peak(mut n: u64) -> u64 {
    let mut max_val = n;
    
    loop {
        let zeros = n.trailing_zeros();
        n >>= zeros;
        
        if n < 10 {
            break;
        }
        
        n = 3 * n + 1; 
        
        if n > max_val {
            max_val = n;
        }
    }
    
    max_val
}

fn main() {
    let start_time = Instant::now();

    // Goal: 10 Billion
    let limit: u64 = 1_000_000_000; 

    let (max_peak_num, max_peak) = (1..=(limit + 1) / 2)
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