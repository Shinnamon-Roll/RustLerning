// แบบทดสอบ Rust - เฉพาะ Struct, Enum, Impl

#[derive(Debug)]
enum UserStatus {
    Active,
    Offline,
    Busy,
    Away,
    Working,
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
    println!("=== Rust Learning Tests (Struct, Enum, Impl) ===\n");

    // Test 1: สร้าง Struct User
    println!("Test 1: สร้าง Struct User");
    let user1 = User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        status: UserStatus::Active,
        department: String::from("Engineering"),
    };
    println!("User 1: {:?}\n", user1);

    // Test 2: Enum - UserStatus ต่างๆ
    println!("Test 2: Enum - สถานะต่างๆของ User");
    let user2 = User {
        id: 2,
        name: String::from("Bob"),
        age: 28,
        status: UserStatus::Offline,
        department: String::from("Sales"),
    };
    println!("User 2 status: {:?}", user2.status);

    let user3 = User {
        id: 3,
        name: String::from("Charlie"),
        age: 35,
        status: UserStatus::Busy,
        department: String::from("HR"),
    };
    println!("User 3 status: {:?}", user3.status);

    let user4 = User {
        id: 4,
        name: String::from("Diana"),
        age: 26,
        status: UserStatus::Away,
        department: String::from("Design"),
    };
    println!("User 4 status: {:?}\n", user4.status);

    // Test 3: Impl - go_offline() method
    println!("Test 3: Impl - go_offline() method");
    let mut user5 = User {
        id: 5,
        name: String::from("Eve"),
        age: 32,
        status: UserStatus::Active,
        department: String::from("Marketing"),
    };
    
    println!("ก่อนเรียก go_offline():");
    println!("  User: {} (status: {:?})", user5.name, user5.status);
    
    user5.go_offline();
    
    println!("หลังเรียก go_offline():");
    println!("  User: {} (status: {:?})\n", user5.name, user5.status);

    // Test 4: ลองใช้หลายๆ user พร้อมกัน
    println!("Test 4: หลายๆ User พร้อมกัน");
    let mut user6 = User {
        id: 6,
        name: String::from("Frank"),
        age: 29,
        status: UserStatus::Working,
        department: String::from("Operations"),
    };

    let mut user7 = User {
        id: 7,
        name: String::from("Grace"),
        age: 27,
        status: UserStatus::Active,
        department: String::from("Support"),
    };

    println!("ก่อน:");
    println!("  Frank: {:?}", user6.status);
    println!("  Grace: {:?}", user7.status);

    user6.go_offline();
    user7.go_offline();

    println!("หลัง go_offline():");
    println!("  Frank: {:?}", user6.status);
    println!("  Grace: {:?}\n", user7.status);

    println!("=== ทดสอบเสร็จสิ้น ===");
}
