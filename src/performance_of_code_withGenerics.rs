// Is ther a runTIME cost when using generic parameters - you might wonder?
// The good news is that using using generic types won't make your program run any slower than it would with concrete types.
// Rust accomplishes this by performing MONOMORPHIZATION of the code using generics at compile time.
// MONO_Phization is the process of turning generic code into specific code by filling in the concrete types that are usedwhen compiled. In this process, the compiler does the opposite of the steps we used to create the generic function : the compiler looks at all the places where the generic code is called and generates code for the concrete types the generic code is called with.

// EXAMPLE:

    let integer = Some(5);
    let float = Some(5.0);

// When Rust compiles this code, it performs MONO_Phization. During that process, the compiler reads the values that have been used in 'Option<T>' instances and identifies two kinds of 'Option<T>': one is 'i32' and the other is 'f64'. As such, it expands the generic definitions specialized to 'i32' and 'f64', thereby, replacing the generic definition with the specific one
// The MONO_Phized version of the looks similar to the ff; (the compiler uses different names than what we're using here for illustraion):

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64).
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }

// The generic 'Option<T>' is replaced with the specific definition created by the compiler.
// Because Rust compiles generic code into code that specifies the type in each instance, we pay no RUNTIME cost for using generics. 
// When the code runs, it performs just as it would if we had duplicated each defeinition by hand. The process of MONO_Phization makes Rust's generics extremely efficient at runtime.
 