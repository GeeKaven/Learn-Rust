// struct 结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体，没有字段名，只有类型
struct Color(i32, i32, i32);

// 单元结构体
// 不需要花括号或圆括号
struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;

    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("s@example.com"),
        sign_in_count: 1,
    };

    // 可变结构体整个实例都是可变的，不允许只将某个字段标记为可变
    let mut user2 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("s@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("x@example.com");

    // 结构体更新语法
    // 这也是移动，总体上说创建 user3 后就不能在使用 user1 了，
    // 因为 user1 中的 username 移动到了 user3 中，不能使用 user1.username 了
    // 但是如果我们给 username 和 email 都赋予新 String，只使用 active 和 sign_in_count
    // 那么 user1 在 user3 创建后仍有效，因为 active 和 sign_in_count 实现的是 copy
    let _user3 = User {
        email: String::from("c@example.com"),
        ..user1 //指定剩余未显示设置的字段，必须放在最后
    };
    println!("email {}", user1.email);

    let _black = Color(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // {:?}, {:#?} println! 宏
    println!("rect1 is {:?}, area: {}", rect1, rect1.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //username: username  简写语法与 js 差不多
        email,    //email: email
        sign_in_count: 1,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数 Rectangle::square(3)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
