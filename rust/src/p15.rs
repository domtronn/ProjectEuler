// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?

// The numnber of ways of getting to to a location on a grid are XCY where X is row and Y is column
// Therefore, on a 20x20 grid, the bottom right corner will be the 20C10

fn main () {
    let x = 20;
    println!("Result -> {}", (x * 2).choose(x));
}

trait Combinatorics {
    fn choose(&self, y: i64) -> i64;
}

impl Combinatorics for i64 {
    fn choose(&self, mut y: i64) -> i64 {
        let (mut x, mut t, l) = (*self, 1, *self - y);

        while x > l {
            t *= x;  x -= 1;
            while y != 0 && t % y == 0 {
                t = t / y;  y -= 1;
            }
        }
        
        return t ;
    }
}
