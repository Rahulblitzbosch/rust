

use std::{i32, io};//for inputs and outputs


fn main() {
//---------------int----------------------------------------
    let mut x: i32 = -200;   //if use u32 then it will throw error bevusde of sign
//same variable initialisation shadow method
    let _var:i32 = 345;
    println!("1st scope var={_var}");
    let _var:i32 = _var+2;
    println!("shadow 1st outer scope var={_var}");
    let _var = _var+3;
    println!("shadow 2nd outer scope var={_var}");
{
    let _var:i32 =_var+2;  //we can use shaadow variable with same name
    println!("shadow 2nd inner scope var={_var}");
}
println!("after 3rd scope outer  var={_var}");
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

    let tup: (i32,f64,u8) =(234,4.666,7);  //better method for touple 
    let(_x,_y,_z) = tup;
    println!("tuple first value={_x}");
    println!("  tuple second value={_y}");
    println!("tuple third value={_z}");

    println!("tuple firsr value={0}",tup.0);

//---------------array------------------

let _arr = [1,2,3,4,5,6];
println!("ist element={}",_arr[1]);

let arr_list=["Hello arr","Hello arr1","Hello arr2","Hello arr3","Hello arr4"]; //imitialised five array with string
println!("ist element={}",arr_list[1]);
println!("ist element={}",arr_list[2]);
println!("ist element={}",arr_list[3]);

let _score:[f64;3]=[2.4,54.6,3.6];
println!("ist element={}",_score[1]);
println!("ist element={}",_score[2]);
println!("ist element={}",_score[3]);





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
