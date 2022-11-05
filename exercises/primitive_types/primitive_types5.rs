// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {

    // 元组
    let mut cat = ("Furry McFurson", 3.5);

    // 对元组进行解构
    let (name, age) = cat;

    // 元组属于基本数据类型，实现了copy的trait
    let cat1 = cat;

    cat.0 = "TOM";

    println!("{} is {} years old.", cat.0, cat.1);
    println!("{} is {} years old.", cat1.0, cat1.1);
    println!("{} is {} years old.", name, age);
}
