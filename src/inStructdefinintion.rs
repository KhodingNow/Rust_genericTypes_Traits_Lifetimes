// We can also define structs to use a generic type parameter in one or more fields using  '<>' syntax.
// Below, we define a 'Point<T>' struct to hold 'x and 'y' coordinate values of any type.

struct Point<T> {
    x:T,
    y:T,
}

fn main() {
    let integer = Point {x: 6, y: 12};
    let float = Point {x: 1.7, y: 3.2};
} // The syntsx for using generics in struct definitions is similar to that used in function definitions.
// Firs, we declare the name of the type parameter inside the angle brackets just after the name of the struct
// Then we use the generic type in the struct definition where we would otherwise specify concrete data types.

//NOTE:

// - Because we have used only one generic type to define 'Point<T>', this definition says that the 'Point<T>' struct is generic over some type 'T', and the fields 'x' and 'y' are BOTH that same type, 
// - whatever that type may be. If we crate an instance of a 'Point<T>' that has values of different types - our code won't compile.

// E.G, 

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 7.4};
}
// In this case, when we assign two differnt data type to type 'T' - an interger and a float, we will have a mismatch and an error.

// To define a Point struct where 'x' and 'y' are both generics but cold have different types, we can use multiple generic type parameters. We change the definition of 'Point' to be generic over types 'T' and 'U' where x is of type "T" and y is of type "U".

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 6, y: 9 };
    let both_float = { x: 1.0, y: 3.9 };
    let inter_and_float = Point { x: 5, y: 1.1 };

} // all the instances of 'Point' above are allowed - BUT don't use all at once, otherwise you may need to re-structure youjr code. All at once, makes the code hard to read.

