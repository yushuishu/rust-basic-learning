fn main() {
    // map()：当 Result 是 Ok 的时候，把 Ok 里的类型通过参数里提供的函数运算并且可以转换成另外一种类型。当 Result 是 Err 的时候，原样返回 Err 和它携带的内容
    let line = "1\n2\n3\n4\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{n}"),
            Err(..) => {}
        }
    }
}