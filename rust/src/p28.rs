// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

// 43 44 45 46 47 48 49
// 42 21 22 23 24 25 26
// 41 20  7  8  9 10 27
// 40 19  6  1  2 11 28
// 39 18  5  4  3 12 29
// 38 17 16 15 14 13 30
// 37 36 35 34 33 32 31

// It can be verified that the sum of the numbers on the diagonals is 101.
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

fn main () {
    println!("Result -> {}", sum_diagonals_of_spirals_up_to(501));
}

fn sum_diagonals_of_spirals_up_to(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += sum_diagonals_of_spiral(n);
        n -= 1;
    }

    return sum;
}

fn sum_diagonals_of_spiral(n: i32) -> i32 {
    let i = calculate_first_i_of_spiral(n);
    let l = calculate_limit_of_spiral(n, i);
    let (mut r, mut sum) = (i - 1, 0);

    if n == 1 { return 1; }
    
    while r < l {
        r += 2 * (n - 1);
        sum += r;
    }
    return sum;
}

fn calculate_limit_of_spiral(n: i32, i: i32) -> i32 {
    return i + ((n - 1) * 8) - 1;
}

fn calculate_first_i_of_spiral(n: i32) -> i32 {
    if n == 1 { return 1; }
    if n == 2 { return 2; }
    return calculate_first_i_of_spiral(n - 1) + ((n - 2) * 8);
}
