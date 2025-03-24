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
// We could have chosen a different name for this generic parameter than the generic parameter declared in the struct definition, but using the same name is conventional.
// If you write a method within an 'impl' that declares a generic type , that method will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.

// We can also specify constraints on generic types when defining methods on the type. We could, for example, implement methods only on 'Point<f32>' instances rather than on 'Point<T>' instances with any generic type.   
// We can use the concrete type 'f32' - meaning we don'y declare any types after 'impl':

    impl Point<f32> {
        fn distance_from_origin(&self) -> {
            (self.x.powi(2) + self.y.powi(2)).sqrt()

        }
    }
    // This code means the type 'Point<f32>' will have a 'distance_from_origin' method; other instances of 'Point<T>' where 'T' is not of type 'f32' will not have this method defined.
    // The method measures how far our point is from the point at coordinates (0.0), (0.0) and uses mathematical operations that are available only for floating point types.
    // Generic type parameters in a struct definition aren't always the same as those you usein that struct method signatures.
    // Below, we use generic types 'x1' and 'y1' for the Point struct and 'x2' and 'y2' for the mixup method signature to make the example clearer.
    // The method creates a new Point instance with the 'x' value from the 'self' Point (of type x1) and the y value from the passed -in Point (of type Y2).
    
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: self.y,
                }
            }
        }

       fn main() {
            let p1 = Point { x: 5, y:10.4 };
            let p2 = Point { x: "Hello", y: 'c'};

            let p3 = p1.mixup(p2);

            println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
       }
       // In main, we've defined a Point that has an 'i32' for x(with value 5) and an f64 for 'y' (with value 10.4). 
       // The p2 variable is a 'Point' struct that has a string slice of x(with value "Hello") and a 'char' for 'y'(with value c).

       // Calling 'mixup' on 'p1' with the argument 'p2' gives us 'p3', which will have an 'i32' for 'x' because x came from 'p1'.
       // The p3 variable will have a 'char' for 'y' because 'y' came from 'p2'.
       // The 'println!' macro call will print 'p3.x = 5, p3.y = c'.
       // The purpose of this example is to determonstrate a situation in which some generic parameters are declared with 'impl' and some are declared with the method definition. Here, the generic, parameters X1 and Y2 are declared after 'impl' because they go with the struct definition.
       // The generic parameters X2 and Y2 are declared after 'fn mixup' because they're only relevant to the method.
       