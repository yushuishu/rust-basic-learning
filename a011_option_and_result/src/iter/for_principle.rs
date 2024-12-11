fn main() {
    // 有了迭代器的背景知识后，我们终于要解开 Rust 语言里面 for 语句的真面目了。
    // for 语句是一种语法糖。语句 for item in c {} 会展开成下面这样：
    //      let mut tmp_iter = c.into_iter();
    //      while let Some(item) = tmp_iter.next() {}

    // 也就是说，for 语句默认使用获取元素所有权的迭代器模式，自动调用了 into_iter() 方法。
    // 因此，for 语句会消耗集合 c。
    // 同时也说明，要将一个类型放在 for 语句里进行迭代，需要这个类型实现了迭代器 into_iter() 方法。
    // 标准库中常见的 Range、Vec、HashMap、BtreeMap 等都实现了 into_iter() 方法，因此它们可以放在 for 语句里进行迭代。

    // for 语句作为一种基础语法，它会消耗掉原集合。有时候希望不获取原集合元素所有权，比如只是打印一下，这时只需要获取集合元素的引用 ，应该怎么办呢？
    // Rust 中也考虑到了这种需求，提供了配套的辅助语法，用这两种形式就不会消耗原集合所有权。
    //      用 for in &c {} 获取元素的不可变引用，相当于调用 c.iter()。
    //      用 for in &mut c {} 获取元素的可变引用，相当于调用 c.iter_mut()。

    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];
    // 相当于调用 a.iter()
    for item in &a {
        println!("{}", item);
    }
    // 相当于调用 a.iter_mut()
    for item in &mut a {
        println!("{}", item);
    }
    // 把这一句放在后面：如果先执行这个for循环示例，就会消耗原所有权，导致另外两种for 示例报错
    for item in a {
        println!("{}", item);
    }
    // 你可以试试把这一句打开
    // println!("{:?}", a);

}