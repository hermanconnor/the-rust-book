// DERIVED TRAITS
// Rust includes functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct: #[derive(Debug)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // REFACTORING WITH TUPLES
    let rect1 = (30, 50);

    println!(
        "TUPLE EXAMPLE: The area of the rectangle is {} square pixels.",
        area_with_tuple(rect1)
    );

    // REFACTORING WITH STRUCTS - ADDS MORE MEANING
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "STRUCT EXAMPLE: The area of the rectangle is {} square pixels.",
        area_with_struct(&rect2)
    );

    // DERIVED TRAITS - to print struct
    println!("rect2 is {:?}", rect2);
    // or
    println!("rect2 is {:#?}", rect2);

    // Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
    {
        let scale = 2;
        let rect3 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect3, so we use a reference to rect3 in the next call.

        dbg!(&rect3);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance.
fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
