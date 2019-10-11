
#![allow(unused_variables)]
fn main() {
    var();
    data_types();
    operations();
    ask_parameter(5);
    ask_parameter_s(5,6);
    function_in_function();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    if_test();

    if_let();

    loop_test();

    ownership();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

    let _s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1_reference = String::from("hello");

    let len_reference = calculate_length_reference(&s1_reference);

    println!("The length of '{}' is {}.", s1_reference, len_reference);

    let mut s = String::from("Hello");

    change(&mut s);

    test_mut_pointer();

    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    slicing();

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let _word = first_word_sliced(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word_sliced(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_sliced(my_string_literal);

    test_struct();

    rectangle();

    test_methods();

    use_enum();

    value_in_cents(Coin::Penny);

    match_option();

    if_let_improved();

    define_match(Coin::Quarter(UsState::Alabama), UsState::Alabama);

    define_if_let(Coin::Quarter(UsState::Alabama), UsState::Alabama);

    vectors();

    manage_string();

    manage_hash_map();

    test_crash();

    test_errors();
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

fn var(){
    // Trying variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Trying shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2 ;

    println!("The value of y is: {}", y);

    // Understanding the difference between shadowing and mutable
    let spaces = "    ";
    let spaces = spaces.len();
    // Does not work
    // let mut spaces = "     ";
    // spaces = spaces.len();

    println!("Number of spaces: {}", spaces);
}

fn data_types(){
    let _guess: u32 = "42".parse().expect("Not a number");
    // let guess = "42".parse().expect("Not a number"); // Does not work

    // Testing float
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Testing boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Testing character type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Testing compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple
    let (_a, b, _c) = tup;

    println!("The value of b is: {}", b);

    println!("The value the third element is: {}", tup.2);

    // Testing array
    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let _array: [i32; 5] = [1, 2, 3, 4, 5]; // Define the type and number of the elements
    let _array = [3; 5]; // Define an array a 5 elements with value 3
    let _first = _array[0];
    let _second = _array[1];
}

fn operations() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;
}

fn ask_parameter(x: i32){
    // A function with one parameter
    println!("The value of x is: {}", x);
}

fn ask_parameter_s(x: i32, y: i32){
    // A function with two parameters
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_in_function(){
    // A function using function body
    let _x = 5;

    // Declaring a var according to another one
    let y = {
        let _x = 3;
        _x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    // A function which returns an i32
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // Doesn't work
    // x + 1;
}

fn if_test(){
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let(){
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // Doesn't work cause not same type
        // "six"
    };

    println!("The value of number is: {}", number);
}

fn loop_test(){
    // Infinite loop
    // loop{
    //     println!("Again");
    // }

    // Loop test
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While test
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For test
    let a  = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("The value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// Test ownership()
fn ownership(){
    let mut s = String::from("Hello");

    s.push_str(" ,world"); // push_str() appends a literal to a String

    println!("{}", s);

    // Interactions
    let x = 5;
    let y = x; // y <- 5, copy the value

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("Hello");
    let _s2 = s1; // Points to the same memory location, doesn't create another String

    // println!("{}, world", s1); // Doesn't work

    let s3 = String::from("Hello");
    let s4 = s3.clone(); // Clone the value

    println!("s1 = {}, s2 = {}", s3, s4);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn test_mut_pointer(){
    let mut s = String::from("Hello");

    let r1 = &s; // No problem
    let r2 = &s; // No problem
    println!("{} and {}", r1,r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // No problem
    println!("{}", r3);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn slicing(){
    let s = String::from("Hello");

    let len = s.len();

    let slice = &s[3..len];
    println!("s[3..len]: {}", slice);
    let slice = &s[3..];
    println!("s[3..]: {}", slice);
    let slice = &s[0..len];
    println!("s[0..len]: {}", slice);
    let slice = &s[..];
    println!("s[..]: {}", slice);
}

fn first_word_sliced(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn test_struct(){
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let _user3 = build_user(String::from("another@example.com"),
    String::from("anotherusername567"));
}

#[derive(Debug)] // For printing the struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn rectangle(){
    // Use traditional way
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Try Tuple
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tup(rect1)
    );

    // Use struct
    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    println!("rect2 is {:?}", rect2);

    // Using a method
    let rect3 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
}

fn test_methods(){
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated functions
    let sq = Rectangle::square(3);

    println!("sq is a square of size {:?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrImproved {
    V4(String),
    V6(String),
}

enum IpAddrDefined {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn use_enum(){
    // Using enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Using a method for enum
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using enum inside struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Using improved enum
    let home = IpAddrImproved::V4(String::from("127.0.0.1"));

    let loopback = IpAddrImproved::V6(String::from("::1"));

    // Using a more defined IpAddr
    let home = IpAddrDefined::V4(127, 0, 0, 1);

    let loopback = IpAddrDefined::V6(String::from("::1"));

    // None in enum
    let some_number = Some(5);
    let some_string = Some("a string");

    // Option<T> test
    let absent_number: Option<i32> = None;
}

// enum with multiple types inside
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn route(ip_kind: IpAddrKind) { }

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


fn match_option() {
    // Basic match option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // If removed, can't work because not exhaustive
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // '_' placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // Match any value
    }
}

fn if_let_improved(){
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn define_match(coin: Coin, state: UsState){
    // Define according to match
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn define_if_let(coin: Coin, state: UsState){
    // Define according to 'if let'
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn vectors(){
    // Empty vector
    let v: Vec<i32> = Vec::new();

    // Create a non-empty vector
    let v = vec![1, 2, 3, 4, 5];

    // Create a mut vector and update it
    let mut v_mut = Vec::new();

    v_mut.push(6);
    v_mut.push(7);
    v_mut.push(8);
    v_mut.push(9);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Doesn't work
    // let twenty: i32 = &v[19]; // It panics
    // let twenty = v.get(19); // It returns None

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


    let first = &v_mut[0];

    // Doesn't work because v is hold
    // v_mut.push(6);

    println!("The first element is: {}", first);

    // Works because v is not hold
    v_mut.push(6);

    // Iterating over the values
    for i in &v_mut{
        println!("{}", i);
    }

    // Increasing the values of v_mut by 50
    for i in &mut v_mut{
        *i += 50;
    }

    // Use enum for storing values
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("Blue")),
    SpreadsheetCell::Float(10.12),
    ];

    // do stuff with v
} // <- v goes out of scope and is freed here

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn manage_string(){
    let mut s = String::new();

    let data = "Initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "Initial contents".to_string();

    let s = String::from("Initial contents");

    // String are UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Update String
    let mut s = String::from("foo");
    s.push_str("bar");

    // Concat Strings
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Add just one char
    let mut s = String::from("lo");
    s.push('l');

    // Concat with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // fn add(self, s: &str) -> String {

    // Concat multiple String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // Complex concat
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Iterating over Strings getting chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Iterating over Strings getting bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

fn manage_hash_map(){

    // Creating basic HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // Creating HashMap from Vectors
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Create HashMap by inserting values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Getting the value of a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating over Key
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}


fn test_crash(){
    // Program will panic, return the msg and stop
    // panic!("Crash and burn");

    let v = vec![1, 2, 3];

    // Will panic
    // v[99];
}

use std::fs::File;
use std::io::ErrorKind;

fn test_errors(){
    let f = File::open("hello.txt");

    // Doesn't work because u32 not in the Result
    // let f: u32 = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Simplifed block from all above
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcut for Panic on Error using unwrap
    let f = File::open("hello.txt").unwrap();

    // Shortcut for Panic on Error using expect
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

use std::fs;

fn read_username_from_file_improved() -> Result<String, io::Error> {
    //// Slighlty improved
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Chained
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // Ultimately improved
    fs::read_to_string("hello.txt")
}

use std::error::Error;

fn read_username_from_file_dynamic() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
