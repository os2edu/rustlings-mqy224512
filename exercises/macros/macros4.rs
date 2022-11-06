// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// 宏的使用要加！号
// 宏要先声明再使用
// 宏需要使用macro_export导出
// 多个表达式之间要用‘;’分开
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
