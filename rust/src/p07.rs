// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn main () {
    println!("Result -> {}", calculate_nth_prime(10001));
}

fn calculate_nth_prime(n: i32) -> i32 {
    let (mut c, mut i) = (1i32, 1i32);
    while c < n {
        i += 2;
        if i.is_prime() { c += 1 }
    }
    return i;
}

trait Primality {
    fn is_prime(&self) -> bool;
}

impl Primality for i32 {
    fn is_prime(&self) -> bool {
        let n = *self;
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
}
