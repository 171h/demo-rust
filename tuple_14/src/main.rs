fn main() {
    println!("Hello, world!");

    let t = ("Huang", "wei", "jie");
    println!("{:?}", t);
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);
    show_tuple(t);

    let (a, b, c) = t;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn show_tuple(t: (&str, &str, &str)) {
    println!("{:?}", t);
}
