// ==================================================== 基础类型
// 数字类型：整数、整数字面量的辅助写法、浮点数（f32、f64）
// 布尔类型：true、false
// 字符：char、Unicode散列值
// 字符串：字符串字面量中的转义（反斜杠\）、禁止转义的字符串子面量（r""、r#""#）、字节串（b""）
// 数组：array类型，可存储同一类型的多个值，固定长度
// 动态数组：Vec向量，可存储同一类型的多个值，容量可变
// 哈希表：存储key-value映射关系

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
    simple_hashMap();
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


}

fn simple_vector() {}

fn simple_hashMap() {}