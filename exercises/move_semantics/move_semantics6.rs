// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {

    // 创建一个String
    let data = "Rust is great!".to_string();

    // 发生了所有权专有
    get_char(&data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 如果不创建临时变量，相当于修改引用所指向的地址
fn string_uppercase(mut data: &String) {
    let data = &data.to_uppercase();
    println!("{}", data);
}
