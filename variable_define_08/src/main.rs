/*
let 变量名 = 值;           // 不指定变量类型
let 变量名:数据类型 = 值;   // 指定变量类型

变量的命名规范
1. 可以包含 字母、数字 和 下划线 。
2. 变量名必须以 字母 或 下划线 开头。不能以 数字 开头。
3. 变量名是 区分大小 写的。也就是大写的 Study 和小写的 study 是两个不同的变量。
4. snake case
*/
fn main() {
    let study:&str = "";
    println!("{}", study);

    let mut price = 188;
    println!("price {}", price);
    price = 288;
    println!("price {}", price);

    const PRICE2: i32 = 128;
    println!("PRICE2 {}", PRICE2);
}
