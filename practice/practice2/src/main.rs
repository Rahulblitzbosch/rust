fn main() {
    println!("Hello, world!");
//Find how many times each item is duplicated in an array.

let arr=[1,3,5,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
let mut count = 0;
let item = 1;
for i in arr.iter() {
    if *i == item {
        count += 1;
    }
}
println!("The item {} is duplicated {} times", item, count);
}
