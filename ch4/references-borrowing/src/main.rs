fn main() {
    // *** REFERENCES AND BORROWING ***

    // The issue with the tuple code in the previous lesson is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length.
    // Instead, we can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

    // Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println!("The length of {} is {}.", s1, length);

    // These ampersands(&) represent references, and they allow you to refer to some value without taking ownership of it.

    // *** MUTABLE REFERENCES ***
    // Like variables, References are IMMUTABLE BY DEFAULT

    let mut s = String::from("hello");

    change(&mut s);

    // We cannot borrow s as mutable more than once at a time.
    // let r1 = &mut s;
    // let r2 = &mut s;

    // ERROR
    // println!("{}, {}", r1, r2);

    // THIS WORKS
    // we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{r2}");

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    {
        // let mut s = String::from("hello");

        // let r1 = &s; // no problem
        // let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM

        // println!("{}, {}, and {}", r1, r2, r3);
    }

    // Multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    {
        let mut s = String::from("hello");

        let r1 = &s; // No problem, not &mut
        let r2 = &s; // No problem not &mut
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // No problem
        println!("{}", r3);
    }
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
