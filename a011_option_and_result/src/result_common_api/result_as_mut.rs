fn main() {
    // as_mut()：创建一个新 Result，里面的两种类型分别是原来两种类型的可变引用，就是从 Result 到 Result<&mut T, &mut E>。原来那个 Result 实例保持不变
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x: Result<i32, i32> = Ok(2);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 42);
    let mut x: Result<i32, i32> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap_err(), 0);
}