fn main() {
    // and_then()：当 Result 是 Ok 时，把这个方法提供的函数或闭包应用到 Ok 携带的内容上面，并返回一个新的 Result。
    //      当 Result 是 Err 的时候，这个方法直接传递返回这个 Err 和它的负载。
    //      这个方法常常用于一路链式操作，前提是过程里的每一步都需要返回 Result
    fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
        x.checked_mul(x).map(|sq| sq.to_string()).ok_or("overflowed")
    }
    assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    assert_eq!(Err("not a number").and_then(sq_then_to_string), Err("not a number"));
}