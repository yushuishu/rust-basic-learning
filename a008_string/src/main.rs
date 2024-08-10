// Rust里的字符串内容相比于其他语言来说还要多一些，是否熟练掌握 Rust 的字符串的使用，对 Rust 代码开发效率有很大影响

//Rust里常见到的一些字符串相关的内容：
// String, &String, 
// str, &str, &'static str
// [u8], &[u8], &[u8; N], Vec<u8>
// as_str(), as_bytes()
// OsStr, OsString
// Path, PathBuf
// CStr, CString
fn main() {
    simple();
    other_type();
    convert_str_string();
}

fn simple() {
    // 静态数据区
    let s1: &'static str = "I am a superman.";

    // 将静态数据区中的字符串字面量拷贝了一份到堆内存中，通过 s2 指向，s2 具有这个堆内存字符串的所有权
    let s2: String = s1.to_string(); 

    // s3 就是对 s2 的不可变引用，因此类型为 &String
    let s3: &String = &s2;

    // s4 是对 s2 的切片引用，类型是 &str。切片就是一块连续内存的某种视图，它可以提取目标对象的全部或一部分。这里 s4 就是取的目标对象字符串的全部。
    let s4: &str = &s2[..];

    // s5 是对 s2 的另一个切片引用，类型也是 &str。与 s4 不同的是，s5 是 s2 的部分视图。具体来说，就是 "I am a" 这一部分
    let s5: &str = &s2[..6];
}

fn other_type() {
    // [u8] 是字节串切片，大小是可以动态变化的。
    // &[u8] 是对字节串切片的引用，即切片引用，与 &str 是类似的。
    // &[u8; N] 是对 u8 数组（其长度为 N）的引用。
    // Vec 是 u8 类型的动态数组。与 String 类似，这是一种具有所有权的类型。

    

}

fn convert_str_string() {
    
}
