fn main() {
    // as_ref()：把 Option 或 &Option 转换成 Option<&T>。
    //      创建一个新 Option，里面的类型是原来类型的引用，就是从 Option 到 Option<&T>。
    //      原来那个 Option 实例保持不变
    let text: Option<String> = Some("Hello, world!".to_string());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");
    println!("text_length: {text_length:?}");
}