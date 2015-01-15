// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

fn main () {
    println!("Result -> {}", calculate_longest_remainder_cycle_up_to(1000));
}

fn calculate_longest_remainder_cycle_up_to(mut d: i32) -> i32 {
    let mut cycle_length = 0;

    loop {
        d -= 1;
        let d_cycle_length = calculate_remainder_cycle_length(d);
        if d_cycle_length > cycle_length {
            cycle_length = d_cycle_length;
        }
        if d == 0 || cycle_length >= d - 1 { break; }
    }

    return d;
}

fn calculate_remainder_cycle_length(d: i32) -> i32 {
    use std::collections::HashMap;

    let mut found_digits = HashMap::new();
    let (mut r, mut pos) = (1, 0);

    while !found_digits.contains_key(&r) && r != 0 {
        found_digits.insert(r, pos);
        r *= 10; r %= d;
        pos += 1;
    }

    if r == 0 {
        return pos;
    } else {
        return pos - found_digits[r];
    }
}
