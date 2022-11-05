// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// 引用的再借用，如果是不可变借用，要满足借用栈规则
// 即数据只要被可变的引用，原始的引用暂时失去作用，直到新的引用完成任务
// 类似于一个stack, 新的引用在栈顶
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut *y;
    *z += 1000;
    *y += 100;
    assert_eq!(x, 1200);
}
