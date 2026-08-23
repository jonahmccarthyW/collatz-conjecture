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

#[derive(Copy, Clone)]
struct CollatzJump {
    multiplier_final: u64,
    constant_final: u64,
    multiplier_peak: u64,
    constant_peak: u64,
}

const fn generate_lut() -> [CollatzJump; LUT_SIZE] {
    let mut lut = [CollatzJump { multiplier_final: 0, constant_final: 0, multiplier_peak: 0, constant_peak: 0 }; LUT_SIZE];
    let mut r = 0;
    
    while r < LUT_SIZE {
        let mut m: u64 = 1 << BITS;
        let mut c: u64 = r as u64;
        let mut divisions = 0;
        let mut m_peak = m;
        let mut c_peak = c;
        
        while divisions < BITS {
            if (c & 1) == 0 {
                c >>= 1;
                m >>= 1;
                divisions += 1;
            } else {
                c = 3 * c + 1;
                m = 3 * m;
                
                if m > m_peak || (m == m_peak && c > c_peak) {
                    m_peak = m;
                    c_peak = c;
                }
            }
        }
        
        lut[r] = CollatzJump { 
            multiplier_final: m, 
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
        
        n = a.wrapping_mul(jump.multiplier_final).wrapping_add(jump.constant_final);
        
        if n < original_n {
            break;
        }
    }
    
    max_val
}

// ~8.8ms pre-computation
fn generate_residues(m: u64) -> Vec<u64> {
    (0..m >> 2)
        .into_par_iter()
        .map(|k| (k << 2) | 3)
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

    // Parallelise over residues
    residues
        .par_iter()
        .map(|&r| {
            let mut best_n = 0;
            let mut best_p = 0;
            let mut n = r;

            // Process all full chunks for this residue
            for _ in 0..full_chunks {
                let p = get_peak(n);
                if p > best_p {
                    best_p = p;
                    best_n = n;
                }
                n += CHUNK_SIZE;
            }

            // Check the partial chunk
            if n <= limit {
                let p = get_peak(n);
                if p > best_p {
                    best_p = p;
                    best_n = n;
                }
            }

            (best_n, best_p)
        })
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .unwrap_or((0, 0))
}


fn main() {
    let limit: u64 = 10_000_000_000; //pb: 339.7492ms, Highest peak: 18,144,594,937,356,598,024, n = 8,528,817,511
    
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