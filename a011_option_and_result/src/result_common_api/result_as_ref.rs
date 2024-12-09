fn main() {
    // as_ref()：创建一个新 Result，里面的两种类型分别是原来两种类型的引用，就是从 Result 到 Result<&T, &E>。原来那个 Result 实例保持不变
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.as_ref(), Ok(&2));
    let x: Result<u32, &str> = Err("Error");
    assert_eq!(x.as_ref(), Err(&"Error"));
}