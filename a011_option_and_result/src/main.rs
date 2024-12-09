/*
Option<T> 定义为包含两个变体的枚举。一个是不带负载的 None，另一个是带一个类型参数作为其负载的 Some。
Option 的实例在 Some 和 None 中取值， 表示这个实例有取空值的可能。
在没有 Option 的语言中，空值是分散在其他类型中的。比如空字符串、空数组、数字 0、NULL 指针等。并且有的语言还把空值区分为空值和未定义的值，如 nil、undefined 等。
Rust 做了两件事情来解决这个混乱的场面：
    第一，Rust 中所有的变量定义后使用前都必须初始化，所以不存在未定义值这个情况。
    第二，Rust 把空值单独提出来统一定义成 Option::None，并在标准库层面上就做好了规范，上层的应用在设计时也应该遵循这个规范。

Result<T, E> 被定义为包含两个变体的枚举。这两个变体各自带一个类型参数作为其负载。Ok(T) 用来表示结果正确，Err(E) 用来表示结果有错误。
对比其他语言函数错误返回的约定，C、CPP、Java 语言里有时用返回 0 来表示函数执行正确，有时又不是这样，你需要根据代码所在的上下文环境来判断返回什么值代表正确，返回什么值代表错误。
 */
fn main() {
    // 解包expect()：消解外面的包裹层，取出里面的值，带一个错误提示信息。如果Option<T>实例为None，就会panic，并打出错误提示信息；Result<T, E>为Err，也会panic，并打出错误提示信息
    test_expect();
    // 解包unwrap()：消解外面的包裹层，取出里面的值。如果Option<T>实例为None，就会panic，与expect()方法不同的地方在于panic时，unwrap()不带提示信息；Result<T, E>为Err，也会panic，不带提示信息。
    test_unwrap();
    // unwrap_or()：消解外面的包裹层，取出里面的值，如果Option<T>实例为None，不会panic，而是取这个函数提供的参考值；Result<T, E>为Err，也不会panic，也是取函数提供的参考值
    test_unwrap_or();
    // unwrap_or_default()：消解外面的包裹层，取出里面的值，如果Option<T>实例为None，不会panic，而是取里面被包裹类型的默认值；Result<T, E>为Err，也不会panic，也是取里面被包裹类型的默认值
    test_unwrap_or_default();

    // 不解包的情况下如果想要获取被包在里面的值就需要用到 Option 和 Result 里的一些常用方法。
    // Option方法：map()、cloned()、is_some()、is_none()、as_ref()、as_mut()、take()、replace()、and_then()
    // src/option_common_api/**

    // Result方法：map()、is_ok()、is_err()、as_ref()、as_mut()、and_then()、map_err()
    // src/result_common_api/**
}

fn test_expect() {
    // Option
    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    // Result
    let path = std::env::var("IMPORTANT_PATH").expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    println!("path: {}", path);
}

fn test_unwrap() {
    // Option
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");
    // Result
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);
}

fn test_unwrap_or() {
    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");
    // Result
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);
    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);
}


fn test_unwrap_or_default() {
    // Option
    let x: Option<u32> = None;
    let y: Option<u32> = Some(12);
    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 12);
    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().unwrap_or_default();
    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);
}