use std::time::Instant;

fn count_steps(mut n: u64) -> (u64, u64) {
    let mut steps = 0;
    let mut max_val = n;
    
    while n > 1 {
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
    
    (steps, max_val)
}

fn main() {
    let start_time = Instant::now();

    let mut max_steps = 0;
    let mut max_steps_num = 0;

    let mut max_peak = 0;
    let mut max_peak_num = 0;

    let limit = 100000;

    for i in 1..=limit {
        let (s, peak) = count_steps(i);
        
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
    "Up to {}:
- Most steps: {}, n = {}
- Highest peak: {}, n = {}
- Time taken: {:?}",
    limit, max_steps, max_steps_num, max_peak, max_peak_num, duration
);
}
