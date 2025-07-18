use std::vec;
#[warn(unused_imports)]
use std::collections::HashMap;
#[derive(Debug)]
enum Game{
    Rock,
    Paper,
    Scissors,
}


#[derive(Debug)]
enum Animal {
    Dog(String,u32),
    Cat(String, u32),
    fish(String, u32),
}

#[derive(Debug)]
struct Base<T> {
    value: T,
}


fn main() {
 let mut vec_static = vec![
    "jan",
    "feb"];

 vec_static.push("mar");
 vec_static.push("apr");
vec_static.push("may");
 vec_static.push("jun");
 vec_static.push("jul");
 vec_static.push("aug");
    vec_static.push("sep");
    vec_static.push("oct");
    vec_static.push("nov");

    vec_static.push("dec");
 println!("{:?}", vec_static);

 let mut days: Vec<&str> = vec::Vec::new();

days.push("mon");
    days.push("tue"); 
    days.push("wed");
    days.push("thu");
    days.push("fri");
    days.push("sat");
    days.push("sun");

 //println!("{:?}", days);
let mut count = 0;
 for ind in &days{
    count += 1;
    println!("{:?} {:6}", ind, count);
 }

 days.remove(5);
 days.remove(0);
 println!("{:?}{:6}", days,count);

 let mut month_dynamic =vec![String::from("jan"),
    String::from("feb"),
    String::from("mar"),
    String::from("apr"),
    String::from("may"),
    String::from("jun"),
    String::from("jul"),
    String::from("aug"),
    String::from("sep"),
    String::from("oct"),
    String::from("nov"),
    String::from("dec")];
 println!("{:?}", month_dynamic);

month_dynamic.remove(10);
//------------Hashmap-----------------------------------


let mut list:HashMap<i32, String>= HashMap::new();
println!("Hashmap{:?}", list);

list.insert(1, String::from("one"));
list.insert(2, String::from("two"));
list.insert(3, String::from("three"));
list.insert(4, String::from("four"));
list.insert(5, String::from("five"));
list.insert(6, String::from("six"));


println!("Hashmap{:#?}", list);

let _first = list.get(&1);
let _second = list.get(&2);
let _third = list.get(&3);

list.insert(2, String::from("ten"));
print!("Hashmap{:#?}\n", list);


let mut month:HashMap<i32, String> = HashMap::new();


month.insert(1, String::from("jan"));
month.insert(2, String::from("feb"));
month.insert(3, String::from("mar"));
month.insert(4, String::from("apr"));
month.insert(5, String::from("may"));
month.insert(6, String::from("jun"));
month.insert(7, String::from("jul"));    
month.insert(8, String::from("aug"));   
month.insert(9, String::from("sep"));
month.insert(10, String::from("oct"));
                                                   //hashmap only two different types
let mut count = 0;
for (key, value) in month.iter() {
    count += 1;
    println!("value:{} & Key:{}", value, key);
}
// Extract key-value pairs into a Vec




let mut sorted_data: Vec<_> = month.iter().collect();


sorted_data.sort_by_key(|&(key, _)| key);



// Result: [("apple", &3), ("banana", &2), ("cherry", &5)]
for (key, value) in sorted_data {
    println!("{}: {}", key, value);
}

fun_month(month);

//-----enum------------------------------------------

let rck = Game::Rock;
let pap = Game::Paper;
let scs = Game::Scissors;
println!("Game: {:?}, {:?}, {:?}", rck, pap, scs);

func_game(rck);


let dog = Animal::Dog(String::from("Buddy"), 5);
let cat = Animal::Cat(String::from("Whiskers"), 3);
//=======pattern matching match=========================
let x = 5;
match x {
    1 => println!("x is 1"),
    2 => println!("x is 2"),
    _ => println!("x is something else"), //_ default case
}

let my_game = Game::Rock;
match my_game {
    Game::Rock => println!("You chose Rock!"),
    Game::Paper => println!("You chose Paper!"),            
    Game::Scissors => println!("You chose Scissors!"),

    // You can add more cases if needed     
}

//----------option type-------------------
//multiple options for values, can be used to handle absence of value
//sgnals and sensor data readin
let my_option: Option<i32> = Some(222);
let my_option2: Option<&str> = Some("fuckoff man");

match my_option {
    Some(value) => println!("The value is: {}", value),
    None => println!("No value found"),    
}

match my_option2 {
    Some(value) => println!("The value is: {}", value),
    None => println!("No value found"),    
}

match divide(10, 2) {
    Some(result) => println!("Result: {}", result),
    None => println!("Division by zero!"),
}   

match divide(10, 0) {
    Some(result) => println!("Result: {}", result),
    None => println!("Division by zero!"),
}



let my_result: Result<i32, &str> = Ok(42);

match my_result {
    Ok(value) => println!("The result is: {}", value),
    Err(error) => println!("Error: {}", error), 

}

}

//this is best method to handle division by zero
// It returns an Option type, which can be Some(result) or None if division by zero
// This is useful to avoid panics in case of division by zero
// It is a good practice to use Option type for operations that can fail
// This way, you can handle the error gracefully without crashing the program
// This is a common pattern in Rust to handle operations that can fail
// It allows you to return a value or indicate that the operation failed
// This is a good way to handle errors in Rust
fn divide (x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None // Return None if division by zero
    } else {
        Some(x / y) // Return Some with the result
    }
}   

enum DivisionError {
    DivisionByZero,
    OtherError(String),
    
}

fn Divide( x: i32, y: i32) -> Result<i32, DivisionError> {
    if y == 0 {
        Err(DivisionError::DivisionByZero) // Return an error if division by zero
    } else {
        Ok(x / y) // Return Ok with the result
    }
}

fn func_game(game: Game) {
    match game {  //similar to switch case
        Game::Rock => println!("You chose Rock!"),
        Game::Paper => println!("You chose Paper!"),
        Game::Scissors => println!("You chose Scissors!"),
    }
}


fn fun_month(month: HashMap<i32, String>) {
    println!("Function month: {:#?}", month);
    // You can iterate over the HashMap here if needed
    for (key, value) in &month {
        println!("Key: {}, Value: {}", key, value);
    }
    // If you want to return something, you can do so here
    // For example, returning the count of items
    let count = month.len();
    println!("Count of items in month: {}", count);
}

//println!("Hashmap{:#?}", month);


