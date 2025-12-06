# RustLerning

A learning project for exploring Rust fundamentals, including structs, enums, methods, and mutable state management.

## Project Overview

This project demonstrates core Rust concepts by implementing a simple user management system with status tracking. It serves as a foundation for understanding:

- **Structs & Enums**: Defining custom types (`User` struct and `UserStatus` enum)
- **Methods**: Implementing behavior on types using `impl` blocks
- **Mutable State**: Working with mutable bindings and modifying struct fields
- **Debug Printing**: Using `derive(Debug)` and format strings for output

## Code Structure

### Main Components

- **`UserStatus` Enum**: Represents user states
  - `Active` - User is currently active
  - `Offline` - User is offline
  - `Busy` - User is busy
  - `Away` - User is away
  - `Working` - User is working

- **`User` Struct**: Contains user information
  - `id` (u64): Unique identifier
  - `name` (String): User's name
  - `age` (u8): User's age
  - `status` (UserStatus): Current status
  - `department` (String): Department assignment

- **`User::go_offline()` Method**: Changes user status to offline

## Getting Started

### Prerequisites

- Rust 1.56+ (with 2024 edition support)

### Build & Run

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run with optimizations
cargo build --release
```

## Example Output

```
Before going offline: User { id: 1, name: "Alice", age: 30, status: Active, department: "Engineering" }
After going offline: User { id: 1, name: "Alice", age: 30, status: Offline, department: "Engineering" }
```

## Learning Topics Covered

- Rust ownership and borrowing
- Using `String` vs string literals
- Struct instantiation and initialization
- Method syntax and `self` parameters
- Pattern matching with enums
- Debug trait and pretty-printing with `{:?}`

## Possible Extensions

- Add more status transition methods (e.g., `go_busy()`, `go_active()`)
- Implement `Display` trait for custom formatting
- Add validation logic in struct initialization
- Implement status change history tracking
- Add `PartialEq` derive for comparing users

## License

This is a learning project. Feel free to use and modify for educational purposes.