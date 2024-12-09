fn main() {
    // map_err()：当 Result 是 Ok 时，传递原样返回。当 Result 是 Err 时，对 Err 携带的内容使用这个方法提供的函数或闭包进行运算及类型转换。这个方法常常用于转换 Result 的 Err 的负载的类型，在错误处理流程中大量使用
    fn stringify(x: u32) -> String { format!("error code: {x}") }
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));
    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}