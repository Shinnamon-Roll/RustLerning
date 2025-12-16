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
                true
            },
            BookStatus::Borrowed(_) => false,
       }
       
    }

    fn return_book(&mut self, user_id: u64) -> bool {
        match self.status {
            BookStatus::Borrowed(borrower) if borrower == user_id => {
                self.status = BookStatus::Available;
                true
            },
            BookStatus::Borrowed(_) => false,  // ผิดคน ยืมไป
            BookStatus::Available => false,  // หนังสือว่างอยู่แล้ว
        }
    }
}

fn main() {
    let mut books: Vec<Book> = Vec::new();
    let mut users: Vec<User> = Vec::new();

    println!("========== เริ่มต้นระบบห้องสมุด ==========\n");

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

    users.push(User {
        id: 2,
        name: String::from("Bob"),
        age: 25,
        status: UserStatus::Active,
        department: String::from("Marketing"),
    });


    println!("📚 หนังสือ: {}", books[0].title);
    println!("👤 ผู้ใช้: {} (ID: {})", users[0].name, users[0].id);
    println!("📖 สถานะเริ่มต้น: Available\n");

    println!("---------- ขั้นตอน 1: ยืมหนังสือ ----------");

    // ยืมหนังสือ
    let borrowBook: bool = books[0].borrow(users[0].id);

    if borrowBook {
        println!("✅ ยืมหนังสือสำเร็จ!");
        println!("📖 สถานะหลังยืม: Borrowed by User ID {}", users[0].id);
    } else {
        match &books[0].status {
            BookStatus::Borrowed(user_id) => {
                println!("❌ หนังสือถูกยืมไปแล้วโดย User ID: {}", user_id);
            },
            _ => {
            }
        }
        println!("❌ ยืมหนังสือไม่สำเร็จ");
    };
    println!();

    println!("---------- ขั้นตอน 2: คืนหนังสือ ----------");

    // คืนหนังสือ
    let returnBook: bool = books[0].return_book(users[1].id);

    if returnBook {
        println!("✅ คืนหนังสือสำเร็จ!");
        println!("📖 สถานะหลังคืน: Available");
    } else {
        println!("❌ คืนหนังสือไม่สำเร็จ");
        match &books[0].status {
            BookStatus::Borrowed(user_id) => {
                println!("   หนังสือถูกยืมโดย User ID {} ไม่ใช่ {}", user_id, users[0].id);
            },
            BookStatus::Available => {
                println!("   หนังสือว่างแล้ว");
            }
        }
    }
    println!("\n========== สิ้นสุดการทำงาน ==========");
}
