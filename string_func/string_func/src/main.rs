fn main() {
    println!("Hello, world!");
    let _s = String::from("hello");
    string_func_print(_s); //if clone it works
  //  println!("{}", _s); //error because _s is moved ownership need to be implemented

   

let mut counter = 0; //initialise default

let result =loop {  //loop returns value if used to initialise a variable
    counter += 1;
    println!("{}", counter);
    if counter == 10 {
        break counter * 2;
    }


    for number in 1..9 {
        println!("{}", number);
    }
//--------------counter up and down------------
/* 
   'counting_up: loop {
        println!("counting up");
        break 'counting_up;
        counting_up += 1;
       
        counting_down:loop {
            countering_down -= 1;
            if countering_down == 0 {
                break 'counting_up;          //can use break 'counting_up or break counting_up can use contimnue
            }

        } */

       
    
};


println!("The result is {}", result);


}

fn string_func_print(mut _s: String) {
    _s.push_str("string");
    println!("{}", _s);

}


   
