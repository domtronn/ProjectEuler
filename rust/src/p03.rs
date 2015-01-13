// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    println!("Result -> {}", is_prime(7));
}

// fn find_largest_prime_factor(n:i64) {
//     loop {
        
//     }
// }

fn is_prime(n:i64) -> bool {
    if n <= 3 {
        return n >= 2
    } else if n % 2 == 0 || n % 3 == 0 {
        return false
    } else {
        let mut i = 5:i32;
        while (i * i <= n) {
            if (n % i == 0 || n % (i + 2) == 0) {
                return false;
            }
            i += 6;
        }
        return true;
    }

}
