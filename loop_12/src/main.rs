fn main() {
    // for in
    // Range
    let list = 1..5;
    for num in list {
        println!("list num is {}", num);
    }

    // RangeInclusive
    let list2 = 1..=5;
    for num in list2 {
        println!("list2 num is {}", num);
    }

    // Vector
    let list3 = vec!["Huang", "wei", "jie"];
    
    for item in list3 {
        println!("list3 item is {}", item);
    }

    // while
    let mut num = 1;
    while num < 20 {
        println!("while num is {}", num);
        num = num *2;
    }

    // loop
    let mut num = 1;
    loop {
        if num > 20 {
            break;
        }
        println!("loop num is {}", num);
        num = num *2;
    }

}
