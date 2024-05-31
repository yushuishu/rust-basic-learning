// 函数、模块
// 函数基本上是所有编程语言的标配，在 Rust 中也不例外，它是一种基本的代码复用方法。在 Rust 中使用 fn 关键字来定义一个函数
// 函数的调用要与函数的签名（函数名、参数个数、参数类型、参数顺序、返回类型）一致，也就是实参和形参要匹配。
// 函数对于几乎所有语言都非常重要，实际上各种编程语言在实现时，都是以函数作为基本单元来组织栈上的内存分配和回收的，这个基本的内存单元就是所谓的栈帧（frame）
mod garden;
fn main() {
    // 函数调用时传入的参数值叫做实际参数（实参）
    print_a_b(5, 'h');
    
    // 标准的函数定义
    fn _add_on_v1(x:u32) -> u32 {x + 1}
    // 闭包使用两个竖线符号 || 定义，而不是用 fn () 来定义；闭包只能在函数内部定义，不可以在函数外部定义；|| 内必须指定类型，不可以省略
    // 闭包的定义
    let _add_on_v2 = |x:u32| -> u32 {x + 1};
    // 闭包的定义，省略了类型标注；|| 内必须指定类型，不可以省略
    let _add_on_v3 = |x:u32| {x + 1};
    // 闭包的定义，花括号也省略了
    let _add_on_v4 = |x:u32| x + 1;

    // 闭包与函数的一个显著不同就是，闭包可以捕获函数中的局部变量为我所用，而函数不行。
    // 比如，闭包 add_v2 捕获了 main 函数中的局部变量 a 来使用，但是函数 add_v1 就没有这个能力。
    let a = 10u32;
    //fn add_v1(x) -> u32 {x + a} //（error[E0434]: can't capture dynamic environment in a fn item）
    let _add_v2 = |x| -> u32 {x + a};

    //let result1 = _add_v1(5); //调用函数
    let result2 = _add_v2(5); //调用闭包

    println!("{}", result2);

    
    // 模块目录结构
    let cabbage_info = garden::vegetables::get_chinese_cabbage_info();
    println!("蔬菜名：{:?}，单价/斤：{:?}，库存（斤）：{:?}，产地：{:?}", cabbage_info.0, cabbage_info.1, cabbage_info.2, cabbage_info.3);
    // 关于pub修饰：如果想在main.rs（或其他模块）中调用get_chinese_cabbage_info()函数，需要确保两件事情：
    // 1、vegetables模块是公开的，这样garden模块的外部代码可以看到它。
    // 2、get_chinese_cabbage_info()函数也是公开的，这样它可以从模块外部被调用。
    // 如果不使用pub修饰garden模块中的vegetables模块，那么vegetables模块将是私有的，只能在garden模块内部被访问。
    // 同样的，如果不使用pub修饰vegetables模块中的get_chinese_cabbage_info函数，那么这个函数将是私有的，只能在vegetables模块内部被访问。


    // Rust 语言中自带单元测试和集成测试方案
    // 在项目目录下运行 cargo test
}

// 标准的函数定义：函数定义时的参数叫作形式参数（形参）
fn print_a_b(a: i32, b: char) {
    println!("The value of a b is: {a}{b}");
}

fn foo() -> u32 { 10u32 }

// 这里配置测试模块
#[cfg(test)]
mod tests {
    use crate::foo;
    // 具体的单元测试用例
    #[test]
    fn it_works() {
        // 调用被测试的函数或功能
        let result = foo();
        // 断言
        assert_eq!(result, 10u32);
    }
}
