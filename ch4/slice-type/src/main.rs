fn main() {
    // *** SLICES ***

    // Slices let you reference a contiguous sequence of elements rather than the whole collection

    // A string slice is a reference to part of a String
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {hello} and world = {world}");

    // if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
    {
        let s = String::from("hello");

        let slice = &s[0..2];
        let slice = &s[..2];
    }

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    {
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[3..len];
        let slice = &s[3..];
    }

    // You can also drop both values to take a slice of the entire string. So these are equal:
    {
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
    }

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // OTHER SLICES

    // Just as we might want to refer to part of a string, we might want to refer to part of an array. We’d do so like this:
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    // Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes();

    // Next, we create an iterator over the array of bytes using the iter method:
    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple instead.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
