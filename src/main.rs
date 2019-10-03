fn main() {
    var();
    data_types();
    ask_parameter(5);
    ask_parameter_s(5,6);
    function_in_function();

    let x = five();
    println!("The value of x is: {}", x);
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
