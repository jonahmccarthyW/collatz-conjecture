use rayon::prelude::*;
use std::time::Instant;
use thousands::Separable;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

// const POW3: [u64; 65] = [
//     1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 
//     59049, 177147, 531441, 1594323, 4782969, 14348907, 
//     43046721, 129140163, 387420489, 1162261467, 3486784401, 
//     10460353203, 31381059609, 94143178827, 282429536481, 
//     847288609443, 2541865828329, 7625597484987, 22876792454961, 
//     68630377364883, 205891132094649, 617673396283947, 
//     1853020188851841, 5559060566555523, 16677181699666569, 
//     50031545098999707, 150094635296999121, 450283905890997363, 
//     1350851717672992089, 4052555153018976267, 12157665459056928801, 
//     18026252303461234787, 17185268762964601129, 14662318141474700155, 
//     7093466277004997233, 2833654757305440083, 8500964271916320249, 
//     7056148742039409131, 2721702152408675777, 8165106457226027331, 
//     6048575297968530377, 18145725893905591131, 17543689534297670161, 
//     15737580455473907251, 10319253219002618521, 12511015583298303947, 
//     639558602475808609, 1918675807427425827, 5756027422282277481, 
//     17268082266846832443, 14910758653121394097, 7838787811945079059, 
//     5069619362125685561, 15208858086377056683, 8733086111712066817
// ];

#[inline(always)]
fn get_peak(mut n: u64) -> u64 {
    let original_n = n;
    let mut max_val = n;

    loop {
        let n_plus_1 = n + 1;
        let k = n_plus_1.trailing_zeros();
        
        if k >= 6 { 
            let p3 = unsafe { *POW3.get_unchecked(k as usize) };
            let a = n_plus_1 >> k;
            let peak = (a.wrapping_mul(p3) << 1) - 2;
            
            if peak > max_val {
                max_val = peak;
            }
            
            n = a.wrapping_mul(p3) - 1;
        } else {
            n = 3 * n + 1;
            if n > max_val {
                max_val = n;
            }
        }
        
        n >>= n.trailing_zeros();
        
        if n < original_n {
            break;
        }
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
    let limit: u64 = 10_000_000_000; //pb: 873.2629ms
    let m: u64 = 1 << 26;
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
                } else if current.1 == best.1 && current.0 < best.0 {
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