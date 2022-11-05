// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.
//   Rust 中字符是Unicode编码，占4个字节，String是UTF-8编码，占3个字节
//   Rust在语言级别只有一种字符串类型，即str,通常以引用的方式出现，即&str，虽然语言级别是str类型，但是在标准库中，还有很多
// 不同用途的字符串类型，而其中使用最广泛的就是String
//   str类型是硬编码进可执行文件，无法被修改，String是一个可增长、可改变并且具有所有权的UTF-8编码字符串，当Rust用户提到
// 字符串时，往往指的是String类型和&str字符串切片类型。
//   除了String类型的字符串，Rust标准库中还提供了其他类型的字符串，如: OsString、OsStr、CsString、CsStr等
// String ---> &str 直接取String的引用就行
// 字符串拼接直接使用+/+=都行，但是要求第二个操作元素是&str类型， 而且第一个操作元素会丢失所有权
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
