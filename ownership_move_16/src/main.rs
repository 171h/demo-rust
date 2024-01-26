fn main() {
    // let name = "Huang wei jie";

    // move
    // let a = b
    // foo(a)

    // 栈 后进先出。类型大小是固定的。如i32
    // 堆 编译时大小未知或不确定大小。用户自己管理，增加内存溢出风险。

    /*
    所有权只会发生在堆上分配的数据，基础数据类型(整型，浮点型，布尔，字符)存储在栈上，
    所以没有所有权的概念。基础类型可以认为是值拷贝，在内存上另外的地方，存储和复制来的数据，然后让新的变量指向它。
     */
    let a = 88;
    let b = a;
    println!("a {}, and b {}", a, b);
    
    // let v1 = vec!["Huang", "wei"];
    // let v2 = v1;
    // println!("{:?}", v1);

    let study_list = vec!["a", "b", "c"];
    let study_list2 = study_list; // 赋值可以移动所有权
    show(study_list2);
    // println!("studyList2 {:?}", study_list2);

    let study_list3 = vec!["a", "b", "c"];
    let study_list4 = study_list3;
    let result = show2(study_list4);
    
    // 通过返回值的形式进行所有权的转移
    println!("result {:?}", result);
    // println!("result {:?}", study_list4);


}

// 赋值并不是唯一涉及移动所有权的操作，值在作为参数传递或从函数返回时也会被移动
fn show2(v: Vec<&str>) ->Vec<&str>{
    println!("v {:?}", v);
    return v;
}


fn show(v: Vec<&str>) {
    println!("{:?}", v);
}
