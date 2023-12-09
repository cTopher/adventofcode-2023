pub const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub const fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
