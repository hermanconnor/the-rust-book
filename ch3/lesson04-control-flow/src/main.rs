fn main() {
    // *** if EXPRESSION ***
    let number = 3;

    if number < 5 {
        println!("Condtition was true!");
    } else {
        println!("Condition was false!");
    }

    // The if condition MUST BE A BOOL
    // No truthy or falsy like javascript
    // let number2 = 3;
    // WILL CAUSE AN ERROR
    // if number2 {
    //     println!("Number 2 is 3");
    // }

    let number3 = 3;

    if number3 != 0 {
        println!("Number was something other than zero");
    }

    // else if
    let num = 6;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    // 'if' IN A 'let' statement
    // if is an expression, so we can use it on the right side of a let statement

    let condition = true;
    let num2 = if condition { 5 } else { 6 };

    println!("The value of num2 is: {num2}");

    // The whole if expression depends on which block of code executes.
    // So, the values that have the potential to be results from each arm of the if must be the same type;
    // let condition = true;

    // CAUSES AN ERROR - integer and string are not same type
    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    // *** loop ***

    // You might need to pass the result of an operation out of the loop to the rest of your code.
    // To do this, you can add the value you want returned after the break expression you use to stop the loop;
    // that value will be returned out of the loop so you can use it, as shown here:
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; //  After the loop, we use a semicolon to end the statement that assigns the value to result.

    println!("The result is {result}");

    // LOOP LABELS

    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    // You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    // Loop labels must begin with a single quote. Here’s an example with two nested loops:

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
    // The outer loop has the label 'counting_up: and it will count up from 0 to 2.
    // The inner loop without a label counts down from 10 to 9.
    // The first break that doesn’t specify a label will exit the inner loop only.
    // The break 'counting_up; statement will exit the outer loop.

    // *** CONDITIONAL LOOPS WITH 'while' ***
    // While a condition evaluates to true, the code runs; otherwise, it exits the loop.

    let mut count_down = 3;

    while count_down != 0 {
        println!("{count_down}!");

        count_down -= 1;
    }

    println!("LIFT OFF!!!");

    // *** LOOP THROUGH A COLLECTION WITH 'for' ***

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // Using for with a range and reverse example:
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
