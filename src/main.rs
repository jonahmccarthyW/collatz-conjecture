use std::time::Instant;
use thousands::Separable;

fn find_max_peak(mut n: u64, cache: &mut Vec<Option<u64>>) -> u64 {
    if let Some(Some(cached_peak)) = cache.get(n as usize) {
        return *cached_peak;
    }
    
    let original_n = n;
    let mut max_val = n;
    
    while n > 1 {
        if let Some(Some(cached_peak)) = cache.get(n as usize) {
            if *cached_peak > max_val {
                max_val = *cached_peak;
            }
            break;
        }
        
        if n > max_val {
            max_val = n;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    
    if original_n < cache.len() as u64 {
        cache[original_n as usize] = Some(max_val);
    }
    
    max_val
}

fn main() {
    let start_time = Instant::now();

    let mut max_peak = 0;
    let mut max_peak_num = 0;

    let limit = 1000_000;

    // Cache now stores only the peak value (u64) instead of a tuple
    let mut cache = vec![None; limit + 1];
    cache[1] = Some(1);

    for i in 1..=limit {
        let peak = find_max_peak(i as u64, &mut cache);
        
        if peak > max_peak {
            max_peak = peak;
            max_peak_num = i;
        }
    }

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