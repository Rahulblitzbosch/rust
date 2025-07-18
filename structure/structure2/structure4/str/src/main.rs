

fn main() {
    println!("Hello, world!");
    //-----------------------------------------------
    // Count the number of 1s in an array
let mut count = 0;
  let _array: [i32; 5] = [1, 2, 1, 4, 5];
 for _i in _array.iter() {  

    if *_i==1{
        count+= 1;
    }


        //println!("{} {}", _i,count);
    }
    println!("Count of 1s: {}", count);
    //-----------------------------------------------
    // Find the maximum value in an array
let mut max = _array[0];
for &value in _array.iter() {
    if value > max {
        max = value;    
    }
}
println!("Maximum value: {}", max);
    //-----------------------------------------------
    // Find the minimum value in an array
let mut min = _array[0];
for &value in _array.iter() {
    if value < min {
        min = value;    
    }
}
println!("Minimum value: {}", min);
    //-----------------------------------------------
    // Calculate the sum of all elements in an array
let mut sum = 0;
for &value in _array.iter() {
    sum += value;                   
}
println!("Sum: {}", sum);
    //-----------------------------------------------
    // Calculate the average of all elements in an array
let mut sum = 0;
for &value in _array.iter() {
    sum += value;                   
}
let average = sum as f32 / _array.len() as f32;
println!("Average: {}", average);
    //-----------------------------------------------
    // Reverse an array
let mut reversed_array = _array;
reversed_array.reverse();
println!("Reversed array: {:?}", reversed_array);
    //-----------------------------------------------
    // Sort an array
let mut sorted_array = _array;
sorted_array.sort();
println!("Sorted array: {:?}", sorted_array);
    //-----------------------------------------------
    // Check if an array is sorted
let is_sorted = sorted_array.windows(2).all(|w| w[0] <= w[1]);
if is_sorted {
    println!("The array is sorted.");                   
}
else {
    println!("The array is not sorted.");   
    }       
    //-----------------------------------------------
    // Find the index of a specific value in an array
let value_to_find = 1;
if let Some(index) = _array.iter().position(|&x| x == value_to_find) {
    println!("Value {} found at index: {}", value_to_find, index);  
} else {
    println!("Value {} not found in the array.", value_to_find);                
    }
    //-----------------------------------------------
    // Find the index of the first occurrence of a specific value in an array    
let value_to_find = 1;
if let Some(index) = _array.iter().position(|&x| x == value_to_find) {
    println!("First occurrence of value {} found at index: {}", value_to_find, index);  
} else {
    println!("Value {} not found in the array.", value_to_find);
    }
    //-----------------------------------------------
    // Find the index of the last occurrence of a specific value in an array
let value_to_find = 1;
if let Some(index) = _array.iter().rposition(|&x| x == value_to_find) {
    println!("Last occurrence of value {} found at index: {}", value_to_find, index);  
} else {
    println!("Value {} not found in the array.", value_to_find);
    }   
    //-----------------------------------------------
    // Find the index of the first occurrence of a specific value in an array
let value_to_find = 1;
if let Some(index) = _array.iter().position(|&x| x == value_to_find) {
    println!("First occurrence of value {} found at index: {}", value_to_find, index);  
} else {
    println!("Value {} not found in the array.", value_to_find);
    }
    //-----------------------------------------------
    // Find the index of the last occurrence of a specific value in an array
let value_to_find = 1;
if let Some(index) = _array.iter().rposition(|&x| x == value_to_find) {
    println!("Last occurrence of value {} found at index: {}", value_to_find, index);  
} else {
    println!("Value {} not found in the array.", value_to_find);
    }   
    //-----------------------------------------------
    // Check if an array contains a specific value
let value_to_check = 1;
if _array.contains(&value_to_check) {
    println!("The array contains the value: {}", value_to_check);   
} else {
    println!("The array does not contain the value: {}", value_to_check);   

    }           
    //-----------------------------------------------

    
}
