fn main() {
    // is_err()：如果 Result 是 Err，返回 true
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_err(), false);
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_err(), true);
}