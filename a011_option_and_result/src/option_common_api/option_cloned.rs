fn main() {
    // cloned()：通过克隆 Option 里面的内容，把 Option<&T> 转换成 Option
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));
}

