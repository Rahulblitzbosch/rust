use std::thread;
use std::time::Duration;
//thread finishes first-before main thread
fn main() {

    // create a thread
   let handle= thread::spawn(|| {

        // everything in here runs
        // in its own separate thread
        for i in 0..20 {  //always less than main

            println!("Loop thread 2 iteration: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
        println!("thread 2 finished");
    });
handle.join().unwrap(); //wait for thread to finish after main executues 
    // main thread
     for i in 0..5 {

        println!("Loop 1 main iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    } 


println!("main thread 1 finished");
}