use std::fmt::{Debug, Display};
use std::ops::Index;
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
    let b = if a % 2 == 1 { "odd" } else { "even" };
    println!("b = {}", b);

    //=======================函数=======================
    //函数的参数必须声明类型,函数名和参数名必须使用snake_case风格,
    fn add_two(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("add_two(1,2) = {}", add_two(1, 2));
    //函数也是表达式,可以把函数的表达式直接赋值给变量
    fn plus_five(a: i32) -> i32 {
        a + 5
    }
    let f = plus_five(1);
    println!("f = {}", f);

    //发散函数 diverging function 一般用于panic 用!作函数返回类型的时候，表示这个函数永远不会返回
    fn _diverges() -> ! {
        panic!("This function never returns!");
    }
    //还有另一种发散函数，就是 loop 循环，它永远不会结束
    fn _diverges2() -> ! {
        loop {}
    }

    //++++++++++++++++++++++++++++++++++++++++++++++++++++++所有权和借用+++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //=======================所有权=======================
    //所有权是rust的一个核心概念，它是一种内存管理机制，它可以保证内存安全，同时也不需要垃圾回收机制
    //变量作用域 Rust中每个值都被一个变量所拥有，当变量离开作用域，这个值就被丢弃了
    {
        let s = "hello"; //s进入作用域
        println!("{}", s);
    }//s离开作用域，被丢弃

    let _s = "hello"; //字符串字面量 不可变 保存在栈上
    let mut s_string = String::from("hello"); //字符串对象 可变
    s_string.push_str(", world!"); //push_str() 在字符串后追加字面量
    println!("{}", s_string);

    //转移所有权
    let x = 5;
    let y = x; //x的值被复制到y，因为i32是基本类型，所以x和y都是5 这里没有发生所有权转移 因为i32是Copy类型(整数类型，固定大小的简单值 都被存在栈上)
    println!("x = {}, y = {}", x, y);

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
    let x: &str = "hello";
    let y = x;
    //这里x只存储了对hello的引用，赋值给y的过程并没有发生所有权转移，因为&str是一个指向字符串字面量的引用，它是一个固定大小的值，存储在栈上
    println!("x = {}, y = {}", x, y);

    //=====深拷贝=======
    //Rust不会自动创建数据的深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 显式调用clone方法 完整拷贝s1在堆上的数据
    println!("s1 = {}, s2 = {}", s1, s2);
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
    println!("{}", x); //这里不会报错，因为x是Copy类型，所以x的值没有发生所有权转移，x仍然有效

    //=====返回值和作用域=======
    //返回值也会发生所有权转移

    let s1 = gives_ownership(); // gives_ownership 将返回值移动给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中,它也将返回值移动给 s3
    println!("s1 = {}, s3 = {}", s1, s3); //这里如果尝试打印s2会报错，因为s2已经失效了

    //=====引用和借用=======

    // 引用和解引用
    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, *y); //解引用
    //引用和解引用是一对互逆操作，引用是指向某个值的指针，解引用是获取指针指向的值
    //不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //传递s1的引用 但是calculate_length函数并不拥有s1的所有权
    println!("The length of '{}' is {}.", s1, len); //所以这里打印s1是没有问题的
    //可变引用 尝试修改引用变量
    let mut s2 = String::from("hello");
    change(&mut s2); //传递s2的可变引用
    println!("s2 = {}", s2);
    //可变引用有一个很大的限制，就是在特定作用域中的特定数据只能有一个可变引用，这样做的目的是为了防止数据竞争
    //数据竞争是指两个或更多指针同时访问同一块数据，至少有一个指针用于写入数据，且没有同步数据访问的机制
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("r1 = {}", r1); //这里打印r1 下面的借用就不会触发borrow checker 即在第一次借用到最后一次使用之间，不能有任何修改借用变量的操作
    let r2 = &mut s; // 如果上面的打印语句不执行，这里会报错，因为s已经被r1借用了，且r1没有完成最后一次使用,所以不能再被r2借用 在编译期就避免了数据竞争
    //println!("r1 = {}, r2 = {}",r1,r2);
    println!("r2 = {}", r2);
    //println!("r1 = {}",r1);

    //NOTICE {}占位符会自动解引用 可以帮我们解决一些编译错误
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 在这里离开了作用域 所以我们可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 = {}", r2);

    //可变引用和不可变引用不能同时存在
    let mut _s = String::from("hello");
    let r1 = &s; //不可变引用
    let r2 = &s; //不可变引用
    println!("r1 = {}, r2 = {}", r1, r2); //这里加一句 下面的可变引用就不会报错
    //let r3 = &mut s; //可变引用
    //println!("{}, {}, and {}", r1, r2, r3); //这里会报错，因为r1和r2是不可变引用，r3是可变引用，可变引用和不可变引用不能同时存在
    assert_eq!(r1, r2); //这里可以使用断言，因为r1和r2是不可变引用，所以不会发生数据竞争

    //NOTICE!!! 引用的作用域是从声明开始一直持续到最后一次使用为止 和变量有所不同 变量的作用域是从声明开始一直持续到当前作用域结束为止(即到达'}'为止)

    //悬垂引用
    //悬垂引用是指指向了已经被释放的内存的指针，rust编译器会阻止这种情况的发生 即在引用结束前不允许变量被释放
    //let reference_to_nothing = dangle(); //这里会报错，因为dangle函数返回的是一个指向堆上数据的引用，但是dangle函数结束后，这个数据就被释放了，所以这里会报错

    //==================================================================复合类型==================================================================
    //=======================字符串与切片=======================
    //字符串字面量
    let s = "hello world";
    //字符串对象
    let _s1 = String::from("hello world");
    //字符串切片
    let hello = &s[0..5]; //btw 这种写法叫序列 详见 main.rs: 154
    let world = &s[6..11];
    println!("hello = {}, world = {}", hello, world);
    //类似golang切片操作 从索引0开始截取和截取到最后一个元素的写法分别如下
    let hello = &s[..5];
    let world = &s[6..];
    println!("hello = {}, world = {}", hello, world);
    //NOTICE 对于字符串使用切片语法时 切片的索引必须是有效的字符索引 如果使用无效的字符索引会导致panic 比如汉字在UTF-8编码中占3个字节 截取&s[2..]会panic
    let _s = "你好世界";
    //let cut = &s[2..]; //这句报错：panicked at 'byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好世界`'
    //println!("cut = {}",cut);
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word = {}", word); //这句打开 下面一句就不会报错 因为不可变借用已经使用过了
    //s.clear(); //这句的参数是对自身的可变借用 然而word是对s的不可变借用 所以这里会报错 参见main.rs: 337

    //=========其它切片========
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    //=================字符串=================
    //Rust中的字符是Unicode类型的 每个字符占据4个字节，但字符串是UTF-8编码，所以一个字符串中的每个字符所占的字节数是变化的（1-4）
    //Rust语言级别的字符串类型就是字符串字面量，它是一个不可变的字符串切片，它的类型是&str 在标准库里还有一个String类型，它是一个可变的字符串对象
    //str是硬编码进可执行文件的，无法被修改 String是一个可变且具有所有权的UTF-8编码字符串

    //String和&str转换 TODO deref 隐式强制转换
    let s = String::from("hello world");
    let s = &s[..]; //转换为字符串切片
    let s = s.to_string(); //转换为字符串对象
    let s = s.as_str(); //转换为字符串切片
    println!("s = {}", s);
    //字符串索引
    let s = String::from("hello");
    //let h = s[0]; //这里会报错 `String` cannot be indexed by `{integer}`
    /*
        字符串源码：
        pub struct String {
            vec: Vec<u8>,
        }
        底层实际上是一个Vec<u8>，不同类型的文字在UTF-8下长度不同 所以通过索引取没有意义 且如果要保证索引读O(1)的时间复杂度
        又不允许遍历到合法字符的结尾，所以干脆禁止这样做
    */
    let h = s.index(0..1);
    println!("h = {}", h);

    //字符串操作
    let mut s = String::from("hello"); //可操作的字符串必须是可变的
    //追加
    s.push_str(" world"); //追加字符串
    s.push('!');//追加字符
    println!("s = {}", s);
    //插入
    s.insert_str(0, "hello Rust"); //在索引0处插入字符串
    s.insert(10, '!'); //在索引10处插入字符
    println!("s = {}", s);
    //替换
    let mut s1 = s.replace("rust", "RUST");//适用于String和&str 返回的是一个新字符串 原字符串不必mut
    let mut s2 = s1.replacen("RUST", "rust", 1); //适用于String和&str 返回的是一个新字符串 原字符串不必mut 只替换第一个目标值
    s2.replace_range(0..5, "HELLO"); //适用于String 直接操作原字符串 必须mut
    println!("s = {:?}", s2);
    //删除
    let p1 = s.pop();//删除最后一个字符并返回
    println!("p1 = {:?}", p1);
    let mut s = String::from("你好你好");
    s.remove(0);//删除第一个字符 // remove的参数如果不是合法字符的边界会报错
    println!("s = {}", s);
    //s.remove(2); //panic 索引2是非法边界
    s.truncate(3);//删除索引3之后的所有字符
    println!("s = {}", s);
    s.clear();//清空字符串
    dbg!(s);
    //连接
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; //s2会自动解引用为&str类型 s1的所有权被转移 不能再打印s1 ‘+’是add() s1的所有权被转移到add()里了
    let mut s3 = s3 + "!";
    s3 += "!";
    println!("s3 = {}", s3);
    // !format方式
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = format!("{}{}{}", s1, s2, "!"); //format!宏会返回一个String对象
    println!("s3 = {}", s3);
    //字符串转义：可以通过'\'输出 ASCII 和 Unicode 字符
    let s = "hello world \x52\x75\x73\x74\n";
    println!("s = {}", s);
    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 原始字符串 不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    //=======================================元组=======================================
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //可以用模式匹配或者.索引的方式获取元组的值
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
    //元组可以作为函数的参数和返回值
    let s1 = String::from("hello");
    let (s2, len) = calculate_length2(s1);
    println!("s2 = {}, len = {}", s2, len);

    //=======================================结构体=======================================
    /*
        1.结构体是一种自定义数据类型，它允许你命名和包装多个相关的值，从而形成一个更加有意义的组合
        2.结构体的每个值都有自己的类型，这些类型被称为结构体的字段，并且可以独立命名和指定类型
        3.初始化实例时，每个字段都必须有值，否则编译器会报错
        4.顺序不必和定义时一致
    */
    let mut u1 = User {
        username: String::from("张三"),
        email: String::from("xxx@example.com"),
        sign_in_count: 1,
        active: true,
    };
    u1.username = String::from("李四"); // 需要将结构体实例定义为可变，才能修改结构体的字段 Rust不支持将结构体的某个字段设置为可变或不可变
    println!("u1.username = {}", u1.username);
    let u2 = build_user(String::from("aaa@example.com"), String::from("王五"));
    println!("u2.username = {}", u2.username);
    // 利用已有的user1 创建user3
    let u3 = User {
        email: String::from("xxx@example.com"),
        ..u1
    }; // u3和u1只有email字段不同 所以可以用 ..u1 语法来创建u3 ..u1这种写法必须在结构体尾部使用
    //username字段发生了所有权转移
    println!("u3.username = {}", u3.username);

    //元组结构体 结构体需要名称，但是结构体字段不一定需要
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black = ({},{},{})", black.0, black.1, black.2);
    println!("origin = ({},{},{})", origin.0, origin.1, origin.2);

    //单元结构体
    struct Unit;
    let _unit = Unit;

    //=======================枚举=======================

    //枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例
    let four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    print_IpAddrKind(&four);

    let info = IpInfo {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    print_IpInfo(info);
    // 枚举成员可以包含数据 见PokerCard
    let clubs = PokerCard::Clubs(1);
    let spades = PokerCard::Spades(2);
    let diamonds = PokerCard::Diamonds(3);
    let hearts = PokerCard::Hearts(4);
    print_PokerCards(&[clubs, spades, diamonds, hearts]);

    //任何类型的数据都可以作为枚举成员 例如字符串、数值甚至结构体
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(1, 2, 3);
    // 上述的四个枚举成员都有不同的类型 其实可以用struct来实现 用枚举的好处是可以将所有的类型都放在一个命名空间下
    // 结合上述场景 如果一个函数的功能是接收并转发Message 但是它不关心消息类型 如果用结构体的话 这个函数无法接受4个类型的结构体参数
    send_Msg(m1);
    send_Msg(m2);
    send_Msg(m3);
    send_Msg(m4);
    //Option枚举 用于处理空值
    /*
        enum Option<T> {
            Some(T),
            None,
        }
    */
    //Option<T>是一个泛型枚举，它有两个成员，Some(T)和None，Some(T)表示一个包含了某个类型的值的Option枚举，
    // None表示一个空值
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number = {:?}, some_string = {:?}, absent_number = {:?}", some_number, some_string, absent_number);
    // 使用Option<T>的好处是当我们想要使用T的时候 就必须处理None的情况 这样可以避免空指针异常 确保我们对有可能的空值进行处理
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}, six = {:?}, none = {:?}", five, six, none);

    //切片 与数组不同，切片的长度在编译期是不确定的，所以切片是一个动态的类型
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
    //创建切片的代价很小，它只是对原数组的引用，所以切片的复制代价也很小

    //========================================流程控制========================================
    //=======================if表达式=======================
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number = {}", number); //每个分支里的返回值类型必须一致
    //=======================循环=======================
    for i in 1..=5 {
        println!("i = {}", i);
    }
    //遍历集合类型的时候 往往要使用集合的引用形式 如果不使用引用形式 这个集合的所有权会被转移到for语句中 然后随着语句执行完成释放 后面没法再用这个集合了
    let mut a = [1, 2, 3, 4, 5];
    for i in &a {
        println!("i = {}", i);
    }
    // 在for中修改元素
    for j in &mut a {
        *j += 1;
    }
    println!("a = {:?}", a);
    // 在循环中获取索引
    for (i, j) in a.iter().enumerate() {
        println!("i = {}, j = {}", i, j);
    }
    //continue
    for i in 1..5 {
        if i == 2 {
            continue;
        } else if i == 4 { break; }
        println!("i = {}", i);
    }
    //while循环
    let mut i = 1;
    while i <= 5 {
        println!("i = {}", i);
        i += 1;
    }
    //loop循环 配合if和break实现条件循环
    let mut i = 1;
    loop {
        if i > 5 { break; }
        println!("i = {}", i);
        i += 1;
    }
    //NOTICE 使用迭代器遍历数组（切片）性能会更好 因为避免了每次的运行时索引边界检查

    // break可以单独使用 也可以带一个返回值

    //==============================模式匹配==============================
    //match表达式 类似golang的switch
    let number = 13;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        13 => println!("thirteen"),
        _ => println!("others"),
    }
    //match表达式的每个分支都是一个模式，模式可以是字面量、变量、通配符、占位符、枚举、结构体、元组、范围、引用、切片、迭代器、守卫
    //模式匹配是穷尽的，如果没有匹配到任何分支，编译器会报错
    //match本身是一个表达式，它的返回值是每个分支的返回值的公共类型
    let ip = IpAddrKind::V4;
    let ip_info = match ip {
        IpAddrKind::V4 => IpInfo {
            kind: IpAddrKind::V4,
            address: String::from("localhost"),
        },
        IpAddrKind::V6 => IpInfo {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        }
    };
    print_IpInfo(ip_info);

    //match表达式的分支可以使用模式绑定
    let actions = [
        Action::Say(String::from("hello")),
        Action::MoveTo(1, 2),
        Action::ChangeColor(255, 255, 0),
    ];
    //这里的action是一个模式绑定，它会将actions数组中的每个元素绑定到action上，然后执行对应的分支
    for action in &actions {
        match action {
            Action::Say(s) => {
                println!("say {}", s);
            }
            Action::MoveTo(x, y) => {
                println!("move to ({},{})", x, y);
            }
            Action::ChangeColor(r, g, b) => {
                println!("change color to ({},{},{})", r, g, b);
            }
        }
    }
    //模式绑定的另一种写法 if let ；只要匹配一个条件 且忽略其他条件时用if let
    if let Action::Say(s) = &actions[0] {
        println!("if let say {}", s);
    }

    //matches!宏 可以用来简化match表达式
    let v = vec![IpAddrKind::V4, IpAddrKind::V6];
    v.iter().filter(|x| matches!(x,IpAddrKind::V4)).for_each(|x| print_IpAddrKind(x));

    //变量遮蔽 无论是match还是if let 都会创建一个新的变量绑定，这个变量绑定会遮蔽外部的同名变量
    let x = Some(5);
    println!("before x = {:?}", x);//Some(5)
    if let Some(x) = x {
        println!("x被变量遮蔽 if let x = {:?}", x); //5
    }
    println!("after x = {:?}", x); //Some(5)

    //Option详解
    //使用Option<T> 是为了从Some中取出其内部的T值 以及处理没有值的情况
    let five = Some(5);
    let seven = plus_two(five);
    let none = plus_two(None);
    println!("five = {:?}, seven = {:?}, none = {:?}", five, seven, none);
    // plus_two接收一个Option<i32类型的参数 返回一个Option<i32>类型的值
    // 在该函数的内部处理中如果传入的是一个None 则返回一个None且不做任何处理 如果传入的是一个Some(i32) 则返回一个Some且对Some内部的值加2

    //除了上述提到的if let模式匹配 也可以使用match模式匹配
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("top = {}", top);
    }
    //这里的while let循环会一直执行，直到stack.pop()返回None，即stack为空

    //单分支多模式
    let x = 1;
    match x {
        1 | 2 => {
            println!("x = 1 or 2");
        }
        _ => {
            println!("x = others");
        }
    }
    //通过序列..=来匹配范围
    let x = 5;
    match x {
        1..=5 => {
            println!("x = 1..=5");
        }
        _ => {
            println!("x = others");
        }
    }
    //解构并分解值：可以使用模式来解构结构体、枚举、元组、引用、切片、迭代器
    //解构结构体
    let user = User {
        username: String::from("张三"),
        email: String::from("zhangsan@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    // let User { username:a, email:b, sign_in_count:c, active:d } = user;
    // //将结构体的字段解构到变量中
    // println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    //简写
    let User { username, email, sign_in_count, active } = user;
    println!("username = {}, email = {}, sign_in_count = {}, active = {}", username, email, sign_in_count, active);

    //匹配固定某个字段的方式
    struct Point1 {
        x: i32,
        y: i32,
    }
    let p = Point1 { x: 0, y: 2 };
    match p {
        Point1 { x, y: 0 } => println!("x轴上,横坐标 = {}", x),
        Point1 { x: 0, y } => println!("y轴上,纵坐标 = {}", y),
        Point1 { x, y } => println!("x = {}, y = {}", x, y),
    }

    //==================================================================方法==================================================================

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        //new是Circle的关联函数，因为它的第一个参数不是self，且new不是关键字 这种方法往往用于初始化当前结构体实例
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle { x, y, radius }
        }
        // Circle的方法 &self表示借用当前Circle结构体
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // self &self &mut self
    // 在area方法中，我们使用了&self，其实是self：&Self的简写 ，这是因为我们不想获取Circle的所有权，只是想借用它，这样Circle就不会被销毁
    // 而self表示获取Circle的所有权，这样Circle就会被销毁，这种情况很少见 &mut self表示获取Circle的可变借用
    //关联函数和方法的区别 关联函数是属于结构体的，而方法是属于结构体实例的

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        //rust中 允许方法名和结构体字段名相同
        fn width(&self) -> u32 {
            self.width
        }
        //关联函数
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    let rect = Rectangle { width: 30, height: 50 };
    // 字段
    println!("rect.width = {}", rect.width);
    // 方法
    println!("rect.width() = {}", rect.width());

    /*
        NOTICE
        在C/C++中有两个不同的运算符：.和->来调用方法，.直接在对象上调用方法 而->在对象的指针上调用方法 这是需要先解引用指针
        e.g. object->method() 等价于 (*object).method()
        而在Rust中，只有.运算符，它会自动解引用对象
        所以上述代码中的rect.width()等价于(&rect).width()
    */
    // 关联函数 rust定义在impl中且没有self的函数被称为关联函数 约定俗成使用new来定义关联函数 所以rust特地没有提供new关键字 因为是函数所以不能用.来调用
    let square = Rectangle::square(10);

    // 可以定义多个impl
    impl Rectangle {
        fn print(&self) {
            println!("width = {}, height = {}", self.width, self.height);
        }
    }

    //为枚举实现方法
    impl Message {
        fn call(&self) {
            println!("call");
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    //=======================泛型=======================
    //泛型是一种抽象，它通过在编译时不指定具体类型来实现代码复用
    //使用特征trait来实现泛型 例如std::ops::Add<Output = T>
    fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
        a + b
    }
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c = {}", c);
    //结构体中使用泛型
    struct Point2<T> {
        x: T,
        y: T,
    }
    let p1 = Point2 { x: 1, y: 2 };
    let p2 = Point2 { x: 1.0, y: 2.0 };
    // 上述Point2结构体中的x和y必须是同一种类型，否则编译器会报错 error[E0308]: mismatched types
    //let _p3 = Point2 { x: 1, y: 2.0 };
    //如果想要x和y是不同的类型，可以使用多个泛型
    struct Point3<T, U> {
        x: T,
        y: U,
    }
    let p3 = Point3 { x: 1, y: 2.1 };
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    //枚举中使用泛型 详见Option<T> 和 Result<T, E>

    //方法中使用泛型 使用泛型参数前 需要提前声明 impl<T>（方法泛型） 这样才能在Point2中用它
    impl<T> Point2<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p4 = Point2 { x: 1, y: 2 };
    println!("p4.x = {}", p4.x());

    //为具体的泛型类型实现方法 即只当Point2的实例是某个具体类型（如f32）时才实现方法
    impl Point2<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p5 = Point2 { x: 1.0, y: 2.0 };
    println!("p5.distance_from_origin = {}", p5.distance_from_origin());
    //const泛型 一个典型应用就是泛型定义数组长度

    //定义函数参数为元素类型是T 长度是N（usize）的数组 可以传入任意长度的数组
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("arr = {:?}", arr);
    }
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3, 4, 5];
    display_array(arr1);
    display_array(arr2);

    //=======================trait特征=======================
    //类似于golang的interface 但需要显式继承
    //trait是一种定义共享行为的方法，可以通过trait来定义共享的行为，然后在不同的类型上实现这些trait，从而实现共享行为
    //定义
    pub trait Summary {
        fn summarize(&self) -> String;
        fn summarizeDefalt(&self) -> String {
            String::from("Read more...")
        }
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{},作者是{}", self.title, self.author)
        }
    }
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
        fn summarizeDefalt(&self) -> String {
            String::from("微博内容太长了，不想看")
        }
    }
    let post = Post { title: "RustDemo".to_string(), author: "zsyu9779".to_string(), content: "Rust great!".to_string() };
    let weibo = Weibo { username: "zsyu9779".to_string(), content: "hello".to_string() };
    println!("post.summarize = {}", post.summarize());
    println!("weibo.summarize = {}", weibo.summarize());
    //上面的Summary 被定义成了pub公开的 如果他人想要使用Summary特性 可以将其引入到作用域中进行实现
    //关于特征实现与定义的位置 有一条非常重要的原则 如果想为类型A实现特征T 那么A或者T至少有一个是在当前作用域中定义的
    //e.g. 如果要为上述的Post实现标准库中的Display特征 是OK的 因为Post定义在当前作用域 同样的为String实现上面的Summary特征也是OK的
    // 但要为String实现Display特征是不行的 因为String是标准库中的类型 没有定义在当前定义域 该规则被称为孤儿规则（orphan rule）确保其他人编写的代码不会破坏你的代码

    //默认实现 可以在trait中定义默认实现，这样其他类型无需再实现该方法，或者可以选择重载该方法 详见上面的summarizeDefalt方法
    println!("post.summarizeDefalt = {}", post.summarizeDefalt());
    println!("weibo.summarizeDefalt = {}", weibo.summarizeDefalt());//重载了
    //默认实现允许调用相同trait中的其他方法
    //...

    //trait作为参数
    // notify函数接收一个实现了Summary特征的参数 该参数可以是任意实现了Summary特征的类型(可以理解为interface参数，真正调用时实现多态)
    pub fn notify(item: &impl Summary) {
        println!("notify = {}", item.summarize());
    }
    notify(&post);
    notify(&weibo);
    //特征约束 trait bound
    //虽然impl Trait这种语法易于理解 但实际上是一个语法糖 完整写法如下
    //pub fn notify<T: Summary>(item: &T) {
    //    println!("notify = {}", item.summarize());
    //}
    //这里的<T: Summary>被称为特征约束，它表示notify函数接收一个实现了Summary特征的泛型参数T
    // 简单场景下impl Trait提供了简洁的语法 但是对于复杂的场景 特征约束可以让我们拥有更大的灵活性和语法表现能力：

    // 1.可以在函数中使用多个泛型参数
    pub fn notify1(item1: &impl Summary, item2: &impl Summary) {
        println!("notify1 = {}", item1.summarize());
        println!("notify1 = {}", item2.summarize());
    }
    //NOTICE 上述代码中的item1和item2可以是不同的类型，只要它们都实现了Summary特征 但如果我们需要限制item1和item2是同一种类型，那么就需要使用特征约束：
    pub fn notify2<T: Summary>(item1: &T, item2: &T) {
        println!("notify2 = {}", item1.summarize());
        println!("notify2 = {}", item2.summarize());
    }
    //Test
    notify1(&post, &weibo);
    //notify2(&post,&weibo); //error[E0308]: mismatched types

    // 2.可以在函数中使用多个特征约束 例如约束参数必须同时满足Summary和Display特征
    pub fn notify3<T: Summary + std::fmt::Display>(item: &T) {
        //参数实现了Display特征就可以直接打出这行
        println!("notify3 = {}", item);
    }
    //Test
    impl Display for Post {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "title = {},author = {},content = {}", self.title, self.author, self.content)
        }
    }
    notify3(&post);

    // 3.可以在函数中使用where子句来简化特征约束
    // 当特征约束很多的时候 函数的签名会变的很复杂
    fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        0
    }
    //可以简化为
    fn some_fn1<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
        0
    }
    //where子句可以将特征约束放在函数签名的最后，这样可以让函数签名更加清晰
    //返回中的impl Trait
    //可以在函数返回值中使用impl Trait，这样可以返回实现了某个特征的类型，这种写法被称为返回值多态（return type polymorphism）
    //...

    //通过derive派生特征
    //形如 #[derive(Debug)]
    //例如 Debug 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 println!("{:?}", s) 的形式打印该结构体的对象。

    //调用方法需要引入特征
    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("a < b");
    }
    //特征对象
    //特征对象是一个存放了实现了某个特征的类型的对象，它可以用作参数或返回值，这样可以在运行时动态的选择实现了某个特征的类型
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("Button");
        }
    }
    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox");
        }
    }
    //特征对象的写法是&dyn Trait，它表示一个实现了Trait特征的引用
    pub struct Screen {
        //动态数组 元素是实现了Draw特征的类型
        pub components: Vec<Box<dyn Draw>>,
    }
    //以下代码块解释Box<dyn Draw>的作用
    {
        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8 : {}", *self)
            }
        }
        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("u8 : {}", *self)
            }
        }

        //若T实现了Draw特征 则调用该函数时传入的Box<T>可以被隐式转换成函数签名中的Box<dyn Draw>
        fn draw1(x: Box<dyn Draw>) {
            // 由于实现了Dereference特征 Box智能指针会自动解引用为它所包裹的值 然后调用该值对应的类型上定义的draw方法
            x.draw();
        }

        fn draw2(x: &dyn Draw) {
            x.draw();
        }

        let x = 1.1f64;
        let y = 8u8;

        //x和y都实现了Draw特征 因为Box<T> 实现了Deref特征 所以Box<T>可以被隐式转换成Box<dyn Draw>
        //基于x的值创建了一个Box<f64>，指针指向的数据被放置在堆上
        draw1(Box::new(x));
        draw1(Box::new(y));
        draw2(&x);
        draw2(&y);
    }

    //接下来完善Screen的run方法 用于将列表中的组件渲染在屏幕上
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    //通过泛型实现Screen如下
    pub struct Screen1<T: Draw> {
        pub components: Vec<T>,
    }
    impl <T> Screen1<T>
        //where特征约束 所以这种写法有个弊端就是列表中的组件必须是同一种类型 即全部是SelectBox或全部是Button
        where T: Draw {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    // 特征对象的动态
}

fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 2),
    }
}

#[derive(Debug)]
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
struct IpInfo {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_IpAddrKind(ip_kind: &IpAddrKind) {
    println!("ip_kind = {:?}", ip_kind);
}

fn print_IpInfo(ip_info: IpInfo) {
    println!("ip_info = {:?}", ip_info);
}

fn print_PokerCards(poker_cards: &[PokerCard]) {
    for card in poker_cards {
        println!("poker_card = {:?}", card);
    }
}

fn send_Msg(msg: Message) {
    println!("msg = {:?}", msg);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
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

fn calculate_length(s: &String) -> usize {
    s.len()
}// 因为函数不拥有s的所有权，所以s离开作用域后不会被drop

fn change(some_string: &mut String) { //这里传递的是可变引用
    some_string.push_str(", world");
}

fn calculate_length2(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


// fn dangle() -> &String { // dangle 返回一个字符串的引用
//    let s = String::from("hello"); // s 是一个新字符串
//    &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

fn first_word(s: &String) -> &str {
    &s[..1]
}

struct Struct {
    e: i32,
}
