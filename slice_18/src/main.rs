fn main() {
    println!("Hello, world!");

    let mut v = Vec::new();
    v.push("Huang");
    v.push("wei");
    v.push("jie");
    v.push("learn");
    v.push("rust");
    println!("len: {}", v.len());

    // 切片的定义
    let s1 = &v[1..3];  // [起始位置..结束位置] 这是一个左闭右开的区间。
    let s2 = &v[..3];   // [..结束位置] 从0到结束位置的元素
    let s3 = &v[1..];   // [起始位置..] 从起始位置到结束位置的元素
    let s4 = &v[..];

    println!("s1:{:?}", s1);
    println!("s2:{:?}", s2);
    println!("s3:{:?}", s3);
    println!("s3:{:?}", s4);
    // 切片当参数，切片通过引用的方式传递给函数。
    show_slice(s1);
    println!("s1:{:?}", s1);
    // 切片当参数，切片通过引用的方式传递给函数。
    show_slice(s1);

    // 可变切片
    // 如果我们声明的原数据是可变的，同时定义切片也有**&mut**关键字，就可以更改切片元素来更改元数据了。
    let mut v2 = Vec::new();
    v2.push("Huang");
    v2.push("wei");
    v2.push("jie");
    println!("v2:{:?}", v2);
    modify_slice(&mut v2[1..3]);
    println!("v2:{:?}", v2);

}

// 切片当参数，切片通过引用的方式传递给函数。
fn show_slice(s: &[&str]) {
    println!("show_slice 函数内：{:?}", s);
}

fn modify_slice(s: &mut [&str]) {
    s[0] = "Wang";
    println!("modify_slice 函数内：{:?}", s);
}
