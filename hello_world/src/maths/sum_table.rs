
pub fn is_prime(n: i32) -> bool {
    let mut i = 2;
    loop {
        if (i * i > n) {
            return false;
        }
        if (n % i == 0) {
            return true;
        }    
        i += 1;
    }
}

pub fn is_prime_bis(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}


pub fn mat_mul<const WA: usize, const WB: usize, const HB: usize>(a: [[i32;WA];WB], b: [[i32;WB];HB]) -> [[i32;WA];HB] {
    let mut c = [[0;WA];HB];
    for i in 0..WB {
        for j in 0..HB {
            for k in 0..WA {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}