// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.
fn main() {

    // 对于数组来说，元素类型和长度都算是数组类型的一部分
    let a: [i32;3] = [1, 2, 3];

    // 能够自动推导数据类型
    //let a = [1, 2, 3 ,4];

    // 数组有5个元素都是3, 可以使用这种方式对数组进行初始化
    // let b = [3; 5];
   // println!("{:?}", b); // [3, 3, 3, 3, 3]

    // slice, 左闭右开
    // let c = &a[0..];
    // println!("{:?}", c);  // [1,2,3]

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
