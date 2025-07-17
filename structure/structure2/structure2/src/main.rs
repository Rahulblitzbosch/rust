
struct Person {
    name: String,
    age: u32,
    adhar: String,
    address: String,
    country: String,
}

 #[derive(Debug)]
struct Color(i32, i32, i32, i32); // Tuple struct

 #[derive(Debug)]
struct Alwaysequal;

 #[derive(Debug)]
struct Iequal;

 #[derive(Debug)]
struct Area{
    length:u32,
    width:u32,
} //structure implememted

impl Area{  //member function implementation starts
fn reactangle(&self)-> u32 {
   self.length*self.width
}

}
/* 
struct Car {
    model: String,
    year: u32,
}
*/
fn main() {


    let _ar1 = Area {length:39,width:40};

    println!("Struct={:#?}Area={:?}",_ar1,_ar1.reactangle());

    let subject = Alwaysequal;
    println!("{:?}",subject);
    let _subject2 = Iequal;
    println!("{:?}",_subject2);
    
/* 
    let person = update_person(Person {
        name: String::from("Alice"),
        age: 30,
    });
*/

 //   println!("Name: {}, Age: {}", person.name, person.age);
 //   println!("Hello, world!");

  //   let car = new_vehicle(model: String::from("Toyota"), year: 2020);
  //------------------------------------------------------------------------


let _black =Color(0, 0, 0, 255); // Tuple struct instantiation
let _meroon = Color(4,5,7, 255); // Tuple struct instantiation


println!("Black Color: {:#?}", _black);




  let person = Person {
        name: String::from("Alice"),
        age: 30,
        adhar: String::from("1234-5678-9012"),
        address: String::from("123 Main St, City, Country"),
        country: String::from("USA"),
    };

        println!("Person Details:");
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Adhar: {}", person.adhar);
    println!("Address: {}", person.address);

   let _person2 = Person {
        name: String::from("Bob"),
        age: person.age+5,
        adhar: String::from("9876-5432-1098"),
        address: String::from("456 Park Ave, City, Country"),
        ..person   //struct update syntax using same as struct object
   };

   println!("Person Details:");
    println!("Name: {}", _person2.name);
    println!("Age: {}", _person2.age);
    println!("Adhar: {}", _person2.adhar);
    println!("Address: {}", _person2.address);
    println!("Country: {}", _person2.country);

   let _person3= Person {
       .._person2 //struct update syntax using same as struct object    
    };

    /* 
   println!("Person Details:");             //ownership because of string
    println!("Name: {}", _person2.name);
    println!("Age: {}", _person2.age);
    println!("Adhar: {}", _person2.adhar);
    println!("Address: {}", _person2.address);
    println!("Country: {}", _person2.country);
      */
    

   println!("Person Details:");
    println!("Name: {}", _person3.name);
    println!("Age: {}", _person3.age);
    println!("Adhar: {}", _person3.adhar);
    println!("Address: {}", _person3.address);
    println!("Country: {}", _person3.country);






}



/* 

fn update_person(mut person: Person) -> Person {
    person.age += 1; // Increment age by 1
    person.name = String::from("Bob"); // Change name to "Bob"
    person
}*/



/* 
fn new_vehicle(model: String, year: u32) -> Car {
    println!("Creating a new vehicle: {} ({})", model,year);
    Car { model, year }

} */