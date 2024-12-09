fn main() {
    // is_none()：如果 Option 是 None 值，返回 true
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}