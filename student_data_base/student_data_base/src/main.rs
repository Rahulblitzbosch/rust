
//name , rollno , marks,grade(proportional to marks eg: >60 -c , >70 -b , >80 -a, >90 >E)

fn main() {
    println!("Hello, world!");
    let tup:(&'static str,i32 ,i8) = ("rahul",33443,80);

  
    let (_string , _rollno , _marks) = tup;

    if tup.2>60 {
        println!("{} , {} , {} , {}" ,tup.0 , tup.1 , tup.2 , "c");
    }
    else if tup.2>70 {    
        println!("{} , {} , {} , {}" ,tup.0 , tup.1 , tup.2 , "b");
    }
    else  if tup.2>80 {
        println!("{} , {} , {} , {}" ,tup.0 , tup.1 , tup.2 , "a");
    }
    else if tup.2>90 {
        println!("{} , {} , {} , {}" ,tup.0 , tup.1 , tup.2 , "E");
    }
    else {
        println!("{} , {} , {} , {}" ,tup.0 , tup.1 , tup.2 , "F");
        
    }
  //  tup_print(tup);
}
