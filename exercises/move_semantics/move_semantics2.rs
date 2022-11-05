// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// 可变借用与不可变借用不能同时存在， 除非是两个对象
// 引用的再借用需要满足borrow stack规则
// 该练习，将一个不可变的vec变为一个可变的vec，并且是共存的
// 方案:
//   1. 拷贝vec0的数据，创建一个新的vector
//   2. 使用borrow stack的规则
fn main() {

    // 创建一个vector对象
    let mut vec0 = Vec::new();


    // 不想所有权转移，创建一个vector, 只能进行数据的拷贝
    let mut vec1 = fill_vec(&mut vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);




}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
