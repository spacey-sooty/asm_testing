extern "C" {
    fn add(a: i64, b: i64) -> i64;
    fn minus(a: i64, b: i64) -> i64;
}

fn main() {
    let sum = unsafe { add(1, 2) };

    println!("Sum = {}", sum);

    let sub = unsafe { minus(2, 1) };

    println!("Sub = {}", sub);
}
