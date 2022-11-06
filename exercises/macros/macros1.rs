// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 宏的使用
fn main() {
    my_macro!();
}
