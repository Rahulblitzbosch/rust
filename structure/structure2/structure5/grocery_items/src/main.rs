
#![allow(unused_variables)]
#![allow(dead_code)]


struct Grocery{
    name: String,
    quantity:u32,
}


struct GroceryList {
    groceries:[Grocery;3],
}

impl GroceryList {
    fn new() -> GroceryList {
        GroceryList {
            groceries: [
                Grocery { name: String::from("Apples"), quantity: 5 },
                Grocery { name: String::from("Bananas"), quantity: 3 },
                Grocery { name: String::from("Carrots"), quantity: 4 },
            ],
        }
    }

    fn show_list(&self) {
        for grocery in &self.groceries {
          //  println!("{}: {}", grocery.name, grocery.quantity);
            println!("{:#?}{:#?}",grocery.name,grocery.quantity);
        

        }
    }   

  fn add_item(&mut self, item: &Grocery) {
        for grocery in &mut self.groceries {
            if grocery.name == item.name {
                grocery.quantity += item.quantity;
               
            }
            
        }
       
       
    //   println!("grossary iteams updated {:#?} {:#?} ", self.groceries[0].name, self.groceries[0].quantity);
     //   println!("added {} {} ", item.name, item.quantity);
      
       // println!("Total quantity of {} is now {}", item.name, self.groceries.quantity);    
    }
   
}




fn main() {

//-----------------Closure-----------------//
    // A closure is a function-like construct that can capture the environment in which it is defined
    // Closures are often used for short-lived operations or when you need to pass a function
    // as an argument to another function.









    let printsup = || {   //similar to function lambda definition closure 
        println!("This is a closure that prints a message.");
    };

    let add_onev=   |x:u32,y:u32|{ 
    let sum = x + y;
    let mult = x * y;
    return mult;
    
    };  // This is a closure that adds one to a number};


    printsup(); // Call the closure

    let result = add_onev(5, 10); // Call the closure with arguments

   // send state name as input to closure and return country

   let identify= |mut _state:String|{

    _state.push_str(" is in India");
    return _state;};

   




   let country= identify(String::from("Karnatake"));

println!("The country is: {}", country);

    //----------------------------------------------------------------------------
//-----------------slices-----slice is maintained in static ------------//


let numbers: [u32; 5] = [1, 2, 3, 4, 5];
let slice = &numbers[1..4]; // Create a slice from the array 
println!("Slice of numbers: {:?}", slice); // Output: [2, 3]

let _string_def = String::from("Hello, world!");
let _string_slive = &_string_def[1..10];
println!("String slice: {}", _string_slive); // Output: Hello, world!

let def = slice[0]; // Accessing the first element of the slice
println!("First element of the slice: {}", def); // Output: 2





    let mut new_list = GroceryList::new();
    let mut _item1: Grocery = Grocery { name: String::from("Apples"), quantity: 3 };
    let mut _item2: Grocery = Grocery { name: String::from("Bananas"), quantity: 1 };
  //  let item3: Grocery = Grocery { name: String::from("eggs"), quantity: 3 };
  println!("+-----------------Grocery List-----------------+");
    new_list.show_list();

    new_list.add_item(&_item1);
   new_list.add_item(&_item2);
    println!();
    new_list.show_list();
    //=======typecasting========
    println!("+-----------------Type Casting-----------------+");
let _char = 'A';
let intiger = _char as u32;

let _inter:u8= 65;
let nchar = _inter as char;

println!("The character '{}", nchar);

    println!("The integer value of character '{}' is: {}", _char, intiger);
    println!("Hello, world!");

    

}

