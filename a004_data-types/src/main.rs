// ==================================================== 基础类型
// 数字类型：整数、整数字面量的辅助写法、浮点数（f32、f64）
// 布尔类型：true、false
// 字符：char、Unicode散列值
// 字符串：字符串字面量中的转义（反斜杠\）、禁止转义的字符串子面量（r""、r#""#）、字节串（b""）
// 数组：array类型，可存储同一类型的多个值，固定长度
// 动态数组：Vec向量，可存储同一类型的多个值，容量可变
// 哈希表：存储key-value映射关系

use std::vec;

// ==================================================== 复合类型
// 元祖：固定（元素）长度的列表
// 结构体：struct关键字，积类型
// 枚举：enum关键字，和类型
fn main() {
    println!("---------------------------------- simple_number");
    simple_number();
    println!("---------------------------------- simple_bool");
    simple_bool();
    println!("---------------------------------- simple_char");
    simple_char();
    println!("---------------------------------- simple_string");
    simple_string();
    println!("---------------------------------- simple_array");
    simple_array();
    println!("---------------------------------- simple_vector");
    simple_vector();
    println!("---------------------------------- simple_hashMap");
    simple_hash_map();
    println!("---------------------------------- simple_tup");
    simple_tup();
    println!("---------------------------------- simple_struct");
    simple_struct();
    println!("---------------------------------- simple_enum");
    simple_enum();
}


fn simple_number() {
    // i8 i16 i32 i64 i128 默认推导为i32
    // usize isize 由平台决定，根据操作系统自动分配空间

    // 进制的字面量
    println!("========= 进制的字面量");
    let a1 = -125;
    let a2 = 99_999;
    let a3 = 0xFF;
    let a4 = 0o13;
    let a5 = 0b10;
    let a6 = 0b1111_0000;
    println!("{a1}  {a2}  {a3}  {a4}  {a5}  {a6}");
    // Max Min
    println!("========= Max Min");
    println!("u32 max: {}", u32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("usize max: {}", usize::MAX);
    println!("isize is {} bytes", std::mem::size_of::<isize>());
    println!("usize is {} bytes", std::mem::size_of::<usize>());
    println!("u64 is {} bytes", std::mem::size_of::<u64>());
    println!("i64 is {} bytes", std::mem::size_of::<i64>());
    println!("i32 is {} bytes", std::mem::size_of::<i32>());
    // float （尽量使用f64，除非明确知道使用f32）
    println!("========= float");
    let f1: f32 = 1.23234;
    let f2: f64 = 9.8888;
    println!("Float are {:.2} {:.2}", f1, f2);
    // 冒号:用于引入格式化选项，这些选项可以控制输出的格式。在{:.2}中，冒号:后面的部分是格式化指令，它告诉Rust如何格式化紧随其后的参数
    // 具体来说，{:.2}中的.2表示小数点后保留两位数字。这里的冒号是必需的，因为它标志着格式化选项的开始。如果没有冒号，Rust将不会识别.2作为格式化指令，而是将其视为普通的文本
    // 冒号还可以用于其他格式化选项:
    // {:08}：将整数格式化为8位宽，不足的部分用0填充。
    // {:#010x}：将整数格式化为10位宽的十六进制数，不足的部分用0填充，并在前面加上0x。
    // {:>10}：将字符串右对齐，总宽度为10个字符。
    // {:<10}：将字符串左对齐，总宽度为10个字符。

    // 例如，{:08.2}可以用来格式化浮点数，使其总宽度为8位，小数点后保留两位，不足的部分用0填充。


}

fn simple_bool() {
    let is_ok = true;
    let can_ok: bool = false;
    println!("is ok? {is_ok} can ok? {can_ok}");
    println!(
        "is ok or can ok ?{}, can ok and is ok? {}",
        is_ok || can_ok,
        is_ok && can_ok
    );
}

fn simple_char() {
    // char：直接存的是Unicode Scalar Value
    let char_c = 'C';
    let emo_char = '☺';
    let chine_char = '中';
    println!("{char_c}  {emo_char}  {chine_char}");
    println!("{}", emo_char as usize);
    println!("{}", emo_char as i32);
    println!("{}", chine_char as i32);
}

fn simple_string() {
    // 字符串：String内部存储的是Unicode字符串的UTF8编码，也就是说String不是char的数组
    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello10 = String::from("Здравствуйте");
    let hello11 = String::from("Hola");
    println!("{hello1}  {hello2}  {hello3}  {hello4}  {hello5}  {hello6}  {hello7}  {hello8}  {hello9}  {hello10}  {hello11}");
    // 注意，Rust 中的 String 不能通过下标去访问。
    // 因为String存储的Unicode序列的UTF8编码，而UTF8编码是变长编码。这样即使能访问成功，也只能取出一个字符的UTF8编码的第一个字节，它很可能是没有意义的。因此Rust直接对String禁止了这个索引操作。
    // let a = hello8[0];

    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("将双引号进行转义：{}", byte_escape);
    
    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("分两行打印：{}", byte_escape);
    
    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("Windows下的换行符：{}", byte_escape);
    
    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("打印出反斜杠本身：{}", byte_escape);
    
    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("强行在字符串后面加个0：{}", byte_escape);

    // 使用 \x 输入等值的ASCII字符（最高7位）
    let byte_escape = "I'm saying hello \x7f";
    println!("输入等值的ASCII字符（最高7位）：{}", byte_escape);
    
    // 使用 \u{} 输入等值的Unicode字符（最高24位）
    let byte_escape = "I'm saying hello \u{0065}";
    println!("输入等值的Unicode字符（最高24位）：{}", byte_escape);

    // 字符串字面量前面加r，表示不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("字符串字面量前面加r：{}", raw_str);
    
    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("使用r##这种形式：{}", quotes);
    
    // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("加任意多的前后配对的#号：{}", longer_delimiter);

    // Rust中的字符串字面量都支持换行写，默认把换行符包含进去。
    let default_crlf_str = "Nice to meet
     you.
    ";
    println!("换行写：{}", default_crlf_str);

    // {:?} 用于调试输出，需要Debug trait，而{}用于常规输出，依赖于Display trait，数组类型默认并不实现Display trait
    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);
    
    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("字节串与原始字面量结合使用：{:?}", raw_bytestring);

}

fn simple_array() {
    // Rust数组是 array 类型，用于存储同一类型的多个值。数组表示成[T; N]，由中括号括起来，中间用分号隔开，分号前面表示类型，分号后面表示数组长度。
    // Rust数组是固定长度的，也就是说在编译阶段就能知道它占用的字节数，并且在运行阶段，不能改变它的长度（尺寸）
    // Rust中区分固定尺寸数组和动态数组，适应不同的场合，固定尺寸的数据类型是可以直接放栈上的，创建和回收都比在堆上动态分配的动态数组性能要好。
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{:?}", months);

    // 数组的访问，可以用下标索引。
    println!("a[1]获取的值为：{}", a[1]);
    println!("months[11]获取的值为：{}", months[11]);

    // 下标索引越界，无法通过编译：index out of bounds: the length is 5 but the index is 5
    //let b = a[5];
    //println!("a[5]的值为：{}", b);
}

fn simple_vector() {
    // Rust 中的动态数组类型是 Vec（Vector），也就是向量，中文翻译成动态数组。它用来存储同一类型的多个值，容量可在程序运行的过程中动态地扩大或缩小
    let v:Vec<i32> = Vec::new();
    println!("new初始化动态数组：{:?}", v);
    let v = vec![1, 2, 3];
    println!("vec初始化动态数组：{:?}", v);
    let mut v = Vec::new();
    println!("可变动态添加前：{:?}", v);
    v.push(5);
    v.push(6);
    v.push(7);
    println!("可变动态添加后：{:?}", v);

    // 动态数组可以用下标进行索引访问
    let s1 = String::from("superman 1");
    let s2 = String::from("superman 2");
    let s3 = String::from("superman 3");
    let s4 = String::from("superman 4");
    let s5 = String::from("superman 5");

    let v = vec![s1, s2, s3, s4, s5];
    println!("动态数组下标访问v[1]：{:?}", v[1]);

    // 下标索引越界，编译通过，运行期间报错：error: process didn't exit successfully: `target\debug\a004_data-types.exe` (exit code: 101)
    //println!("动态数组下标索引越界访问v[5]：{:?}", v[5]);

}

fn simple_hash_map() {
    // 哈希表能从一个键索引到一个值，所以应用场景非常广泛。具体的详细，查看HashMap工程模块。
    let mut colors = HashMap::new();
    colors.insert(String::from("Red"), 10);
    colors.insert(String::from("Green"), 10);
    colors.insert(String::from("Blue"), 50);
    println!("scores：{:?}", colors);

    
    // 哈希表是一种常见的结构，用于存储 Key-Value 映射关系，基本在各种语言中都有内置提供。
    // Rust 中的哈希表类型为 HashMap。对一个 HashMap 结构来说，Key 要求是同一种类型，比如是字符串就统一用字符串，是数字就统一用数字。Value 也是一样，要求是同一种类型。Key 和 Value 的类型不需要相同。

    // 要使用HashMap，必须先引入std::collections::HashMap模块（也可以在函数外层）
    use std::collections::HashMap;

    // 1、使用new函数创建一个新的、空的HashMap
    let mut colors = HashMap::new();
    colors.insert(String::from("Red"), 10);
    colors.insert(String::from("Green"), 10);
    colors.insert(String::from("Blue"), 50);
    println!("scores：{:?}", colors);

    // 2、新建带有元素的HashMap
    // 通过传入一个键值对的集合（比如：数组、切片或迭代器），我们可以在创建HashMap的同时初始化它。
    // 这可以通过collect方法来实现，它通常与vec!宏或数组字面量一起使用，以创建包含(key, value)元组的集合。
    // 首先创建一个HashMap，它的键是String类型，值是i32类型。
    // 然后，我们使用vec!宏创建了一个包含三个(key, value)元组的向量，并使用into_iter方法将其转换为迭代器。
    // 最后，我们使用collect方法将其收集到一个HashMap中。
    let colors:HashMap<String, i32> = vec![(String::from("Red"), 20), (String::from("Green"), 20), (String::from("Blue"), 60)].into_iter().collect();
    println!("scores：{:?}", colors);

    // HashMap::from是一个创建HashMap的便捷方法，主要用于从实现了IntoIterator特征且迭代器产出元组 (K, V) 的类型创建一个HashMap。
    let colors_init = [(String::from("Red"), 100), (String::from("Green"), 100), (String::from("Blue"), 200)];
    let colors = HashMap::from(colors_init);
    println!("scores：{:?}", colors);

    // 访问hashmap中的值，使用get方法或get_mut方法，具体取决于是否需要获取值的可变引用
    let mut colors = HashMap::new();
    colors.insert(String::from("Red"), 10);
    colors.insert(String::from("Green"), 10);
    colors.insert(String::from("Blue"), 50);
    // 访问值
    println!("red value is {:?}", colors.get("Red"));
    println!("green value is {:?}", colors.get("Green"));
    println!("blue value is {:?}", colors.get("Blue"));
    // 修改值
    if let Some(value) = colors.get_mut("Blue") {
        *value = 100;
    } else {
        println!("not found");
    }
    println!("修改后的值：{:?}", colors);

    // 根据键是否存在来执行不同的操作（比如：只在键不存在时插入值，或者在键存在时更新值），可以使用entry API，避免了不必要的查找
    // or_insert方法会在键不存在时插入给定的值，并返回键的值的可变引用
    colors.entry(String::from("Yellow")).or_insert(200);
    println!("or_insert方法，添加后的值：{:?}", colors);
    // and_modify方法会修改现有的值
    colors.entry(String::from("Yellow")).and_modify(|v| *v *= 2);
    println!("and_modify方法，修改后的值：{:?}", colors);
}


fn simple_tup() {
    // 元组是一个固定（元素）长度的列表，每个元素类型可以不一样。用小括号括起来，元素之间用逗号隔开
    let x = (100, 9.9, "hello rust");
    // 运算符访问其元素，下标从0开始，注意语法
    let a = x.0;
    let b = x.1;
    let c = x.2;
    println!("a={} b={} c={}", a, b, c);

    // 与数组的相同点是：它们都是固定元素个数的，在运行时不可伸缩。
    // 与数组的不同点是：元组的每个元素的类型可以不一样。
    
    // 元组在 Rust 中很有用，因为它可以用于函数的返回值，相当于把多个想返回的值捆绑在一起，一次性返回。


}

struct User {
    active: bool,
    username: String,
    email: String,
    age: u64,
}
fn simple_struct() {
    // Rust 中使用 struct 关键字来定义结构体
    let user = User {
        active: true,
        username: String::from("root"),
        email: String::from("123@qq.com"),
        age: 20,
    };
 
    // 结构体（struct）默认情况下不能直接打印，因为Rust是一种类型安全的语言，它要求在打印之前必须明确知道如何格式化输出，具体的详细，查看struct工程模块。
    println!("active：{}，username：{}，email：{}，age：{}", user.active, user.username, user.email, user.age);

}

enum IpAddrKind {
    V4,
    V6,
}
fn simple_enum() {
    // 枚举类型里面的选项叫做此枚举的变体（variants）。变体是其所属枚举类型的一部分。
    // 与结构体不同，结构体类型是里面的所有字段（所有类型）同时起作用，来产生一个具体的实例，而枚举类型是其中的一个变体起作用，来产生一个具体实例
    // 学术上，通常把枚举叫作和类型（sum type），把结构体叫作积类型（product type）
    // 枚举就像一个载体，可以携带任何类型。
    let _ip_four = IpAddrKind::V4;
    let _ip_six = IpAddrKind::V6;
    
}