fn main() {
    // if else if else
    let total = 301.00;
    if total > 500.00 {
        println!("打8折,{}", total * 0.8);
    } else if total > 300.0 {
        println!("打8.5折,{}", total * 0.85);
    } else {
        println!("打9折,{}", total * 0.9);
    }

    // match
    let code = "100861";
    let choose = match code {
        "10010" => "联通",
        "10086" => "移动",
        "10000" => "电信",
        _ => "Unknown",
    };
    println!("选择 {}", choose);
}
