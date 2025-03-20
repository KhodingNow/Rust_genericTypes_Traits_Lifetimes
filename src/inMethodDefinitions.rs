// We can implement methods on structs and enums and use generic types in their definitions too. 

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 7, y: 9};

    println!("p.x = {}", p.y());
}

// Here, we've defined a method named x on Point<T> that returns a reference to the data in the filed x.
// Note that we have to declare T just after 'impl' so we can use T to specify that we're implementing methods on the type Point<T>.
// By declaring T as a generic type after 'impl', Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
  