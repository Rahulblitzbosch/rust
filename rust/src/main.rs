

use std::io;//for inputs and outputs


fn main() {
//---------------int----------------------------------------
    let mut x: i32 = -200;   //if use u32 then it will throw error bevusde of sign
    println!("{}",x);   //{} is variable
    println!("x={}",x);   //{} is variable C
    println!("{x}");  //another way to print
    println!("x={x}"); //another way to print C++
    println!("Hello, world!");  //its macro not library
    x=30;
    println!("after mute x={x}");
let  y: i32 = 500;
let  z : i32 = x+y; //you 

println!("z={z}");
//-----------------------boolean----------------

let _is_a:bool =true;
let _is_b:bool=false;

if _is_a
{
    println!("true its a");
}
else {
    println!("flase not a");
}
//----------------string--------------------

let mut str  : &'static str = "hello bosch"; //not good C++ implicit 
println!("{str}");

str ="not bosch here";

println!("{str}");


let mut _nw_i  = String ::from("new"); //explicit 
println!("{_nw_i}");
_nw_i.push_str("Less new"); //for string just push for &str cocantinate string
_nw_i.push_str(str); //concatinate strinh
//str.push("only in case of &"); //dont use &str because it wont work

println!("{str}");

println!("{_nw_i}");

let mut _guess:String = String::new();
io::stdin().read_line(&mut _guess).expect("error");   //like scanf 
println!("guess={_guess}");

  example_function(42);
 let  ret:i32 = return_function(); 
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
