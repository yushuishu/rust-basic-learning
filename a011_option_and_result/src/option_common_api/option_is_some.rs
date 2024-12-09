fn main() {
    // is_some()：如果 Option 是 Some 值，返回 true
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}