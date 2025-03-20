fn main() {
    // Removing duplication by extracting a function
    // - Generics allows us to replace specific types with a placeholder that represents multiple types to reomove code duplication
    // First - lets look at how to remove duplication in a way that does'nt involve generic types  y extracting a functionthat replaces specific values with a placeHolder
    // that represents multiple values.
    
    let numer_list = vec! [34, 50, 25, 100, 65];

    let mut largest = &number_list [0];    

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }
    
    printlin!("The largest number is {largest}");
// we store a list of integers in the variable 'number_list'and place a reference to the first number in the list
// in a variable named 'largest'. We then iterate through all the numbers in the list, and if the current number is greater than the number stored 
// in 'largest', we replace the reference in that variable. 
// However, if the current number is less than or equal to the largest number seen so far, the variable doesn't change , and the code moves on to the next number in the list.
// After considering all the numbers in the list, 'largest' should refer to the largest number, which is 100. 

// Please find the largest number in two different lists:

let numer_list = vec! [34, 50, 25, 100, 65];

let mut largest = &number_list[0];   
    
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    printlin!(".......{largest}")

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        
        if number > largest {
            largest = number;
        }        
    }
    printlin!("The largets number is {largest}");
} // Although this code works, duplicating code is tedious and error prone. We also have to remember to update it in multiple places when we want to change it.
// To eliminate this duplication, we'll create an abstraction by defining a function that operates on any list of integers passed as parameters. This solution makes the code clearer and lets us express the concept of finding the largest number in the list abstractly.



 

