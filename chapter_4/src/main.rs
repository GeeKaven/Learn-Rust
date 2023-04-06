// 所有权 （ownership）
// 1. Rust 中的每一个值都有一个 所有者（owner）。
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。 `drop`

// 所有权移动 move, 变量每一次赋值可以说是一次所有权的移动。
// 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
// 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
fn main() {
    let s = String::from("Hello"); // s 进入作用域

    // s 移动到函数中
    take_ownership(s);
    // 所有这句不起作用 println!("xx: {}", s);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 一个变量的可变引用同时只能创建一个
    // 也不能在拥有不可变引用的同时拥有可变引用
    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    // slice 引用集合中一段连续的元素序列
    let str = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
}

fn take_ownership(some_string: String) {
    //some_string进入作用域
    println!("{}", some_string)
} // some_string 移出作用域，并调用 `drop` 方法
  // 占用的内存被释放

// 参数为引用， 默认引用无法修改里面的值，
// 可变引用：将其参数变成 `&mut String` 调用变成calculate_length(&mut s)
fn calculate_length(s: &String) -> usize {
    s.len()
}
