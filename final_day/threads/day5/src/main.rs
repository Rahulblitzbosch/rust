use std::{f32::consts::E, io};
use chrono::{Local, Utc};
use rand::Rng;
#[warn(unused_variables)]
#[warn(dead_code)]
struct Task {
    id: u32,
    description: String,
    status:bool,
}

#[warn(dead_code)]
enum TaskStatus {
    Pending,
    Completed,
}

impl Task {
    fn new(mut id: u32,mut description: String) -> Task {
        Task {
            id:355,
            description:String::from("house maintainance"),
            status: false,
        }
    }
       
}




fn read_input(){
    println!("Enter task id:");
    let mut input = String::new();
  /*   io::stdin().read_line(&mut input).expect("Failed to read line");
    let id: u32 = input.trim().parse().expect("Please enter a valid number");*/

   match io::stdin().read_line(&mut input)
    {
        Ok(_) => {
            let mut str = input.trim().to_string();
            println!("Task ID: {}",str);
        }
        Err(e) => println!("Failed to read line {}",e),
    }




    
    
}




fn main() {
   
let mut task = Task::new(1, String::from("Learn Rust"));
    println!("Default Task ID: {}, Description: {}", task.id, task.description);

    let status = TaskStatus::Pending;
    match status {
        TaskStatus::Pending =>  read_input(),
        TaskStatus::Completed => println!("Task is completed"),
    }

//----------------------------chrono----------------------------------------

  // Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);
    
  //---------------------------rand

    let mut rng = rand::rng();

    // simulate rolling a die
    println!("roll = {}", rng.gen_range(1..7));  

   

    

//--------------------------------------------------------------------------------------

   
}
