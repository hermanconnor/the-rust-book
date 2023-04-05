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
    // As you saw in the guessing game tutorial in Chapter 2, you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
    let a = 5;

    let a = 5 + 1; // Overshadows the first 'a' variable

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");

    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type

    println!("Number of spaces is: {spaces}")
}
