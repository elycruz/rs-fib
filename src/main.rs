fn main() {
    println!("Hello, world!");
    println!("{:?}", fib_i64(0xFFFFFF));
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
