// Rust里的字符串内容相比于其他语言来说还要多一些，是否熟练掌握 Rust 的字符串的使用，对 Rust 代码开发效率有很大影响

//Rust里常见到的一些字符串相关的内容：
// String, &String, 
// str, &str, &'static str
// [u8], &[u8], &[u8; N], Vec<u8>
// as_str(), as_bytes()
// OsStr, OsString
// Path, PathBuf
// CStr, CString
// -------------------- 总结
// 类型                                字符串                                字节串
//具有所有权                          String                                Vec<u8>
//切片                                str                                   [u8]
//切片引用                            &str                                  &[u8]
//所有权到切片引用      let a_slice: &str = &a_string[..];            let a_slice: &[u8] = &a_vec[..];
//切片引用到所有权      let a_string: String = a_slice.to_string();   let a_vec: Vec<u8> = a_slice.to_vec();
//                    let a_string: String = a_slice.to_owned();     let a_vec: Vec<u8> = a_slice.to_owned();
fn main() {
    println!("---------------------------------------------------------- 常用类型，字符串");
    simple();
    println!("---------------------------------------------------------- 字节串");
    vec_type();
    println!("---------------------------------------------------------- 字符串 装换");
    convert_str_string();
    println!("---------------------------------------------------------- 隐式引用类型转换");
    implicit_reference_type_conversion();
    println!("---------------------------------------------------------- 字节串 转 字符串");
    bytes_to_string();
    println!("---------------------------------------------------------- 字符串切割成字符数组");
    cut_string_to_char_array();
    println!("---------------------------------------------------------- parse()方法，字符串转换到任意 Rust 类型");
    parse_method();
}

// 常用类型，字符串
fn simple() {
    // 静态数据区
    let s1: &'static str = "I am a superman.";

    // 将静态数据区中的字符串字面量拷贝了一份到堆内存中，通过 s2 指向，s2 具有这个堆内存字符串的所有权
    let s2: String = s1.to_string(); 

    // s3 就是对 s2 的不可变引用，因此类型为 &String
    let s3: &String = &s2;
    println!("{s3}");

    // s4 是对 s2 的切片引用，类型是 &str。切片就是一块连续内存的某种视图，它可以提取目标对象的全部或一部分。这里 s4 就是取的目标对象字符串的全部。
    let s4: &str = &s2[..];
    println!("{s4}");

    // s5 是对 s2 的另一个切片引用，类型也是 &str。与 s4 不同的是，s5 是 s2 的部分视图。具体来说，就是 "I am a" 这一部分
    let s5: &str = &s2[..6];
    println!("{s5}");

    // 字符串切片引用，转换成所有权型字符串
    let s: &str = "a am a superman.";
    let s_1: String = String::from(s);
    let s_2: String = s.to_string();
    let s_3: String = s.to_owned();
    println!("方法一：{s_1} 方法二：{s_2} 方法三：{s_3}")
}

/// 字节串
fn vec_type() {
    // [u8] 是字节串切片，大小是可以动态变化的。
    // &[u8] 是对字节串切片的引用，即切片引用，与 &str 是类似的。
    // &[u8; N] 是对 u8 数组（其长度为 N）的引用。
    // Vec 是 u8 类型的动态数组。与 String 类似，这是一种具有所有权的类型。
    // Vec 与 &[u8] 的关系如下：
    let a_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // a_slice 是 [1, 2, 3, 4, 5]
    let a_slice: &[u8] = &a_vec[0..5];
    println!("{:?}", a_slice);
    // 用 .to_vec() 方法将切片转换成Vec
    let another_vec: Vec<u8> = a_slice.to_vec();
    println!("{:?}", another_vec);
    // 或者使用 .to_owned() 方法
    let another_vec: Vec<u8> = a_slice.to_owned();
    println!("{:?}", another_vec);

}

// 字符串 装换
fn convert_str_string() {
    // as_str()、as_bytes()、as_slice()

    // String 类型上有个方法是 as_str()。它返回 &str 类型。这个方法效果其实等价于 &a_string[..]，也就是包含完整的字符串内容的切片。
    let s: String = String::from("foo");
    assert_eq!("foo", s.as_str());

    // String 类型上还有个方法是 as_bytes()，它返回 &[u8] 类型。
    let s: String = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    // &str 也可以转成 &[u8]
    let bytes: &[u8] = "bors".as_bytes();
    assert_eq!(b"bors", bytes);

    // Vec 上有个 as_slice() 函数，与 String 上的 as_str() 对应，把完整内容转换成切片引用 &[T]，等价于 &a_vec[..]
    let a_vec = vec![1, 2, 3, 5, 8];
    assert_eq!(&[1, 2, 3, 5, 8], a_vec.as_slice());

}

// 隐式引用类型转换
// 源                目标
// &String           &str
// &Vec<u8>          &[u8]
// &Vec<T>           &[T]
fn implicit_reference_type_conversion() {
    let s = String::from("I am a superman.");
    foo(&s);

    // 函数的参数类型我们定义成 &String。那么在函数调用时，这个函数只接受 &String 类型的参数传入。如果我们定义一个字符串字面量变量，想传进 foo 函数中，就发现不行
    // 提示报错信息：mismmatched types expected reference '&String' found reference '&str'
    let s1 = "I am a superman.";
    println!("{s1}");
    // foo(s1);

    // 把 foo 参数的类型由 &String 改为 &str，就编译通过
    foo2(&s);
    foo2(s1);

    // 在 Rust 中对 String 做引用操作时，可以告诉 Rust 编译器，想把 &String 直接转换到 &str 类型。只需要在代码中明确指定目标类型就可以了
    // 同样的原理，不仅可以作用在 String 上，也可以作用在 Vec 上 ，更进一步的话，还可以作用在 Vec 上
    let s = String::from("I am a superman.");
    let s1 = &s;
    let s2: &str = &s;
    println!("s1:{s1} s2:{s2}");

    let v: Vec<u32> = vec![1,2,3,4,5];
    foo3(&v);
    let a_slice = v.as_slice();
    foo3(a_slice);

}

fn foo(s: &String) {
    println!("{s}");
}
fn foo2(s: &str) {
    println!("{s}");
}
fn foo3(s: &[u32]) {
    println!("{:?}", s);
}

// 字节串 转 字符串
fn bytes_to_string() {
    // 通过 as_bytes() 方法将字符串转换成 &[u8]。相反的操作也是有的，就是把 &[u8] 转换成字符串
    // Rust 中的字符串实际是一个 UTF-8 序列，因此转换的过程也是与 UTF-8 编码相关的

    // String::from_utf8() 可以把 Vec 转换成 String，转换不一定成功，因为一个字节序列不一定是有效的 UTF-8 编码序列。它返回的是 Result（关于 Result，我们后面会专题讲解，这里仅做了解），需要自行做错误处理
    // String::from_utf8_unchecked() 可以把 Vec 转换成 String。不检查字节序列是不是无效的 UTF-8 编码，直接返回 String 类型。但是这个函数是 unsafe 的，一般不推荐使用
    // str::from_utf8() 可以将 &[u8] 转换成 &str。它返回的是 Result，需要自行做错误处理
    // str::from_utf8_unchecked() 可以把 &[u8] 转换成 &str。它直接返回 &str 类型。但是这个函数是 unsafe 的，一般不推荐使用

    let s_vec: Vec<u8> = vec![104, 101, 108, 108, 111];

    match String::from_utf8(s_vec) {
        Ok(s) => println!("转成成功：{}", s),
        Err(e) => println!("转换失败：{}", e),
    }

}

// 字符串切割成字符数组
fn cut_string_to_char_array() {
    // &str 类型有个 chars() 函数，可以用来把字符串转换为一个迭代器，迭代器是一种通用的抽象，就是用来按顺序安全迭代的，我们后面也会讲到这个概念。通过这个迭代器，就可以取出 char
    let s = String::from("中国你好");                                                                                 
    let char_vec: Vec<char> = s.chars().collect();                                                                     
    println!("{:?}", char_vec); 
    for ch in s.chars() {
        println!("{:?}", ch); 
    }
}

// 字符串转换到任意 Rust 类型
fn parse_method() {
    // parse() 方法非常强大，可以从字符串转换到任意 Rust 类型，只要这个类型实现了 FromStr 这个 Trait（Trait 是 Rust 中一个极其重要的概念，后面我们会讲述）即可。把字符串解析成 Rust 类型，肯定有不成功的可能，所以这个方法返回的是一个 Result
    // parse() 函数就相当于 Rust 语言内置的统一的解析器接口，如果你自己实现的类型需要与字符串互相转换，就可以考虑实现这个接口，这样的话就比较能被整个 Rust 社区接受，这就是所谓的 Rust 地道风格的体现

    let a = "10".parse::<u32>();    
    println!("{:?}", a);
    
    let a = "10".parse::<f32>();
    println!("{:?}", a);
    
    let a = "4.2".parse::<f32>();
    println!("{:?}", a);
    
    let a = "true".parse::<bool>();
    println!("{:?}", a);

    let a = "a".parse::<char>();
    println!("{:?}", a);
    
    let a = "192.168.1.100".parse::<std::net::IpAddr>();
    println!("{:?}", a);

}
