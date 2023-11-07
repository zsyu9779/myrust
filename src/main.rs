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
}

struct Struct {
    e: i32,
}
