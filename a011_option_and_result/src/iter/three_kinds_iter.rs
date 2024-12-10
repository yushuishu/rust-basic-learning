fn main() {
    // Rust 中的迭代器根据所有权三态可以分成三种。
    //      1、获取集合元素不可变引用的迭代器，对应方法为 iter()。
    //      2、获取集合元素可变引用的迭代器，对应方法为 iter_mut()。
    //      3、获取集合元素所有权的迭代器，对应方法为 into_iter()。

    let mut a = [1, 2, 3];

    // 转换第一种迭代器
    let mut an_iter = a.iter();
    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 转换第二种迭代器
    let mut an_iter = a.iter_mut();
    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 转换第三种迭代器
    let mut an_iter = a.into_iter();
    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 为什么使用了所有权迭代器into_iter()，这里还能打印出整数i32数组？
    // 原因：
    //      整型、浮点型、布尔值、字符、元组和数组等，都实现了 Copy trait，当它们被赋值或传递时，默认是做复制所有权操作
    //      非 Copy 类型：如 String、Vec<T>、HashMap 等，当它们被赋值或传递时，会发生所有权的移动，这意味着原变量将不再可用。
    println!("{:?}", a);



    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];

    // 转换第一种迭代器
    let mut an_iter = a.iter();
    assert_eq!(Some(&"1".to_string()), an_iter.next());
    assert_eq!(Some(&"2".to_string()), an_iter.next());
    assert_eq!(Some(&"3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 转换第二种迭代器
    let mut an_iter = a.iter_mut();
    assert_eq!(Some(&mut "1".to_string()), an_iter.next());
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // 转换第三种迭代器
    let mut an_iter = a.into_iter();
    assert_eq!(Some("1".to_string()), an_iter.next());
    assert_eq!(Some("2".to_string()), an_iter.next());
    assert_eq!(Some("3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // ide提示错误；编译报错
    // 使用所有权的迭代器into_iter()，String会发生所有权转移
    // println!("{:?}", a);

}