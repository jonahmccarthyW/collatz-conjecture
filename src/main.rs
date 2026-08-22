use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const BITS: u32 = 10;
const LUT_SIZE: usize = 1 << BITS;
const MASK: u64 = (1 << BITS) - 1;
const CHUNK_POWER: u32 = 24;
const CHUNK_SIZE: u64 = 1 << CHUNK_POWER;

// Precomputed powers of 3.
const POW3: [u64; 65] = {
    let mut arr = [0; 65];
    let mut i = 0;
    let mut current: u64 = 1;
    while i < 65 {
        arr[i] = current;
        current = current.wrapping_mul(3);
        i += 1;
    }
    arr
};

#[derive(Copy, Clone)]
struct CollatzJump {
    p: u32, // Number of odd steps (3^p)
    constant_final: u64,
    multiplier_peak: u64,
    constant_peak: u64,
}

// Generate the 12-bit LUT
const fn generate_lut() -> [CollatzJump; LUT_SIZE] {
    let mut lut = [CollatzJump { p: 0, constant_final: 0, multiplier_peak: 0, constant_peak: 0 }; LUT_SIZE];
    let mut r = 0;
    
    while r < LUT_SIZE {
        let mut m: u64 = 1 << BITS;
        let mut c: u64 = r as u64;
        let mut divisions = 0;
        let mut odd_steps = 0;
        let mut m_peak = m;
        let mut c_peak = c;
        
        while divisions < BITS {
            if c % 2 == 0 {
                c >>= 1;
                m >>= 1;
                divisions += 1;
            } else {
                c = 3 * c + 1;
                m = 3 * m;
                odd_steps += 1;
                
                if m > m_peak || (m == m_peak && c > c_peak) {
                    m_peak = m;
                    c_peak = c;
                }
            }
        }
        
        lut[r] = CollatzJump { 
            p: odd_steps, 
            constant_final: c, 
            multiplier_peak: m_peak, 
            constant_peak: c_peak };
        r += 1;
    }
    lut
}

static JUMP_LUT: [CollatzJump; LUT_SIZE] = generate_lut();

#[inline(always)]
fn get_peak(mut n: u64) -> u64 {
    let original_n = n;
    let mut max_val = n;

    loop {
        let r = (n & MASK) as usize;
        let a = n >> BITS;
        
        let jump = unsafe { JUMP_LUT.get_unchecked(r) };
        
        let local_peak = a.wrapping_mul(jump.multiplier_peak).wrapping_add(jump.constant_peak);
        if local_peak > max_val {
            max_val = local_peak;
        }
        
        let p3 = unsafe { *POW3.get_unchecked(jump.p as usize) };
        n = a.wrapping_mul(p3).wrapping_add(jump.constant_final);
        
        if n < original_n {
            break;
        }
    }
    
    max_val
}

// ~30ms pre-computation
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
                
                if a < m { return false; }
                if (a & 1) != 0 { return true; }

                a *= 3;
                b = 3 * b + 1;
            }
        })
        .collect()
}

fn find_highest_peak(limit: u64) -> (u64, u64) {
    let residues = generate_residues(CHUNK_SIZE);
    let full_chunks = limit / CHUNK_SIZE;

    // Process full chunks in parallel
    let (mut peak_num, mut peak) = (0..full_chunks)
        .into_par_iter()
        .fold(
            || (0, 0),
            |mut local_best, k| {
                let base = k * CHUNK_SIZE;
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
                if current.1 > best.1 || (current.1 == best.1 && current.0 < best.0) {
                    current
                } else {
                    best
                }
            },
        );

    let final_base = full_chunks * CHUNK_SIZE;
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
    (peak_num, peak)
}


fn main() {
    let limit: u64 = 10_000_000_000; //pb: 431.3827ms, Highest peak: 18,144,594,937,356,598,024, n = 8,528,817,511
    
    let start_time = Instant::now();
    let (peak_num, peak) = find_highest_peak(limit);
    let duration = start_time.elapsed();

    println!(
        "\nResults for limit = {}:
- Highest peak: {} (Found at n = {})
- Time taken:   {:?}\n",
        limit.separate_with_commas(),
        peak.separate_with_commas(),
        peak_num.separate_with_commas(),
        duration
    );
}