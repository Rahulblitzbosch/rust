




fn main() {
    println!("Hello, world!");
    //write functions here
    // This is a placeholder for the main function.
    //write your code here
    // You can implement the logic for your application here.
    // For example, you might want to call other functions or modules.
    // This is a simple example of how you might structure your main function.
    // You can also include any necessary imports or module calls here.
    //how to write functions in Rust
    // You can define functions above or below the main function.
    // Functions can be defined using the `fn` keyword, followed by the function name and
    // parameters in parentheses. The function body is enclosed in curly braces.
  example_function(42);
 let ret:i32 = return_function(); 
    println!("Returned value: {}", ret);
}

   
   
fn return_function() -> i32 {
    // This function returns an integer value.
    let value = 10;
   // println!("Returning value: {}", value);
    value
}

fn example_function(val: i32)  {
        // This is a simple function that takes an integer parameter and returns it.
        println!("The value is: {}", val);
    }
