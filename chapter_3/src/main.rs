fn main() {
    // 默认变量不可变
    let x = 5;
    println!("Immutable: The value of x is: {x}");
    // 无法通过 x = 6;

    // 使用 mut 关键字使变了可变
    let mut y = 5;
    println!("Mutability: The value of y is: {y}");
    y = 6;
    println!("Mutability: The value of y is: {y}");

    // 常量，总是不可变的，不允许使用 mut, 并且 必须 注明值的类型
    // 命名约定 单词全大写加下划线
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // Shadowing (隐藏) 可以定义一个与之前变量同名的变量
    // 与 mut 不同点是，其实是创建了一个新变量
    // 所以主要用于类型转换

    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // 数据类型
    // Scalar Types: 整型，浮点型，布尔型和字符型
    let _i32: i32 = 1;
    let _u32: u32 = 2;

    let _fx = 2.0;
    let _fy: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 5.5;
    let _multiplication = 4 * 100;
    let _division = 38.2 / 12.3;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _p: char = 'ℤ';
    let _emoji = '🤣';

    // Compound Types (符合类型)：Tuple (元组), Array (数组)
    let tup = (500, 1.2, 1);
    let (t1, t2, t3) = tup; // destructuring (解构)
    println!("t1: {t1}, t2: {t2}, t3: {t3}");

    println!("t1: {}, t2: {}, t3: {}", tup.0, tup.1, tup.2);

    // 数组长度是固定的!!!
    let arr = [1, 2, 3, 4, 5];

    // 创建[3,3,3,3,3], [num; length]
    let _arr1 = [3; 5];
    println!("Array0: {}, Array3: {}", arr[0], arr[3]);

    // 函数
    my_function(3);

    let five = five();
    println!("Five : {five}");

    // condition 流程控制
    let num = 2;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // !! 代码块的值是其最后一个表达式的值
    let _num1 = if condition { 5 } else { 6 };

    // 循环： loop, while, for
    // loop 重复执行
    // 
    let mut index = 0;
    let res = loop {
        index += 1;
        if index == 10 {
            break index * 2;
        }
    };
    println!("Result is {res}");

    // 循环标签
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }

    // for
    for el in arr {
        println!("Arr value is: {el}")
    }

    for c in (1..4).rev() {
        println!("rev {c}");
    }

}

// Functions (函数)， 函数名风格snake case，小写字母加下划线
// 必须声明每个参数的类型
fn my_function(a: i32) {
    println!("my function a: {a}");
}

// 表达式结尾没有分号，如果加上分号就变成了语句
// 表达式有返回值，语句没有
// 有返回值就代表可以绑定到其他变量上

// 具有返回值的函数
fn five() -> i32 {
    5
}
