// 结构体有三种形式，分别是命名结构体、元组结构体和单元结构体
/*
#[derive(Debug)]：在 Rust 中叫属性标注，具体来说这里用的是派生宏属性，派生宏作用在下面紧接着的结构体类型上，可以为结构体自动添加一些功能
比如，派生 Debug 这个宏可以让我们在 println! 中用 {:?} 格式把结构体打印出来，这对于调试是非常方便的。
这跟 Java 中的标注语法非常像，功能也是类似的，都会对原代码的元素产生作用
*/
fn main() {
    println!("-------------------------------------------------------------- 命名结构体");
    named_struct();
    println!("-------------------------------------------------------------- 元组结构体（匿名结构体）");
    tuple_struct();
    println!("-------------------------------------------------------------- 单元结构体");
    unit_struct();
    println!("-------------------------------------------------------------- 结构体所有权特性：部分移动（结构体中的部分字段是可以被移出去的）");
    ownership_partial_move();
    println!("-------------------------------------------------------------- 关联方法");
    association_method();
}


fn named_struct() {
    // 结构体的实例化
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user = User {
        active,    // 这里本来应该是 active: active,
        username,  // 这里本来应该是 username: username,
        email,     // 这里本来应该是 email: email,
        sign_in_count: 1,
    };
    println!("username: {}", user.username);

    // 单独更新了 email 字段
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // 想再创建一个新的 user2，而两个实例之间只有部分字段不同。这时，Rust 也提供了偷懒的办法
    // 这种写法可以帮助我们少写很多重复代码。特别是当这个结构体比较大的时候，比如有几十个字段，而我们只想更新其中的一两个字段的时候，就显得特别有用了
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1    // 注意这里，直接用 ..user1
    };
    println!("{:?}", user2);
}
// 用户信息，完全由 4 个基础类型的字段组合而成。User 的实例化需要这 4 个字段同时起作用，缺一不可
#[derive(Debug)]
struct User {
    // bool 类型，表示这个用户是否是激活状态
    active: bool,
    // 字符串类型，表示这个用户的名字
    username: String,
    // 字符串类型，表示这个用户的邮箱名
    email: String,
    // u64 类型，用来记录这个用户登录了多少次
    sign_in_count: u64,
}
// 班级信息，结构体类型也可以参与更复杂结构体的构建
struct Class {
    // 几班
    serial_number: u32,
    // 几年级
    grade_number: u32,
    // 起始年份
    entry_year: String,
    // User 的动态数组
    members: Vec<User>,
}


fn tuple_struct() {
    // 表示颜色RGB的值
    let black = Color(0, 0, 0);
    println!("black: {:?}", black);
    // 表示点位xyz
    let origin = Point(0, 0, 0);
    println!("origin: {:?}", origin);
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn unit_struct() {
    // 单元结构体就是只有一个类型名字，没有任何字段的结构体，它就相当于定义了一种类型，它的名字就是一种信息，有类型名就可以进行实例化，承载很多东西
    // 这也是做了实例化操作
    let module = ArticleModule;
    println!("module: {:?}", module);
}
#[derive(Debug)]
struct ArticleModule;


fn ownership_partial_move() {
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user3 = User3 {
        active,
        username,
        email,
        sign_in_count: 1,
    };

    // 这里发生了 partial_move
    let email = user3.email;
    println!("{}", email);
    // 这一句无法通过编译，提示如下
    // error[E0382]: borrow of partially moved value: `user3`
    // --> src\main.rs:100:22
    //     |
    //     97  |     let email = user3.email;
    // |                 ----------- value partially moved here
    //     ...
    //     100 |     println!("{:?}", user3);
    // |                      ^^^^^ value borrowed here after partial move
    //println!("{:?}", user3);


    // 因为 email 字段是 String 类型，是一种所有权类型，于是 email 字段的值被移动了。
    // 移动后，email 变量拥有了那个值的所有权。而 user1 中的 email 字段就被标记无法访问了
    // 稍微改一下这段代码，不直接打印 user1 实例整体，而是分别打印 email 之外的另外三个字段
    println!("{}", user3.username);
    println!("{}", user3.active);
    println!("{}", user3.sign_in_count);
}
#[derive(Debug)]
struct User3 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn association_method() {
    // Rectangle 类型实现了 area 方法，并在 Rectangle 的实例 rect 上使用点号（.）操作符调用了这个方法。
    let rect = Rectangle {
        width: 50,
        height: 10,
    };
    println!("{}", rect.area1());
    // self: Rust 的一个语法糖，self 的完整写法是 self: Self，而 Self 是 Rust 里一个特殊的类型名，它表示正在被实现（impl）的那个类型。
    // 和java中的this的概念是一样的，不过rust需要显示的写出来，而java隐藏的提供了
    // Rust 中所有权形式和借用形式总是成对出现，在 impl 的时候也是如此。方法的签名中也会对应三种参数形式

    // 实例的引用也是可以直接调用方法的。比如，对于不可变引用，可以像下面这样调用。Rust 会自动做正确的多级解引用操作。
    let rect1 = Rectangle {
        width: 50,
        height: 10,
    };
    // 在这里，取了实例的引用
    let r1 = &rect1;
    let r2 = &&rect1;
    // 不管有多少层
    let r3 = &&&&&&&&&&&&&&&&&&&&&&rect1;
    let r4 = &&r1;
    // 以下4行都能正确执行
    r1.area2();
    r2.area2();
    r3.area2();
    r4.area2();

    // 关联函数（静态方法）的定义和使用
    Rectangle::numbers(10, 10);

    // 构造函数
    Rectangle::new(20, 20);

    // 在对结构体做实例化的时候，Rust 又给我们提供了一个便利的设施，Default。
    // 需要在结构体上加一个Default派生宏
    // 方式一：
    let rt1: Rectangle = Default::default();
    // 方式二：
    let rt2 = Rectangle::default();

    println!("{:?}", rt1);
    println!("{:?}", rt2);

    // 创建默认初始化值的Rectangle实例
    let rectangle = crate::Rectangle::new(INIT_WIDTH, INIT_HEIGHT);
    println!("{:?}", rectangle);
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

const INIT_WIDTH: u32 = 100;
const INIT_HEIGHT: u32 = 100;

impl Rectangle {
    // 传入实例的所有权
    // self: Self
    fn area1(self) -> u32 {
        self.width * self.height
    }
    // 传入实例的不可变引用
    // self: &Self
    fn area2(&self) -> u32 {
        self.width * self.height
    }
    // 传入实例的可变引用
    // self: &mut Self
    fn area3(&mut self) -> u32 {
        self.width * self.height
    }
}
// 对同一个类型，impl 可以分开写多次。这在组织代码的时候比较方便。
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数（静态方法）
    // 如果它的第一个参数不是 self 参数，那么它就叫做此类型的关联函数
    // 关联函数使用类型配合路径符 :: 来调用
    // 与Java 里的静态方法起着类似的作用，但是 Rust 这里不需要额外引入一个 static 修饰符去定义，因为靠是否有 Self 参数就已经能明确地区分实例方法与关联函数了
    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }
    // 构造函数
    // Rust 中没有专门的构造函数，但是用于构造实例的需求是不会变的
    // Rust 中结构体可以直接实例化，基于这一点，Rust 社区一般约定使用 new() 这个名字的关联函数，像下面这样把类型的实例化包起来
    // new 这个名字并不是强制的。所以在社区的很多库里还会看到 from()、from_xxx() 等其他名字起构造函数的功能
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height,
        }
    }
}