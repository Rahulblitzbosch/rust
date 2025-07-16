
fn main() {
let str: String = String::from("Rahul");
let _str2:String = String::from("krishnan");
 println!("returned string is {}",string_detect(str));
}

fn string_detect(str:String)->String{  //String is heap allocated
  let  mut temp: String = String::new(); //blank key no value


  for  c in str.chars(){  //get chars from string 
    if c == 'l' {
  break;
    }
    temp.push(c);     
  }
  

 // println!("{}",temp);
temp
}