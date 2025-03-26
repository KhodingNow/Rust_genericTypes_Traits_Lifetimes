// A definition of a public 'Summary' trait that expresses this beahvior (..media aggregator displaying summaries of data..)

    pub trait Summary {
        fn summarize(&self) -> String;
    }

// Here, we daclare a trait using the 'trait' keyWord and then the trait's name, which is 'Summary' in this case.
// We also declare the trait as 'pub' so that crates depending on this crate can make use of this trait too as we'll see.
// Inside the curly brackets, we declare the method signatures that describe the behaviours of the types that implement this trait, which in this case is 'fn summarize(&self) -> String;'


//  Implementing a Trait on a Type;

// Now that we have defined the desired signatures of the 'Summary' trait's methods, we can implement it on the types in our media aggregator.
// Bellow is an implementation of the "Summary" trait on the "Newsarticle" struct that uses the headline, the author, and the location to create the return value of 'summarize'.
// For the Tweet struct, we define 'summarize' as the username followed by the entire text of the tweet, assuming the tweet content is already  limited to 280 characters.

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!(" {}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

// Implementing a trait on a type is similar to implementing regular methods. The difference is that after the 'impl', we put the trait name we want to implement, then use the 'for' keyword, and then specify 
// the name of the type we want to implement the trait for. Within the 'impl' block
// we put the method signatures that the trait definitions has defined.
// Instead of a semicolon after each signature, we use curly brackets and fill in thge method body with the specific bahaviour that we want the methods of the trait to have for the particular type.

// Now that the library has implemented the 'Summary' trait on 'NewsArticle' and 'Tweet', users of the crate can call the trait methods on instances of 'NewsArticle' and 'Tweet' in the same waywe call regular methods.
// The only difference is that the user must bring the trait into scope as well as the types.

//EXAMPLE:

    use aggregator::{Summary, Tweet};

    fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        printlin!("1 new tweet: {}", tweet.summarize());
    }