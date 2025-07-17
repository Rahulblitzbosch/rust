struct Book{  //capitalized 'B' for struct name
name: String,
author: String,
year: u32,
pages: u32,
available: bool,
}
//type alias fot book doesnt exist in Rust, so we use struct directly
fn main() {

    let  obj = Book {                                 //object instantiation
        name: String::from("The Rust Programming Language"),
        author: String::from("Stefan Klabnik"),
        year: 2018,
        pages: 552,
        available: true,
    };

    let mut obj_copy = Book {                                 //object instantiation
        name: String::from("The Rust Programming Language"),
        author: String::from("Stefan Klabnik"),
        year: 2018,
        pages: 552,
        available: true,
    };



    let _obj2=obj_copy; //this line is valid because `_obj_copy` is mutable
    let _obj3 = _obj2;

    struct_func(_obj3); //passing object to function

   // obj_copy.available = false; //this line is valid because `obj_copy` is mutable

   // obj.available = false; //this line will cause an error because `available` is not mutable



print!(
        "Book Name: {}\nAuthor: {}\nYear: {}\nPages: {}\nAvailable: {}",
        obj.name, obj.author, obj.year, obj.pages, obj.available );
}

fn struct_func(mut book: Book) { //function that takes a Book object
    book.available=false; //this line will cause an error because `available` is not mutable
    println!("Book Name: {:}", book.name);
    println!("Author: {:}", book.author);
    println!("Year: {:}", book.year);
    println!("Pages: {:}", book.pages);
    println!("Available: {:}", book.available);
}