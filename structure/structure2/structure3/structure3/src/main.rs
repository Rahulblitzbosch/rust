/*Multiple Member functions of a Structure*/
/*Implementation in one impl*/
/*Member function of one object using object of another structure*/
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}   //Structure Implemented
struct Circle{
    radius:f32
}

impl Rectangle {    //Member function implementation starts
    fn area(&self) -> f32 {   //syntax of member function declaration
        self.width * self.height
    }
    fn what_shape(&self) -> String {   
        //syntax of member function declaration
        if self.width==self.height{
            String::from("Square")
        }
        else{
            String::from("Rectangle")
        }
    }
    fn can_hold(&self, another: &Rectangle) -> bool {
        //Member function, taking other object as argument
        self.width >= another.width && self.height >= another.height
    }
    fn can_hold_circle(&self, another: &Circle)->bool{
        //Member function with object of different structure
        if(self.width*self.height>3.14*another.radius*another.radius){
            true
        }
        else{
            false
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    let rect2 = Rectangle {
        width: 30.0,
        height: 30.0,
    };
    let rect3 = Rectangle {
        width: 30.0,
        height: 60.0,
    };
    let circle1=Circle{
        radius:20.0,
    };
    println!("The shape is: {} {:#?}",rect1.what_shape(),rect1);
    println!("The area of the {} is {} square pixels.",
        rect1.what_shape(),
        rect1.area()      //Calling the member function
    );
    println!("Can rect1 hold rect2? {}",
    rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",
    rect1.can_hold(&rect3));
    println!("Can rect1 hold circle1? {}",
    rect1.can_hold_circle(&circle1));
}
