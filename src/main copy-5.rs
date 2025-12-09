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
    Available,
    Borrowed(u64),   
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

impl Book {
    fn borrow(&mut self, user_id: u64) -> bool {
       match self.status {
            BookStatus::Available => {
                self.status = BookStatus::Borrowed(user_id);
                print!("User ID {} ", user_id);
                true
            },
            BookStatus::Borrowed(_) => false,
       }
       
    }
}

fn main() {
    let mut books: Vec<Book> = Vec::new();
    let mut users: Vec<User> = Vec::new();

    books.push(Book {
        title: String::from("The Rust Programming Language"),
        price: 39.99,
        author: String::from("Steve Klabnik and Carol Nichols"),
        status: BookStatus::Available
    });

    users.push(User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        status: UserStatus::Active,
        department: String::from("Engineering"),
    });

    let is_successfull = books[0].borrow(users[0].id);
    if is_successfull {
        println!("ยืมหนังสือสำเร็จ {}", books[0].title);
    } else {
        println!("หนังสือถูกยืมไปแล้ว {}", books[0].title);
    }

    let is_successfull = books[0].borrow(users[0].id);
    if is_successfull {
        println!("ยืมหนังสือสำเร็จ {} ", books[0].title);
    } else {
        println!("หนังสือถูกยืมไปแล้ว {}", books[0].title);

        match &books[0].status {
            BookStatus::Borrowed(user_id) => {
                println!("หนังสือถูกยืมโดย User ID: {}", user_id);
            }
            _ => println!("สถานะหนังสือไม่ทราบ"),
        }
    }

    



}
