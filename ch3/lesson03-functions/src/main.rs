fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // STATEMENTS AND EXPRESSIONS
    // Statements - instructions that perform some action and don't return a value.
    // Expression - evaluates to a resultant value.

    // Statement
    // let y = 6;

    // Expression - result in a value
    // Calling a function is an expression.
    // Calling a macro is an expression.
    // Do not include ending semicolons.
    // A new scope block created with curly brackets is an expression, for example:
    let y = {
        let x = 3;
        x + 1 // no ending semicolon.
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

// *** FUNCTIONS ***
// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

// If a function has a parameter,
// you MUST declare the type of each parameter.
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// FUNCTIONS WITH RETURN VALUES
// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the 'return' keyword and specifying a value, but most functions return the last expression implicitly.

fn five() -> i32 {
    // No semicolon after returns the value 5
    5
}

fn plus_one(x: i32) -> i32 {
    // placing a semicolon after will cause an error
    x + 1
}
