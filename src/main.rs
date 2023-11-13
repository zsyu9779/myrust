use num::complex::Complex;

fn main() {
    //++++++++++++++++++++++++++++++++++++++++++++++++++++变量绑定与解构++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let a = "hello world!";
    println!("{}", a);

    // let b = 5; 创建的是不可变变量
    let mut b = 5;
    println!("{}", b);
    b = 6;
    println!("{}", b);

    let _x = 5;
    let _y = 10;

    let (aa, mut bb): (bool, bool) = (true, false);
    println!("aa = {:?}, bb = {:?}", aa, bb);
    bb = true;
    assert_eq!(aa, bb);

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    //===============变量遮蔽=====================
    let x = 5;
    //在main函数的作用域对x进行遮蔽
    let x = x + 1;

    {
        //在{}的作用域内对x进行遮蔽
        let x = x * 2;
        println!("the x in scope is {}", x)
    }
    println!("the x in main is {}", x);
    /*
        以上写法和mut变量不同点是 mut 可以修改在同一个内存地址上的值，不会发生内存对象的再分配，性能更好
        而变量遮蔽生成了完全不同的新变量，只是刚好同名。变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    */

    //++++++++++++++++++++++++++++++++++++++++++++++++++++基本类型++++++++++++++++++++++++++++++++++++++++++++++++++++++

    //===============数值类型=====================

    //整形溢出
    //let a :u8 = 256; //这种写法正常情况在编译时会报错: the literal `256` does not fit into the type `u8` whose range is `0..=255`
    let a: u8 = 255;
    /*
        使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
        如果使用 checked_* 方法时发生溢出，则返回 None 值
        使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
        使用 saturating_* 方法使值达到最小值或最大值
     */
    let b = a.wrapping_add(20);
    let c = a.wrapping_mul(2);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    //浮点类型
    //两种基本类型 f32 和 f64
    let x = 0.1; // f64
    let y: f32 = 0.02; // f32
    /*
        Rust 的浮点数类型遵循 IEEE-754 标准
        浮点数大部分情况下是不精确的，所以不要用 == 比较浮点数
    */
    assert_eq!(x + y == 0.12, false);
    /*
    thread 'main' panicked at 'assertion failed: `(left == right)`
    left: `0.120000005`,
    right: `0.12`', src/main.rs:69:5
    */

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert_eq!(abc.0 + abc.1, abc.2);
    // 由于f64精度高于f32 所以下面这个断言会panic
    //assert_eq!(xyz.0 + xyz.1, xyz.2);

    //========NaN=====
    //NaN 是一个特殊的浮点数，表示非数字（Not a Number）
    //NaN 与任何值都不相等，包括它自己
    let nan = (-42.0_f32).sqrt();
    assert_eq!(nan != nan, true);
    //判断一个值是否是nan 使用 is_nan 方法
    assert_eq!(nan.is_nan(), true);

    //数值运算 没什么特别
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 求余
    let remainder = 43 % 5;

    println!("sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}", sum, difference, product, quotient, remainder);

    //赋值方法
    //编译器会根据值自动推断变量类型
    let _a = 20;
    //显式指定类型
    let _a: i32 = 20;
    //后缀指定类型
    let _a = 20i32;
    let _a = 20_i32;
    //同样类型的变量才可以运算

    //对于较长的数字，可以使用下划线来提高可读性
    let a = 1_000_000;
    println!("a = {}", a);

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    //===============位运算=====================
    let a: i32 = 2; // 二进制 0000 0010
    let b: i32 = 3; // 二进制 0000 0011
    println!("a & b = {}", a & b); // 0000 0010
    println!("a | b = {}", a | b); // 0000 0011
    println!("a ^ b = {}", a ^ b); // 0000 0001
    println!("!a = {}", !a); // 1111 1101
    println!("a << 1 = {}", a << 1); // 0000 0100
    println!("a >> 1 = {}", a >> 1); // 0000 0001

    //===============序列=====================
    // rust提供了一种简介的方式 生成连续的数字序列
    // .. 生成一个左闭右开的区间
    for i in 0..5 {
        println!("i = {}", i);
    }
    // ..= 生成一个左闭右闭的区间
    for i in 5..=10 {
        println!("i = {}", i);
    }
    //序列只允许用于整数或字符类型 因为他们可以连续
    for c in 'a'..='z' {
        print!("{}  ", c);
    }
    //使用as完成类型转换
    for i in 0..5 {
        println!("i = {:.2}", i as f32);
    }

    //有理数和复数 有理数和复数未包含在标准库中，需要引入第三方库

    let a = Complex { re: 1.0, im: 2.0 };
    let b = Complex::new(3.0, 4.0);
    let result = a + b;
    println!("result = {}", result);

    //rust中的字符类型
    let c = '张'; //单个汉字
    let emoji = '😻'; //emoji表情
    let japanese = 'の'; //日文
    println!("c = {}, emoji = {}, japanese = {}", c, emoji, japanese);
    //rust中的字符类型是32位的，可以存储任意的unicode字符
    println!("字符'张'占用了{}个字节", std::mem::size_of_val(&c));

    //====================布尔类型====================
    let _b = true;
    let b: bool = false;
    if !b { println!("b is {}", b) }

    //====================单元====================
    //单元类型()，也被称为unit类型，它只有一个值，也就是()，它在函数没有返回值时使用
    //可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。

    //=======================语句和表达式=======================
    // 函数
    fn add_with_extra(a: i32, b: i32) -> i32 {
        // 语句会执行一些操作，但是不返回值
        let a = a + 1; //语句
        let b = b + 1; //语句
        // 表达式会返回一些值
        a + b //表达式
    }
    println!("add_with_extra(1, 2) = {}", add_with_extra(1, 2));
    //表达式不能包含分号 在表达式后加上分号就会变成一条语句 不会返回值
    //如果表达式不返回任何值，会隐式返回一个单元类型的值()
    // if语句块也可以是一个表达式 可以用于赋值 类似三元表达式
    let a = 1;
    let b = if a%2==1 { "odd" }else { "even" };
    println!("b = {}", b);

    //=======================函数=======================
    //函数的参数必须声明类型,函数名和参数名必须使用snake_case风格,
    fn add_two(a:i32,b:i32)->i32{
        a+b
    }
    println!("add_two(1,2) = {}",add_two(1,2));
    //函数也是表达式,可以把函数的表达式直接赋值给变量
    fn plus_five(a:i32)->i32{
        a+5
    }
    let f = plus_five(1);
    println!("f = {}",f);

    //发散函数 diverging function 一般用于panic 用!作函数返回类型的时候，表示这个函数永远不会返回
    fn _diverges()->!{
        panic!("This function never returns!");
    }
    //还有另一种发散函数，就是 loop 循环，它永远不会结束
    fn _diverges2()->!{
        loop{}
    }

    //++++++++++++++++++++++++++++++++++++++++++++++++++++++所有权和借用+++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //=======================所有权=======================
    //所有权是rust的一个核心概念，它是一种内存管理机制，它可以保证内存安全，同时也不需要垃圾回收机制
    //变量作用域 Rust中每个值都被一个变量所拥有，当变量离开作用域，这个值就被丢弃了
    {
        let s = "hello"; //s进入作用域
        println!("{}",s);
    }//s离开作用域，被丢弃

    let _s = "hello"; //字符串字面量 不可变 保存在栈上
    let mut s_string = String::from("hello"); //字符串对象 可变
    s_string.push_str(", world!"); //push_str() 在字符串后追加字面量
    println!("{}", s_string);

    //转移所有权
    let x = 5;
    let y = x; //x的值被复制到y，因为i32是基本类型，所以x和y都是5 这里没有发生所有权转移 因为i32是Copy类型(整数类型，固定大小的简单值 都被存在栈上)
    println!("x = {}, y = {}",x,y);

    let s1 = String::from("hello");
    let _s2 = s1;
    //s1的值被复制到s2，但是s1的值是指向堆上的内存地址，所以s1和s2都指向了同一块内存地址
    //这里发生了所有权转移，s1失效了，不能再使用,因为假设这里没有发生所有权转移 s1和s2都有意义，指向了同一块内存地址，
    //那么当s1和s2都离开作用域时，就会发生两次释放内存的操作，这就是所谓的double free，这是一种内存安全问题，rust编译器会阻止这种情况的发生
    //println!("s1 = {}, s2 = {}",s1,s2);
    /*
        //这里会报错，因为s1已经失效了
        255 |     let s1 = String::from("hello");
            |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        256 |     let s2 = s1;
            |              -- value moved here
        ...
        260 |     println!("s1 = {}, s2 = {}",s1,s2);
            |                                 ^^ value borrowed here after move
    */
    let x : &str = "hello";
    let y = x;
    //这里x只存储了对hello的引用，赋值给y的过程并没有发生所有权转移，因为&str是一个指向字符串字面量的引用，它是一个固定大小的值，存储在栈上
    println!("x = {}, y = {}",x,y);

    //=====深拷贝=======
    //Rust不会自动创建数据的深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 显式调用clone方法 完整拷贝s1在堆上的数据
    println!("s1 = {}, s2 = {}",s1,s2);
    //NOTICE clone会对所有数据进行深拷贝，这会消耗很多资源，所以在rust中，深拷贝是显式的，而不是隐式的

    //=====浅拷贝=======
    //浅拷贝只发生在栈上 可Copy的类型有：整数类型、浮点类型、布尔类型、字符类型、元组类型(其中的每个元素都是Copy类型)

    //=====所有权和函数=======
    //所有权也会发生在函数调用中 例如
    let s = String::from("hello");
    takes_ownership(s); //s的值被移动到函数里
    //println!("{}",s); //这里会报错，因为s已经失效了
    let x = 5;
    makes_copy(x); //x的值被复制到函数里
    println!("{}",x); //这里不会报错，因为x是Copy类型，所以x的值没有发生所有权转移，x仍然有效

    //=====返回值和作用域=======
    //返回值也会发生所有权转移

    let s1 = gives_ownership(); // gives_ownership 将返回值移动给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中,它也将返回值移动给 s3
    println!("s1 = {}, s3 = {}",s1,s3); //这里如果尝试打印s2会报错，因为s2已经失效了

}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
}// some_string 离开作用域，drop函数被调用，some_string的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
}// some_integer 离开作用域 不会有特殊操作

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // some_string 被返回并移出函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // a_string 被返回并移出函数
}

struct Struct {
    e: i32,
}
