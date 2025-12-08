#[derive(Debug)]
enum UserStatus {
    Active,
    Offline,
    Busy,
    Away,
    Working,

}

#[derive(Debug)]

struct Book {
    title: String,
    price: f32,
    author: String,
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
    });

    users.push(User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        status: UserStatus::Active,
        department: String::from("Engineering"),
    });
    users.push(User {
        id: 2,
        name: String::from("Bob"),
        age: 28,
        status: UserStatus::Working,
        department: String::from("Sales"),
    });
    users.push(User {
        id: 3,
        name: String::from("Charlie"),
        age: 35,
        status: UserStatus::Busy,
        department: String::from("HR"),
    });

    println!("ตอนนี้มีหนังสือทั้งหมด: {:?}", books[0].title);

    println!("ตอนนี้มีพนักงานทั้งหมด: {:?}", users[2]);
    
    println!("จำนวนพนักงาน: {} คน", users.len());


}
