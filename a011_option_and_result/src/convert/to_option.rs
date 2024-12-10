fn main() {
    // 从 Result 到 Option：ok()
    // 如果 Result 是 Ok，就把内容重新包在 Some 里。如果 Result 是 Err，就直接换成 None，丢弃 Err 里的内容，同时原 Result 实例被消费。
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);



    // 从 Result 到 Option：err()
    // 如果 Result 是 Ok，直接换成 None，丢弃 Ok 里的内容。如果 Result 是 Err，把内容重新包在 Some 里，同时原 Result 实例被消费
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));

}