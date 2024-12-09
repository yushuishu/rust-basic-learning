fn main() {
    // map()：在 Option 是 Some 的情况下，通过 map 中提供的函数或闭包把 Option 里的类型转换成另一种类型。
    //      在 Option 是 None 的情况下，保持 None 不变。
    //      map() 会消耗原类型，也就是获取所有权
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);
}