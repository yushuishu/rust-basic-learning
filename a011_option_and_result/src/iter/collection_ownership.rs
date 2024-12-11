fn main() {
    // 获取集合类型中元素的所有权
    // 想要获取 Vec 里的一个元素，只需要下标操作就可以了
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1, s2, s3, s4];
    // 这里，我们想访问 s1 的内容，无法编译通过，提示不能从 Vec 中用下标操作符移出元素
    // let a = v[0];
    // 改一下代码，明确 a 只获得 v 中第一个元素的引用
    let a = &v[0];

    // 可能为了从集合中获得 s1 的所有权，而不得不使用 let a = v[0].clone()。
    // 而根据迭代器知识，使用 into_iter() 就可以拿到并操作上述动态数组 v 中元素的所有权

}