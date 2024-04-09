
const SECOND_HOURS:usize = 3_600;
const SECOND_DAY:usize = 24 * SECOND_HOURS;

static MY_STATIC:i32 = 50;
static mut MY_MUT_STATIC:i32 = 50;

fn main() {
    println!("SECOND_HOURS:{SECOND_HOURS}");
    println!("SECOND_DAY:{SECOND_DAY}");

    const INFO:&str = "hello world";
    println!("INFO:{INFO}");
    
    println!("MY_STATIC:{MY_STATIC}");

    // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
    // 可变静态可以被多个线程改变:违反别名或数据竞争将导致未定义的行为
    //println!("MY_MUT_STATIC:{MY_MUT_STATIC}");
    
    // 使用unsafe，来操作可变静态
    unsafe{
        println!("MY_MUT_STATIC:{MY_MUT_STATIC}");
        MY_MUT_STATIC = 100;
        println!("MY_MUT_STATIC:{MY_MUT_STATIC}");
    }
}
