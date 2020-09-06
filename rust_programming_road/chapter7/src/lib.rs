/// # chapter7 结构化编程
/// 
/// ## 面向对象风格编程
/// 
/// 
/// rust支持面向对象风格的编程， 传统面向对象中父子继承是为了实现代码复用和多态，
/// 其本质在类型系统概念中属于子类型多态，而Rust使用trait和泛型提供参数化多态就
/// 就完全满足这个需求。
/// 
/// 对于代码复用， Rust提供泛型单态化和triat对象来避免代码重复，从而支持代码复用。
/// Rust还提供了功能强大的宏系统帮助复用代码，甚至还可以使用一些设计模式来避免
/// 代码重复。
/// Rust还实现了一种名为**特化**，的功能来增强代码的高效复用。
/// 
/// 总结：
/// - 封装， rust提供了结构体，枚举来封装数据， 并可以使用pub关键字定义其字段可见性
/// 提供了impl关键字来实现数据的行为
/// - 多态，通过trait和泛型以及枚举来允许程序操作不同类型的值
/// - 代码复用， 通过泛型，trait对象，宏，语法扩展，代码生成来设计模式
/// 
/// ## 结构体
/// 
/// 结构体和枚举体是Rust中最基本的两种符合类型，这两种复合类型都属于同一种概念，
/// 他们都属代数数据类型(ADT, Algebraic Dat Type). 
/// 
/// ### 代数数据类型之积类型
/// 
/// 代数数据类型就是值具备了代数能力的数据类型，即数据类型可以进行代数运算并马努一定运算
/// 规则，正是这一点保证了数据类型中的许多性质是可以**复用**的。比如一个结构体中包含的成员
/// 都是拥有复制语义的简单原始数据类型，那么这个结构体也是可以通过派生属性#[derive]来放心
/// 的为其实现Copy
/// 
/// Rust 中的结构体属于代数数据类型中的积类型， 积类型是来自范畴论的术语
/// 积类型， 可以通过乘法原理来理解
/// 描述的是，做一件事需要分成很多步，每一步都相互依赖，他表示的是一种组合。
/// 积类型代表的是一种数据结构的复合方式
/// 
/// ## 使用结构体进行面向对象风格编程
/// 
/// 实现在终端输出指定颜色的字符
/// 
/// ```
/// let hi = "hello".red().on_yellow();// 表示的是黄色背景下的红色字符串hello.
/// println!("{}", hi);
/// ``` 
/// 
/// 在终端显式带颜色的字符，使用的ANSI转移序列， ANSI转移序列就是指如ESC和[组合而成的字符
/// 序列，可以实现在屏幕上定位光标或改变输出字符颜色等功能，所以也被称为控制字符。被定义与
/// ASCII码中。
/// ESC有三种表示方式： 
/// - 在shell中表示为\e.
/// - 在ASCII十六进制表示\x1B
/// - 在ASCII八进制表示为\033
/// 
/// 
/// 在终端显式带指定颜色的字符hello， 
/// 
/// echo "\e[31;43mHello\e[0m"
/// echo "\x1B[31;43mHello\x1B[0m"
/// echo "\033[31;43mHello\033[0m"
/// 
/// \x1B为前缀，表示这是一个ANSI控制序列的开始， 用分号相隔的31;43属于颜色代码， 31是
/// 前景色，代表红色， 43位背景色， 代表黄色，字面m为结束符，最后的\x1B[0m结尾代表重置
/// 全部属性，表示一个ANSCi控制序列的结束。
/// 
/// 
/// 实现步骤；
/// 1 定义一个结构体，来封装动态变化的两部分数据
/// 2 为此结构体定义指定颜色的方法，如， red， on_yellow
/// 3 为了实现直接在字符串字面量上链式调用red和on_yellow方法，就必须
/// 为&'a str类型也实现red和on_yellow方法
/// 4 为结构体实现方法， 用于组装ANSI字符串序列
/// 5 打印结果
/// 
///  
/// 
///  
#[test]
pub fn example() {
    // 积类型 Struct
    #[derive(Debug, Clone,Copy)]
    struct Book<'a> { // 所有的字段都是复制语义的
        name: &'a str,
        isbn: i32,
        version: i32,
    }

    let book  = Book {
        name: "the Rust book", isbn: 20181212, version: 1
    };

    let book2 = Book {
        version: 2, ..book // 结构体跟新语法.., book的所有权并没有被转移， 其中
        // 所有的字段都是具有复制的语义的， 这里不会发生所有权转移， 
        // 结构体跟新语法允许使用".."语法来减少代码重复。
    };

    println!("{:?}", book);
    println!("{:?}", book2);

    // 如果结构体中使用了移动语义的成员字段，则不允许实现Copy
    #[derive(Debug, Clone)]
    struct BookNoCopy { //Rust不允许包含String类型的字段实现Copy,
        name: String,
        isbn: i32,
        version: i32,
    }

    let book = BookNoCopy {
        name: "the rust book".to_string(), 
        isbn: 20181212,
        version: 1,
    };

    let book2 = BookNoCopy {
        // version: 2,
        name : String::from("hello, world"),
        ..book 
        // ..book // there will be move , this book name have been move book2
    };

    println!("{:?}", book2);
    // println!("{:?}", book);// error, book, name have move to book2
}



pub mod color;