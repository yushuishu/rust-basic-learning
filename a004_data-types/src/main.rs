fn main() {
    println!("---------------------------------- simple()");
    simple();


}


fn simple() {
    // i8 i16 i32 i64 i128 默认推导为i32
    // usize isize 由平台决定，根据操作系统自动分配空间

    // 进制的字面量
    let a1 = -125;
    let a2 = 0xFF;
    let a3 = 0o13;
    let a4 = 0b10;
    println!("{a1}  {a2}  {a3}  {a4}");
    // Max Min
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
    let f1: f32 = 1.23234;
    let f2: f64 = 9.8888;
    println!("Float are {:.2} {:.2}", f1, f2);

    // bool
    let is_ok = true;
    let can_ok:bool = false;
    println!("is ok? {is_ok} can ok? {can_ok}");
    println!("is ok or can ok ?{}, can ok and is ok? {}", is_ok || can_ok, is_ok && can_ok);

    // char
    let char_c = 'C';
    let emo_char = '☺';
    println!("You Get {char_c} feel {emo_char}");
    println!("{}", emo_char as usize);
    println!("{}", emo_char as i32);
}
