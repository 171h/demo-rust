fn main() {
    println!("Hello, world!");

    let arr1: [&str; 3] = ["Huang", "wei", "jie"];
    let mut arr2 = ["H", "u", "a", "n", "g"];
    println!("{:?}", arr1);
    println!("arr1 length is {}", arr1.len());
    println!("{:?}", arr2);
    println!("arr2 length is {}", arr2.len());

    arr2[0] = "";
    println!("{:?}", arr2);

    show_arr(arr2);
    println!("{:?}", arr2);
    
    modify_arr(&mut arr2);
    println!("{:?}", arr2);
}

fn show_arr(mut arr: [&str; 5]) {
    let l = arr.len();
    arr[3] = "171h";
    for i in 0..l {
        println!("arr item is {}", arr[i]);
    }
}

fn modify_arr(arr: &mut[&str; 5]) {
    let l = arr.len();
    arr[3] = "171h";
    for i in 0..l {
        println!("arr item is {}", arr[i]);
    }
}
