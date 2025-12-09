#[derive(Debug)]
enum UserStatus {
    Active,
    Offline,
    Busy,
    Away,
    Working,
}

#[derive(Debug)]
enum BookStatus {
    Avialable,
    Borrowed,   
}

#[derive(Debug)]

struct Book {
    title: String,
    price: f32,
    author: String,
    status: BookStatus
}
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    age: u8,
    status: UserStatus,
    department: String,
}

impl User {
    fn go_offline(&mut self) {
        self.status = UserStatus::Offline;
    }
}

fn main() {
    let mut books: Vec<Book> = Vec::new();
    let mut users: Vec<User> = Vec::new();

    books.push(Book {
        title: String::from("The Rust Programming Language"),
        price: 39.99,
        author: String::from("Steve Klabnik and Carol Nichols"),
        status: BookStatus::Avialable
    });

    users.push(User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        status: UserStatus::Active,
        department: String::from("Engineering"),
    });

    for user in users.iter() {
        print!("ID: {} ชื่อ: {} -> ", user.id, user.name);

        match user.status {
            UserStatus::Active => println!("สถานะ: Active"),
            UserStatus::Offline => println!("สถานะ: Offline"),
            UserStatus::Busy => println!("สถานะ: Busy"),
            UserStatus::Away => println!("สถานะ: Away"),
            UserStatus::Working => println!("สถานะ: Working"),
            _ => println!("สถานะ: ไม่ทราบ"),
        }
    }

    for book in books.iter() {
        print!("ชื่อหนังสือ: {} ผู้เขียน: {} ราคา: {} บาท -> ", book.title, book.author, book.price);

        match book.status {
            BookStatus::Avialable => print!("สถานะ: Avialable"),
            BookStatus::Borrowed => print!("สถานะ: Borrowed"),
            _ => println!("สถานะ: ไม่ทราบ"),
        }
    }





}
