fn main() {
    println!("Hello, world!");
    hello();
    println!("get_name {}", get_name());
    println!("get_name2 {}", get_name2());
    let mut price = 99;
    double_price(99);
    println!("外部价 is {}", price);
    double_price2(&mut price);
    println!("外部价 is {}", price);
    let name = String::from("Huang wei jie");
    show_name(name);
    // println!("调用show_name函数后 {}", name); //报错
}


fn hello() {
    println!("Hello, Rust!!!");
}

fn get_name() -> String {
    return String::from("I am leaning Rust.");
}

// 没有return 关键字，函数会默认使用最后一条语句的执行结构作为返回值，并且数据类型要保持一致
fn get_name2() -> String {
    String::from("I am leaning Rust.")
}

// 值传递，函数内部和外部各自保存了不同的值，互不影响，因此内外结果不一致。
fn double_price(mut price: i32) {
    price = price * 2;
    println!("内部价 is {}", price);
}

// 引用传递，把当前变量的内存地址传递给函数，传递的变量和函数参数共同指向了同一个内存位置。引用传递在参数类型的前面加上&符号
fn double_price2(p: &mut i32) {
    // *解引用符，用于获取访问变量p指向的内存地址上存储的变量的值
    *p = *p * 2;
    println!("内部价2 is {}", p);
}

fn show_name(name: String) {
    println!("我是名字是 {}", name);
}
