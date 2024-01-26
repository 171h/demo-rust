fn main() {
    println!("Hello, world!");

    let study_list = vec!["a", "b", "c"];
    let study_list2 = study_list; // 赋值可以移动所有权
    show(&study_list2);
    println!("{:?}", study_list2);

    /*
    Borrowing（借用），就是一个函数中的变量传递给另外一个函数作为参数暂时使用。
    也会要求函数参数离开自己作用域的时候将所有权 还给当初传递给它的变量（好借好还，再借不难嘛!）。
    可变借用时，借用函数定义时候和使用的时候都要用 &mut，同时被借用的变量要加上 mut 关键字
     */
    let mut study_list3 = vec!["a", "b", "c"];
    println!("{:?}", study_list3);
    show2(&mut study_list3);
    println!("{:?}", study_list3);

}

fn show(v: &Vec<&str>) {
    println!("{:?}", v);
}

fn show2(v: &mut Vec<&str>) {
    v[0] = "i";
    println!("{:?}", v);
}
