fn main() {
    println!("Hello, world!");
    let _val:i32 = 223;
    another_function();
    another_function_parameter(5);


   
    let _new_ret= another_function_return('a',_val); 
    print!("calling function inside {}",call_func()); //return type printing inside call by value
    println!("The value of new_ret is: {}",_new_ret); 
    let ret:bool = even_func(4);
    if ret{
        println!("even");
    }
    else {
        println!("Odd");
    }

}
//---------stack allocation and deallocation memory management----------------------
fn another_function() {
    println!("Another function.");
}
//function passing function cant use let 
 fn another_function_parameter(x: u32 ) {
    println!("The value of x is: {}", x);
}
//function with return
fn another_function_return(_pas_chr:char,_val:i32)->i32 {
    println!("Another function with return.char {_pas_chr},{_val}");
   return 10; // or just 10

}

fn call_func()->i32{
    println!("calling function inside");
    22
}

fn even_func(x:i32)->bool{
    let x = x&1;
    if x==1{
        return false;
    }
    else {
       return true; 
    }
   
}