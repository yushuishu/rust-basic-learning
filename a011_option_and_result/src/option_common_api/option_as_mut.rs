fn main() {
    // as_mut()：把 Option 或 &mut Option 转换成 Option<&mut T>
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));
}