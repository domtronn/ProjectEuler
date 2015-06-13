// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main () {
    let n = 100;
    println!("Result -> {}", (sum(n) * sum(n)) - sum_sq(n));
}

fn sum(n: i32) -> i32 {
    return (n * (n + 1)) / 2;
}

fn sum_sq(n: i32) -> i32 {
    return (((2 * n) + 1) * (n + 1) *  n) / 6;
}
