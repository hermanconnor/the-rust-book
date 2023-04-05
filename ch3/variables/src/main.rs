fn main() {
    // VARIABLES - by default variables are immutable

    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of is is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of is is: {x}");

    // CONSTANTS - are not allowed to change, CAN'T USE mut with constants, ALWAYS immutable, type of the value MUST BE annotated

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // SHADOWING
    let a = 5;

    let a = 5 + 1; // Overshadows the first 'a' variable

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");

    // We are effectively creating a new variable when we use the let keyword again
    // Therefore, we can change the type of the value
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type

    println!("Number of spaces is: {spaces}")
}
