
// 控制流语句
fn main() {
    println!("-------------------------------------------------------------- simple_if");
    simple_if();
    println!("-------------------------------------------------------------- simple_loop");
    simple_loop();
    println!("-------------------------------------------------------------- simple_while");
    simple_while();
    println!("-------------------------------------------------------------- simple_for");
    simple_for();
    println!("-------------------------------------------------------------- simple_interval");
    simple_for_interval();
}

fn simple_if() {
    let x = 1;
    // 实现了类似其它语言中的三目运算符这样的设计，在 Rust 中，不需要额外提供那样的特殊语法。
    let y = if x == 0 {
        // 代码块最后一句不加分号，表示把值返回回去
        100
    } else {
        // 代码块最后一句不加分号，表示把值返回回去
        101
    };
    println!("y is {}", y);
}
fn simple_loop() {
    // 用于无条件（无限）循环
    let mut counter = 0;
    // 这里接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值
            break counter * 2;
        }
    };
    // 这种返回一个值到外面对一个变量初始化的方式，是 Rust 中的习惯用法，这能让代码更紧凑。
    println!("the result is {result}")
}
fn simple_while() {
    // while 循环为条件判断循环，当后面的条件为真的时候，执行循环体里面的代码
    // 和前面的 if 语句一样，Rust 中的 while 后面的条件表达式不推荐用（）包裹起来
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    
}
fn simple_for() {
    // for 循环在 Rust 中，基本上只用于迭代器（暂时可以想象成对数组，动态数组等）的遍历。Rust 中没有 C 语言那种 for 循环风格的语法支持，因为那被认为是一种不好的设计
    let a = [10, 20, 30, 40, 50];
    for item in a {
        println!("value is {item}");
    }
}

fn simple_for_interval() {
    // 生成遍历区间： ..（两个点）

    // 左闭右开
    for num in 1..4 {       
        println!("左闭右开：{num}"); 
    }
    println!("------------");
    // 左闭右闭
    for num in 1..=4 {
        println!("左闭右闭：{num}"); 
    }
    println!("------------");
    // 反向
    for num in (1..4).rev() {
        println!("反向：{num}"); 
    }
    println!("------------");
    // 字符
    for ch in 'a'..='d' {
        println!("字符：{ch}"); 
    }
}

