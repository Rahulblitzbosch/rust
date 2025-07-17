// Initializing Macros as Constants (Can't be mutable)
const MAX_STUDENTS: usize = 5; //to create array of tuples
const NUM_GRADES: usize = 3;   //to create array of marks
type StudentRecord = (String, [f32; NUM_GRADES], f32); //tuple with name, grade array & average

fn main() {
        // Initialize student data
    let mut students = initialize_students();
    
    // Calculate averages
    calculate_averages(&mut students);
    
    // Generate and display report
    display_grade_report(&students);   //Display all student data
    
    // Display top student
    display_top_student(&students);  //display name & grade of student with highest mark
    
    // Display performance categories
   display_performance_categories(&students);  //display all student & category
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

fn calculate_averages(students:&mut [StudentRecord;MAX_STUDENTS]){
for val in students{
    val.2=(val.1[0]+val.1[1]+val.1[2])/3.0;
}
}

fn display_grade_report(students: &[StudentRecord;MAX_STUDENTS]){
println!("=========GRADE REPORT===========");
for val in students{
    //println!("{}  {}{}{}  {}  ",val.0,val.1[0],val.1[1],val.1[2],val.2);
    println!("{:<10}  {:?}  {:<10.2}  ",val.0,val.1,val.2);
}
}

fn display_top_student(students: &[StudentRecord;MAX_STUDENTS]){
    let mut max=0.0;
    for val in students{
        if max<val.2{
            max=val.2;
        }
    }
    for val in students{
        if val.2==max{
            println!("{} is the topper with {:.2} marks",val.0,val.2);
        }
    }
}

fn display_performance_categories(students: &[StudentRecord;MAX_STUDENTS]){
    for val in students{
        if val.2>=90.0{
            println!("{} is Excellent",val.0);
        }
        else if (val.2<90.0)&&(val.2>=70.0) {
            println!("{} is Good",val.0);}
        else{
            println!("{} Needs Improvement",val.0);
        }
    }
}

