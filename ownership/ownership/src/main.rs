/*Allocate some thing on heap need to free Dynamic

Anything on heap has owner
Ans when heap is there is only 1 owner*/

/*Mutable referece cant be used simultanious reference after mutable cant use immutable reference ….its called thread safe

Mutable reference can be used only once 
Prevent race condition --two pointers running 
Cant have multiple mutable referece of same variable
Multiple immutable reference is possitble


Non mutable reference can be used where data is not chnaging


 */



fn main() {
    let mut _str:String =String::from("Rahul"); //dymaic think about ownership in heap
   let _str2:&str="krishnan"; //static string save in stack

  //  let mut _str2=_str.clone();  //use clone for ownership 

    println!("{}",_str); //mask this to remove ownership error
  //  println!("{}",_str2);

//    func_dynamic(_str);
   // println!("{}",_str); //ownership error

   func_static(_str2);
   println!("{}",_str2); //no ownership error 

let _s1 = String::from("hello");

let(_s1,_s2)= funct_calc(_s1);
println!("{} {}",_s1,_s2);

let _s3 = String::from("string in func");

let _s4:  String=str_func(_s3.clone()); //same as C code heap size doubled not good
print!("{}",_s3);
println!("{}",_s4);
}

fn str_func(mut _s3:String)->String{
    println!("{}",_s3);
    _s3.push_str(" justcheck ");
    _s3
}

fn funct_calc(_s1:String)->(String,u64){
    let _s2:u64=_s1.len() as u64;
    return (_s1,_s2);
}


fn func_dynamic(_str:String){

}



fn func_static(_str2:&str){

}
