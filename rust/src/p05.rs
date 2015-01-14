// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main () {
    println!("Result -> {}", find_lcm_of_i_upto(20));
}

fn find_lcm_of_i_upto(mut n:i32) -> i32 {
    let mut result = n;
    let mut inc = n;

    while n > 1 {
        while result % (n - 1) != 0 {
            result += inc;
        }
        inc = result;
        n -= 1;
    }
    return result;
}
