// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

// Let P = ab
// Suppose that P is of the form xyzzyx for some x,y,z in {1,...,9}
// Then take P = 111111 = (100000x + 10000y + 1000z + 100z + 10y + x)
//                    P = 100001x + 10010y + 1100z
//                    P = 11(9091x + 910y + 100z)
//
// Now, since P must have 11 as a prime factor, we know that if 11 does not divide a then 11 must divide b

fn main () {
    println!("Result -> {}", find_largest_palindrome() );
}

fn find_largest_palindrome () -> i32 {
    let (mut largest_palindrome, mut a) = (0i32, 999);
    let (mut b_divisor, mut b);

    while a >= 100 {
        if a % 11 == 0 {
            b = 999;
            b_divisor = 1;
        } else {
            b = 990;
            b_divisor = 11;
        }

        while b >= a {
            if a * b <= largest_palindrome {
                break;
            }

            if is_palindrome(a * b) {
                largest_palindrome = a * b;
            }
            b -= b_divisor;
        }
        a -= 1;
    }

    return largest_palindrome;
}

fn is_palindrome(n:i32) -> bool {
    return n == reverse(n);
}

fn reverse(mut n:i32) -> i32 {
    let mut reverse = 0i32;

    while n > 0 {
        reverse = (10 * reverse) + (n % 10);
        n = n / 10 as i32;
    }
    return reverse;
}
