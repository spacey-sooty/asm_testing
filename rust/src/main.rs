extern "C" {
    fn add(a: i64, b: i64) -> i64;
    fn minus(a: i64, b: i64) -> i64;
}

fn main() {
    let sum = unsafe { add(1, 2) };

    println!("Sum = {}\n", sum);

    let sub = unsafe { minus(2, 1) };

    println!("Sub = {}\n", sub);
}

#[cfg(test)]
mod tests {
    use super::{add, minus};

    #[test]
    fn assert_add() {
        assert_eq!(unsafe { add(1, 2) }, 3);
        assert_eq!(unsafe { add(1, 4) }, 5);
    }

    #[test]
    fn assert_minus() {
        assert_eq!(unsafe { minus(1, 2) }, 1);
        assert_eq!(unsafe { minus(1, 0) }, -1);
    }
}
