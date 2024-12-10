fn main() {
    // 迭代器上有一个标准方法，叫作 next()，这个方法返回 Option，其中 Item 就是组成迭代器的元素。
    // 这个方法的字面意思就是迭代出下一个元素。如果这个集合被迭代完成了，那么最后一次执行会返回 None。
    // 比如在迭代器上调用 .next() 返回 u32 数字
    let a = vec![1, 2, 3, 4, 5];
    let mut an_iter = a.into_iter();
    while let Some(x) = an_iter.next() {
        println!("{x}");
    }

}