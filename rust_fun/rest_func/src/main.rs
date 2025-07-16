fn main() {
    println!("Hello, world!");
    let _val:i32 = 223;
    another_function();
    another_function_parameter(5);
//---------condition---------------------------------
   /*  if _val  //not possible since defaul use boolean
    {
println!("true");
    }
    else
    {
        println!("false");
    } */

    if _val<300
    {
        println!("true");
    }

    if _val%2==0
    {
        println!("even");
    }

    let _numb:i8 = if _val>0 {10} else {20};  //initislisation based on condition can be used with string 
    println!("The value of _numb is: {}", _numb);

let _strin: &str  = if _val>0 {"true sys"} else {"false sys"};   //not goodway
println!("The value of _strin is: {}", _strin);

    //-----------------------------------------------------------------------


   
    let _new_ret= another_function_return('a',_val); 
    print!("calling function inside {}",call_func()); //return type printing inside call by value
    println!("The value of new_ret is: {}",_new_ret); 
    let ret:bool = even_func(4);
    if ret{                 //default condition is bool based different from 
        println!("even");
    }
    else {
        println!("Odd");
    }


let _arr:[f32;3]=[1.3,55.3,5.3];

println!("the value is {:.2}",array_fun(_arr));  //:.2 is for decimal resolution



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

pub fn even_func(x:i32)->bool{
    let x = x&1;
    if x==1{
        return false;
    }
    else {
       return true; 
    }
   
}


fn array_fun(_arr:[f32;3])->f32 {
    return (_arr[0]+_arr[1]+_arr[2])/3.0;
}