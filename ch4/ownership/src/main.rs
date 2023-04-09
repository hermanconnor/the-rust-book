fn main() {
    // *** OWNERSHIP RULES ***
    // Each value has an owner
    // There can only be 1 owner at a time
    // When the owner goes out of scope, the value will be dropped

    // *** VARIABLE SCOPE ***
    // A scope is the range within a program for which an item is valid.

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // Do Stuff with s
        println!("s is: {s}");
    } // this scope is now over, and s is no longer valid

    // *** THE STRING TYPE ***

    // This kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // *** MEMORY AND ALLOCATION ***
    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    //
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    //
    // That first part is done by us:
    // when we call String::from, its implementation requests the memory it needs.

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.

    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    let s1 = String::from("hello");
    let s2 = s1;

    // ERROR
    // println!("{}, world!", s1);

    // In this example, we would say that s1 was 'moved' into s2
    // Because Rust invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    // Rust will never automatically create “deep” copies of your data.
    // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    // *** CLONE ***
    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // STACK-ONLY DATA: COPY
    // This code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack.
    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    //
    // So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
    // All the integer types, such as u32,
    // The Boolean type - bool, with values true and false.
    // All floating-point types, such as f64.
    // The char type.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // *** OWNERSHIP AND FUNCTIONS ***
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // Passing a variable to a function will move or copy, just as assignment does.
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...

        let x = 5; // x comes into scope.

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    // *** RETURN VALUES AND SCOPE ***
    // Returning values can also transfer ownership.
    // The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
    // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    //Rust does let us return multiple values using a tuple
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope - 1 line above actually

    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope -1 line above actually
    println!("{}", some_integer)
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
