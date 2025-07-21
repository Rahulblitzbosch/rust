use std::fs::File;
use std::io::Read;
use std::io::Write;

//copy data from one file data1.txt to another data2.txt

fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("data1.txt").unwrap();
	
	println!("path is: {:?}",data_file);

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);


     // Open a file in read only mode in the local file system
    let data_result = File::create("data2.txt");
	
    // Reading a file returns a Result enum
    // Result can be a file or an error
    let mut data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    // Write to the file
    data_file.write_all(file_content.as_bytes()).unwrap();




}