pub fn fibo(a : i16) -> i16 {
    if (a == 0) {
        return 0;
    }
        
    if (a == 1) {
        return 1;
    }

    return fibo(a - 1) + fibo(a - 2);
}

fn anoter_fibonacci(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}



fn yet_again_fibo(n: usize) -> usize {
    let mut level = 0;
    let (mut curr, mut next) = (0, 1);
    while level < n {
        (curr, next) = (next, curr + next);
        level += 1;
    }
    curr
}


#[test]
fn test_fibo() {
    assert_eq!(fibo(0), 0);
    assert_eq!(fibo(1), 1);
    assert_eq!(fibo(2), 1);
    assert_eq!(fibo(3), 2);
    assert_eq!(fibo(4), 3);
    assert_eq!(fibo(5), 5);
    assert_eq!(fibo(6), 8);
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    const fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    // Use checked_add to avoid overflow
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}