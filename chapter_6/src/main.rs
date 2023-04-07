enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let m = Message::Write(String::from("Hello"));

    // option 内置的枚举
    // 当不确定一个值是否为空时可以使用 Option 枚举，
    // 这也在使用这个值时，会考虑到可能为空，会为其做判断
    let _some_num = Some(5);
    let _some_str = Some('e');

    // match 控制流
    let nickel = Coin::Nickel;
    println!("Coin : {}", value_in_cents(nickel));

    // `if let` 一个 match 语法糖，只匹配某一模式时有用
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max: {}", max);
    } else {
        println!("None");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
