fn main() {
    // 从Option 到 Result：ok_or()
    // Option 实例如果是 Some，直接把内容重新包在 Result::Ok() 里。如果是 None，使用 ok_or() 里提供的参数作为 Err 的内容
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));
}