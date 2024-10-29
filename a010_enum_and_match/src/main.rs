/*
枚举：强大的复合类型
它容纳选项的可能性，每一种可能的选项都是一个变体（variant）。Rust 中的枚举使用关键字 enum 定义，这点与 Java、C++ 都是一样的。
与它们不同的是，Rust 中的枚举具有更强大的表达能力。
在 Rust 中，枚举中的所有条目被叫做这个枚举的变体。
枚举与结构体不同，结构体的实例化需要所有字段一起起作用，而枚举的实例化只需要且只能是其中一个变体起作用。
 Rust 中枚举的强大之处在于，enum 中的变体可以挂载各种形式的类型。所有其他类型，比如字符串、元组、结构体等等，都可以作为 enum 的负载（payload）被挂载到其中一个变体上
枚举的变体能够挂载各种类型的负载，是 Rust 中的枚举超强能力的来源。enum 就像一个筐，什么都能往里面装。
*/
use std::thread::spawn;

fn main() {
    println!("-------------------------------------------------------------- 枚举");
    enum_example();
    println!("-------------------------------------------------------------- 模式匹配");
    pattern_match_example();
}


fn enum_example() {
    // 枚举的实例化实际是枚举变体的实例化
    let shape = Shape::Rectangle { width: 10, height: 50 };
    println!("Shape: {:?}", shape);

    let a = WebEvent::PageLoad;
    let b = WebEvent::PageUnload;
    let c = WebEvent::KeyPress('c');
    let d = WebEvent::Paste(String::from("batman"));
    let e = WebEvent::Click { x: 320, y: 240 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", e);

    // 定义类似 C 语言中的枚举，在定义枚举变体的时候，指定具体的值。这在底层系统级开发、协议栈开发、嵌入式开发的场景会经常用到
    // 使用 as 进行类型的转化
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    // 实例化枚举，impl枚举
    let add = MyEnum::Add;
    // 实例化后执行枚举的方法
    add.run(100, 200);

}

// 定义了一个形状（Shape）枚举，它有三个变体：长方形 Rectangle、三角形 Triangle 和圆形 Circle。
// 定义方式一：
#[derive(Debug)]
enum Shape {
    Rectangle {width: u32, height: u32},
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle{ origin: (u32, u32), radius: u32 },
}
// 定义方式二：
#[derive(Debug)]
enum Shape2 {
    Rectangle,
    Triangle,
    Circle,
}
// 原点坐标
struct Point(u32, u32);
// 矩形
struct Rectangle {
    // 宽
    width: u32,
    // 高
    height: u32
}
// 三角形
struct Triangle {
    // a角的坐标
    a: Point,
    // b角的坐标
    b: Point,
    // c角的坐标
    c: Point,
}
// 原型
struct Circle {
    // 原点坐标
    origin: Point,
    // 半径长度
    radius: u32,
}

// Web 事件
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// 给枚举变体一个起始数字值
enum Number {
    Zero = 0,
    One,
    Two,
}
enum MyEnum {
    Add,
    Subtract,
}
impl MyEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        // match 语句
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn pattern_match_example() {
    // 创建实例
    let shape_a = Shape2::Triangle;
    // 匹配实例：match 表达式中各个分支返回的值的类型必须相同
    let ret = match shape_a {
        Shape2::Rectangle => {
            println!("{:?}", Shape2::Rectangle);
            1
        }
        Shape2::Triangle => {
            println!("{:?}", Shape2::Triangle);
            2
        }
        Shape2::Circle => {
            println!("{:?}", Shape2::Circle);
            3
        }
    };
    println!("ret: {}", ret);

    // _ 占位符，，有时不想处理一些分支，可以用 _ 如果不使用 _ 占位，编译报错
    let shape_b = Shape::Rectangle;
    let ret = match shape_b {
        Shape::Rectangle => { 1 }
        _ => { 10 }
    };
    println!("{}", ret);

    // match 除了配合枚举进行分支管理外，还可以与其他基础类型结合进行分支分派，Java 的 switch .. case 灵活很多
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // 匹配单个数字
        1 => println!("One!"),
        // 匹配几个数字
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个范围，左闭右闭区间
        13..=19 => println!("A teen"),
        // 处理剩下的情况
        _ => println!("Ain't special"),
    }

    // 当要匹配的分支只有两个或者在这个位置只想先处理一个分支的时候，可以直接用 if let
    let shape_a = Shape::Rectangle;
    if let Shape::Rectangle = shape_a {
        println!("1");
    } else {
        println!("10");
    }

    // while 后面也可以跟 let，实现模式匹配
    let mut shape_a = Shape::Rectangle;
    let mut i = 0;
    while let Shape::Rectangle = shape_a {    // 注意这一句
        if i > 9 {
            println!("Greater than 9, quit!");
            shape_a = Shape::Circle;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            i += 1;
        }
    }

    // 元组匹配
    let a = (1, 2, 'a');
    let (b,c,d) = a;
    println!("{:?}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    // 这种种用法叫做元组的析构，常用来从函数的多个返回值里取出数据
    let (b,c,d) = foo();
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    // 匹配结构体
    // 如果匹配时，不适用ref，那么在模式匹配的过程中发生了 partially moved，
    // age 和 student 采用了复制所有权的形式，name 字符串值则是采用了移动所有权的形式
    // a.name 被部分移动到了新的变量 name，所以打印 a 时， a.name 就无法直接使用了
    // ref 这个关键字修饰符告诉 Rust 编译器，现在只需要获得那个字段的引用，不要移动所有权
    // 使用了 ref 后，新定义的 name 变量的值其实是 &a.name
    // 相应的，还有 ref mut 的形式。它是用于在模式匹配中获得目标的可变引用
    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    let User {
        ref name,
        age,
        student,
    } = a;
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    println!("{:?}", a);

    // 函数参数中的模式匹配
    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    foo2(a);
}

fn foo() -> (u32, u32, char) {
    (1, 2, 'a')
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool
}

fn foo2(User { name, age, student }: User) {
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}