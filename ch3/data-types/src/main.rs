fn main() {
    // *** FLOATING POINT NUMBERS ***(f32 and f64) - Numbers with decimal points
    // f64 is the default. All floating-point types are signed.

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x is {x} and y is {y}");

    // NUMERIC OPERATIONS
    // Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. Integer division rounds down to the nearest integer. The following code shows how you’d use each numeric operation in a let statement:

    // addition
    let sum = 5 + 10;
    println!("sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product is {product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = -5 / 3; // Results in -1
    println!("quotient is {quotient} and floored is {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {remainder}");

    // *** BOOLEAN *** Type(bool) - true or false values, 1byte in size
    let t = true;
    println!("t is {t}");

    let f: bool = false; // with explicit type annotation
    println!("f is {f}");

    // *** CHAR *** Type - 4bytes in size
    // Specify char literals with SINGLE QUOTES.
    //  Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    // Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eye_emoji = '😍';
    println!("c is {c}, z is {z}, emoji is {heart_eye_emoji}");

    // *** TUPLE *** Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

    let (x, y, z) = tup;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");

    // *** ARRAY *** Type
    // Unlike a tuple, every element of an array MUST HAVE THE SAME TYPE.
    // Arrays in Rust have a fixed length.
    // Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array.

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

    let b = [3; 5]; // same as let b = [3, 3, 3, 3, 3];
    println!("b = {:?}", b);

    // Accessing Array Elements
    // You can access elements of an array using indexing, like this:

    let first = a[0];
    let second = a[1];
    println!("first is {first}, second is {second}");
}
