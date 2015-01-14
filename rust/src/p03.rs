// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    println!("Result -> {}", find_largest_prime_factor(600851475143));
}

fn find_largest_prime_factor(n:i64) -> i64 {
    let (mut c, mut r) = (3, 0);

    while c * c < n {
        c += 2;
        if is_prime(c) && n % c == 0 { r = c; }
    }
    return r;
}

fn is_prime(n:i64) -> bool {
    if n <= 3 {
        return n >= 2
    } else if n % 2 == 0 || n % 3 == 0 {
        return false
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        return true;
    }

}
