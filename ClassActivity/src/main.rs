1.// Builder Pattern
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

struct UserBuilder {
    name: String,
    age: u32,
    email: Option<String>,
}

impl UserBuilder {
    fn new(name: String, age: u32) -> Self {
        Self { name, age, email: None }
    }

    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    fn build(self) -> User {
        User { name: self.name, age: self.age, email: self.email }
    }
}

let user = UserBuilder::new("Alice".into(), 30)
    .email("alice@mail.com".into())
    .build();
// Recursion
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
// Pure Function
fn add(a: i32, b: i32) -> i32 {
    a + b  // pure
}
// Function COmposition
fn double(x: i32) -> i32 { x * 2 }
fn add_three(x: i32) -> i32 { x + 3 }

let composed = |x| add_three(double(x));
println!("{}", composed(5)); // 13

// Curring

fn add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

let add5 = add(5);
println!("{}", add5(3)); // 8

// Closure

let factor = 2;
let multiply = |x| x * factor;
println!("{}", multiply(3)); // 6

// Lazy Evaluation
let nums = 1..; // infinite range
let squares: Vec<_> = nums.take(5).map(|x| x * x).collect();

// List Comprehension
let squares: Vec<_> = (0..5).map(|x| x * x).collect();

// Enum
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}
// switch
let x = 2;
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("something else"),
}

//pattern matching
enum Option<T> {
    Some(T),
    None,
}

let value = Some(42);

match value {
    Some(v) => println!("Got {}", v),
    None => println!("Nothing"),
}
