/*
常量 就是那些值不能被改变的变量。定义后，再也没有任何方法可以改变常量的值。

const：不可改变的值（通常使用这种）。
static：具有 ‘static 生命周期的，可以是可变的变量（须使用 static mut 关键字）。

使用 const 关键字定义常量。
定义常量时必须指定数据类型。
常量名称的命名规则和之前变量的命名规则一样，但常量名称一般都是 大写字母。
*/
fn main() {
    const PI:f64 = 3.1415926;
    println!("PI {}", PI);

    let _name:&str = "Go语言一本通";
    let _name:&str = "我在学Rust";
    println!("name {}", _name);

    let _price:i32 = 199;
    let _price:&str = "299";
    println!("price {}", _price);

    const DISCOUNT:f64 = 0.8;
    // const DISCOUNT:f64 = 0.6;
    println!("DISCOUNT {}", DISCOUNT);

    static BOOK: &'static str = "Go语言一本通";
    println!("BOOK {}", BOOK);
    // BOOK = "Go语言一本通123";
    // println!("BOOK {}", BOOK);
}
