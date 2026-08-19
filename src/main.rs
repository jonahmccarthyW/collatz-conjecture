use std::time::Instant;
use thousands::Separable;

fn count_steps(mut n: u64, cache: &mut Vec<Option<(u64, u64)>>) -> (u64, u64) {
    if let Some(Some(result)) = cache.get(n as usize) {
        return *result;
    }
    
    let original_n = n;
    let mut steps = 0;
    let mut max_val = n;
    
    while n > 1 {
        if let Some(Some((cached_steps, cached_max))) = cache.get(n as usize) {
            steps += *cached_steps;
            if *cached_max > max_val {
                max_val = *cached_max;
            }
            if original_n < cache.len() as u64 {
                cache[original_n as usize] = Some((steps, max_val));
            }
            return (steps, max_val);
        }
        
        if n > max_val {
            max_val = n;
        }
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    
    if original_n < cache.len() as u64 {
        cache[original_n as usize] = Some((steps, max_val));
    }
    
    (steps, max_val)
}

fn main() {
    let start_time = Instant::now();

    let mut max_steps = 0;
    let mut max_steps_num = 0;

    let mut max_peak = 0;
    let mut max_peak_num = 0;

    let limit = 1000000; //10000000000

    let mut cache = vec![None; limit + 1];
    cache[1] = Some((0, 1));

    for i in 1..=limit {
        let (s, peak) = count_steps(i as u64, &mut cache);
        
        if s > max_steps {
            max_steps = s;
            max_steps_num = i;
        }
        
        if peak > max_peak {
            max_peak = peak;
            max_peak_num = i;
        }
    }

    let duration = start_time.elapsed();

    println!(
        "\nUp to {}:
- Most steps: {}, n = {}
- Highest peak: {}, n = {}
- Time taken: {:?}\n",
        limit.separate_with_commas(),
        max_steps.separate_with_commas(),
        max_steps_num.separate_with_commas(),
        max_peak.separate_with_commas(),
        max_peak_num.separate_with_commas(),
        duration
    );
}