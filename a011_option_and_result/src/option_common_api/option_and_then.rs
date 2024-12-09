fn main() {
    // and_then()：如果 Option 是 None，返回 None；如果 Option 是 Some，就把参数里面提供的函数或闭包应用到被包裹的内容上，并返回运算后的结果
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }
    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);
}