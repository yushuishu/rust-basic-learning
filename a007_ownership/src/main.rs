
// 所有权的基础是三条定义: 
//      1、Rust 中，每一个值都有一个所有者
//      2、任何一个时刻，一个值只有一个所有者
//      3、当所有者所在作用域（scope）结束的时候，其管理的值会被一起释放掉
fn main() {
    //scope1();
    //scope1();
    //ownership1();
    //ownership2();

    // 默认做复制所有权的操作的有 7 种：
    //      所有的整数类型，比如 u32；
    //      布尔类型 bool；
    //      浮点数类型，比如 f32、f64；
    //      字符类型 char；
    //      由以上类型组成的元组类型 tuple，如（i32, i32, char）；
    //      由以上类型组成的数组类型 array，如 [9; 100]；
    //      不可变引用类型 &。
    // 其他类型默认都是做移动所有权的操作。
    //ownership3();
    //ownership4();
    //ownership5();
    ownership6();
    ownership7();
    ownership8();

    // 关于引用（借用）的一些规则：
    //      所有权型变量的作用域是从它定义时开始到所属那层花括号结束。
    //      引用型变量的作用域是从它定义起到它最后一次使用时结束。
    //      引用（不可变引用和可变引用）型变量的作用域不会长于所有权变量的作用域。这是肯定的，不然就会出现悬锤引用，这是典型的内存安全问题。
    //      一个所有权型变量的不可变引用可以同时存在多个，可以复制多份。
    //      一个所有权型变量的可变引用与不可变引用的作用域不能交叠，也可以说不能同时存在。
    //      某个时刻对某个所有权型变量只能存在一个可变引用，不能有超过一个可变借用同时存在，也可以说，对同一个所有权型变量的可变借用之间的作用域不能交叠。
    //      在有借用存在的情况下，不能通过原所有权型变量对值进行更新。当借用完成后（借用的作用域结束后），物归原主，又可以使用所有权型变量对值做更新操作了。
    ownership9();
    ownership10();
    ownership11();
    ownership12();
    ownership13();

    // 可变引用的再赋值，会执行移动操作，原来的那个可变引用变量就不能用了，
    // 一个所有权型变量的可变引用也具有所有权特征，它可以被理解为那个所有权变量的独家代理，具有排它性。
    ownership14();
    ownership15();

    // 多级引用规则：
    //      对于多级可变引用，要利用可变引用去修改目标资源的值的时候，需要做正确的多级解引用操作，比如例子中的 **c，做了两级解引用。
    //      只有全是多级可变引用的情况下，才能修改到目标资源的值。
    //      对于多级引用（包含可变和不可变），打印语句中，可以自动为我们解引用正确的层数，直到访问到目标资源的值，这很符合人的直觉和业务的需求。
    multi_level_quote1();
    multi_level_quote2();
    multi_level_quote3();
    multi_level_quote4();

    // 使用引用，改进函数 ownership3()
    improve_fn();
}

fn scope1() {
    let s = String::from("hello"); 
    
    // do stuff with s
    // 变量s的作用域到这里结束
}

fn scope2() {
    let a = 1u32;
    { 
        let s = String::from("hello");

        // 变量s的作用域到这里结束
    }

    // xxx

    // 变量a的作用域到这里结束
}

fn ownership1() {
    // a 具有对值 10u32 的所有权
    let a = 10u32;
    // 把值 10u32 复制了一份，b 具有对这个新的 10u32 值的所有权
    let b = a;
    println!("{a}");
    println!("{b}");

    // a、b 两个变量就离开了作用域，其对应的两个 10u32，就都被回收了
    // 这里是栈帧结束，栈帧内存被回收，局部变量位于栈帧中，所以它们所占用的内存就被回收了
}

fn ownership2() {
    // 变量 s1 持有这个字符串的所有权
    let s1 = String::from("I am a superman.");
    // s2 持有那个字符串的所有权，s1 处于无效状态（invalid）
    let s2 = s1;
    
    // 输出报错，因为字符串的所有权移交给了s2， s1 处于无效状态（invalid）
    //println!("{s1}");

    println!("{s2}");

    // 2 所拥有的字符串内存，就被回收掉了，s1 所对应的那个局部变量的内存空间也一并被回收了
}

fn ownership3_foo(s:String) {
    // 函数的参数 s 获取了值的所有权。
    // 函数参数是这个函数的一个局部变量，它在这个函数栈帧结束的时候会被回收，
    // 因此字符串在被调用者中已经不存在了
    println!("s is {s}");
}

fn ownership3() {
    let s1 = String::from("I am a superman.");
    ownership3_foo(s1);

    // 编译报错：value borrowed here after move
    // s1 所有权已经被移动进函数里面了，不能在移动后再使用了。
    println!("s1 is {s1}");

    // Java 默认做了引用的拷贝，并且新旧两个变量同时指向原来那个对象。
    // 而 Rust 不一样，Rust 虽然也是把字符串的引用由 s1 拷贝到了 s2，但是只保留了最新的 s2 到字符串的指向，同时却把 s1 到字符串的指向给“抹去”了
}

fn ownership4() {
    let s1 = String::from("I am a superman.");
    // 既然能把所有权移动到函数里面，也当然能把所有权转移出来。
    s1 = ownership4_foo(s1);
    println!("s1 is {s1}");
}

fn ownership4_foo(s:String) -> String {
    println!("s is {s}");
    // 既然能把所有权移动到函数里面，也当然能把所有权转移出来。
    s
}

fn ownership5() {
    let s1 = String::from("I am a superman.");

    for i in 1..10 {
        // 编译报错
        // 第一遍循环时（i = 1），s1 字符串值的所有权移交给了temp_s
        // 第二遍循环时（i = 2），s 是无状态的，无效的，此时报错
        
        //let temp_s = s;
        //println!("temp_s is {}", temp_s);
    }
}

struct Point {
    x:i64,
    y:i64,
    z:i64
}

fn ownership6() {
    let point1 = Point {
        x: 50,
        y: 100,
        z: 10,
    };
    let point2 = point1;

    // 报错：value borrowed here after move
    // 结构体的所有权已经被移交给了 point2，不能再使用 point1
    //println!("point1.x:{}, point1.y:{}, point1.z:{}", point1.x, point1.y, point1.z);
    println!("point2.x:{}, point2.y:{}, point2.z:{}", point2.x, point2.y, point2.z);
}


// 借用概念也是实际生活中思维的映射。比如你有一样东西，别人想用一下，可以从你这里借，你可以借出。
// 那“引用”概念又是什么呢？其实在 Rust 中，借用和引用是一体两面。你把东西借给别人用，也就是别人持有了对你这个东西的引用。

// 在 Rust 中，变量前用“&”符号来表示引用，比如 &x。

// 其实引用也是一种值，并且是固定尺寸的值，一般来说，与机器 CPU 位数一致，比如 64 位或 32 位。因为是值，
// 所以就可以赋给另一个变量。同时它又是固定的而且是小尺寸的值，那其实赋值的时候，就可以直接复制一份这个引用。
fn ownership7() {
    let a = 10u32;
    let b = &a;        // b是变量a的一级引用
    let c = &&&&&a;    // c是变量a的多级引用
    let d = &b;        // d是变量a的间接引用
    let e = b;         // 引用b再赋值给e
    
    // Rust 识别了我们一般情况下的意图，不会打印出引用的内存地址什么的，而是打印出了被引用对象的值。

    // b 和 e 都是对 a 的一级引用
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
   
}

fn ownership8() {
    // 字符串的所有权仍然在 s1，
    // s2、s3、s4、s5 都是对这个所有权变量的引用
    let s1 = String::from("I am a superman.");
    let s2 = &s1;
    let s3 = &&&&&s1;
    let s4 = &s2;
    let s5 = s2;
    
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    println!("{s5}");
}

fn ownership9() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{a}"); // 这一句移到前面 这里来：报错
    println!("{b}");
    
    // 报错原因：不可变借用 与 可变借用 不能重叠交叉
    // println!()打印，会对所有权变量做不可变借用操作，
    // b是可变借用操作：作用域范围是 let b = &mut a; 到 println!("{b}");
}

fn ownership10() {
    let mut a = 10u32;

    // 可变借用
    let b = &mut a;
    *b = 20;

    // 不可变借用：在利用b更新了a的值后，c再次借用a
    let c = &a;
    
    // 可变借用 在这里使用了
    println!("{b}");  // 加了一句打印语句


    // 报错原因：不可变借用 与 可变借用 不能重叠交叉
    // 不可变借用的作用域范围就一行  let c = &a;
    // 可变借用的作用域范围是 let b = &mut a; 到 println!("{b}");
}

fn ownership11() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let d = &mut a;
    
    println!("{b}");      // 打印b

    // 报错原因：同一个所有权型变量的可变借用之间的作用域也不能交叠（可变借用 与 可变借用 之间不能重叠交叉）
}

fn ownership12() {
    let mut a = 10u32;
    let r1 = &a;
    a = 20;
    
    
    println!("{r1}");

    // 报错原因：在有 借用 的情况下，不能对所有权变量进行更改值的操作（写操作）
}

fn ownership13() {
    let mut a = 10u32;
    let r1 = &mut a;
    a = 20;
    
    
    println!("{r1}");

    // 报错原因：在有 借用 的情况下，不能对所有权变量进行更改值的操作（写操作）
}

fn ownership14() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    
    // 报错提示：r1 的值移动给了 r2，因此 r1 不能再被使用了。
    println!("{r1}")

    // 可变引用的再赋值，会执行移动操作，原来的那个可变引用变量就不能用了，
    // 一个所有权型变量的可变引用也具有所有权特征，它可以被理解为那个所有权变量的独家代理，具有排它性。
}

fn ownership15() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    
    // 打印r2，输出10
    println!("{r2}");
}

fn multi_level_quote1() {
    let mut a1 = 10u32;
    let mut a2 = 15u32;

    let mut b = &mut a1;
    b = &mut a2;

    let mut c = &a1;
    c = &a2;
}

fn multi_level_quote2() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    let c = &mut b;
    // 多级解引用操作
    **c = 30;
    
    println!("{c}");
}

fn multi_level_quote3() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    let c = &mut b;
    // 这里对二级可变引用只使用一级解引用操作，
    // 报错。它正确识别到了中间引用的类型为 &mut u32，而我们却要给它赋值为 u32
    *c = 30;
    
    println!("{c}");
}

fn multi_level_quote4() {
    let mut a1 = 10u32;
    let b = &mut a1;
    let mut c = &b;
    let d = &mut c;
    
    // 提示：不能这样更新目标的值，因为目标躲在一个 & 引用后面。
    ***d = 30;
    
    println!("{d}");
}

fn improve_foo(s: &String) {
    println!("in fn foo: {s}");
}

fn improve_fn() {
    // 使用引用，改进函数 ownership3()
    let s1 = String::from("I am a superman.");
    // 注意这里传的是字符串的引用 &s1
    improve_foo(&s1);
    // 这里可以打印s1的值了
    println!("{s1}");
}

