/**
 * @reference https://www.goldennumber.net/math/
 */
fn main() {
    println!("Hello, world!");
    println!("{:?}", fib_i64(0xFFFFFF));
    println!("{:?}th fib number is {:?}", 21, fib_nth_i32(21));
}

static FIVE_O: f64 = 5.0;

static O_FIVE: f64 = 0.5;

// Phi, can also be expressed all in fives as below
// 5 ^ .5 * .5 + .5 = Φ
// (this is what we're doing below):
fn get_phi () -> f64 {
    FIVE_O.powf(O_FIVE) * O_FIVE + O_FIVE
}

fn fib_i64 (limit: i64) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    let mut a = 0;
    let mut b = 1;
    while a <= limit {
        out.push(a);
        if b <= limit {
            out.push(b);
        }
        a = a + b;
        b = a + b;
    }
    out
}

fn fib_nth_i32 (nth: i32) -> i64 {
    ((get_phi().powi(nth) / FIVE_O.powf(O_FIVE)) as i64)
}
