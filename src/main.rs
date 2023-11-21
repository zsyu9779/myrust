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
    assert_eq!(slice, &[2,3]);

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
    s.insert(10,'!'); //在索引10处插入字符
    println!("s = {}", s);
    //替换
    let mut s1 = s.replace("rust","RUST");//适用于String和&str 返回的是一个新字符串 原字符串不必mut
    let mut s2 = s1.replacen("RUST", "rust", 1); //适用于String和&str 返回的是一个新字符串 原字符串不必mut 只替换第一个目标值
    s2.replace_range(0..5,"HELLO"); //适用于String 直接操作原字符串 必须mut
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
    let mut s3 = s3 +"!";
    s3 +="!";
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
    let tup : (i32,f64,u8) = (500,6.4,1);
    //可以用模式匹配或者.索引的方式获取元组的值
    let (x,y,z) = tup;
    println!("x = {}, y = {}, z = {}",x,y,z);
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}",tup.0,tup.1,tup.2);
    //元组可以作为函数的参数和返回值
    let s1 = String::from("hello");
    let (s2,len) = calculate_length2(s1);
    println!("s2 = {}, len = {}",s2,len);

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
    println!("u1.username = {}",u1.username);
    let u2 = build_user(String::from("aaa@example.com"),String::from("王五"));
    println!("u2.username = {}",u2.username);
    // 利用已有的user1 创建user3
    let u3 = User {
        email: String::from("xxx@example.com"),
        ..u1
    }; // u3和u1只有email字段不同 所以可以用 ..u1 语法来创建u3 ..u1这种写法必须在结构体尾部使用
    //username字段发生了所有权转移
    println!("u3.username = {}",u3.username);

    //元组结构体 结构体需要名称，但是结构体字段不一定需要
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("black = ({},{},{})",black.0,black.1,black.2);
    println!("origin = ({},{},{})",origin.0,origin.1,origin.2);

    //单元结构体
    struct Unit;
    let _unit = Unit;




}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email:String,username: String) -> User {
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

fn calculate_length2(s: String) ->(String,usize) {
    let len = s.len();
    (s,len)
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
