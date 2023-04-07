use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    let t = &v[2];

    println!("v: {:?}, third: {}", v, t);

    for i in &mut v {
        println!("{i}");
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 只能将 &str 和 String 相加，不能将两个 String 值相加, &String 被降转为 &str了
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // 宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
    let _s = format!("{s2}-{s3}");

    // rust 中不支持索引字符串 ❌：s[1]

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 像 String 这样有所有权的值，将被移动，Hashmap成为这些值的所有者
    map.insert(field_name, field_value);
    // 后续 field_name，field_value 将无法调用

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    
}
