

// Initializing Macros as Constants (Can't be mutable)
const MAX_STUDENTS: usize = 5; //to create array of tuples
const NUM_GRADES: usize = 3;   //to create array of marks
//(only initializing the type)
type StudentRecord = (String, [f32; NUM_GRADES], f32); //tuple with name, grade array & average


fn main() {
    println!("Hello, world!");
 // Initialize student data
    let mut students = initialize_students();
    
   // println!("{}",students.[0])
    // Calculate averages
  //  calculate_averages(&mut students);
    
    // Generate and display report
  //  display_grade_report(&students);
    
    // Display top student
   // display_top_student(&students);
    
    // Display performance categories
  //  display_performance_categories(&students);


    
}

/// Initialize student array with fixed names and grades
fn initialize_students() -> [StudentRecord; MAX_STUDENTS] {
    [
        (String::from("Alice"), [92.5, 88.0, 95.0], 0.0),
        (String::from("Bob"), [76.0, 82.5, 79.0], 0.0),
        (String::from("Charlie"), [88.0, 91.0, 89.5], 0.0),
        (String::from("Diana"), [65.0, 72.0, 68.5], 0.0),
        (String::from("Ethan"), [95.0, 97.0, 96.5], 0.0),
    ]
}
