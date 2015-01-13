// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn main () {
    println!("Result -> {}", sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15));
}

fn sum_divisible_by(n: i32) -> i32 {
    let limit = 1000i32;
    let mut result = 0i32;
    let mut count = 0i32;

    while count * n < limit {
        result += count * n;
        count += 1;
    }

    return result;
}
