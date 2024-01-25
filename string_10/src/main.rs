fn main() {
    let lesson: &str = "我在学Rust";
    println!("{}", lesson);

    let s1: String = String::new();
    println!("s1:{}, s1-len:{}", s1, s1.len());

    let s2: String = String::from("Rust2");
    println!("s2:{}, s2-len:{}", s2, s2.len());

    let mut s3: String = String::new();
    s3.push_str("s3 push_str");
    println!("s3 {}", s3);

    s3.push('s');
    println!("s3 {}", s3);

    s3.push('t');
    println!("s3 {}", s3);

    let s4: String = String::from("huangweijie");
    println!("s4 {}", s4);

    let result: String = s4.replace("jie", "lilili");
    println!("result {}", result);

    let s6 = "Huang".to_string();
    println!("s6 {}", s6);

    show_name(s6.as_str());

    let s8: &str = "\tHuang\tWei\tjie\r\nis a good man\r\n";
    println!("s8 {}", s8);
    println!("length is {}", s8.len());
    println!("length is {}", s8.trim().len());
    println!("s8 {}", s8);

    let s9 = "常量名称的命名规则，和之前变量的命名规则一样，但常量名称一般都是大写字母。";
    for item in s9.split("，") {
        println!("item {}", item);
    }

    for item in s9.chars() {
        println!("item {}", item);
    }

    let s11 = "常量名称的命名规则";
    let s12 = "huangweijie";
    let result2 = s11.to_string() + &s12.to_string();
    println!("result2 {}", result2)
}

fn show_name(name: &str) {
    println!("充电科目：{}", name);
}
