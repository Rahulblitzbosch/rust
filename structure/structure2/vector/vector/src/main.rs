fn main() {
   let Vec = vec![1, 2, 3, 4, 5];

let mut vec_string: Vec<&str> = vec!["one",
       "two",
       "three",
       "four",];
//vec_string[0] = "ten"; //modifying static string full replacement
   let mut vec_dynamic = vec![String::from("one"),
       String::from("two"),
       String::from("three"),
       String::from("four"),];
vec_dynamic[0].push_str("ten"); //vector of dynamic strings concat

vec_string[0] = "ten"; //modifying static string full replacement

   println!("Vector: {:?}", Vec);
   println!("String Array: {:?}", vec_string); 

    println!("Dynamic String Vector: {:?}", vec_dynamic);

    //println!("Dynamic String Vector after modification: {:?}", vec_string[14]); //gives error for array indexing outof bounds
    println!("Dynamic String Vector after modification: {:?}", vec_string.get(6)); //gives none for out of bounds access
  
  //vec_dynamic.push(String::from("six")); //adding new element to dynamic string vector
  //  println!("Dynamic String Vector after adding new element: {:?}", vec_dynamic);
    vec_string.push(String::from("six")); //adding new element to static string vector
    println!("String Array after adding new element: {:?}", vec_string);
     
}

