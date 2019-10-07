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
}

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
