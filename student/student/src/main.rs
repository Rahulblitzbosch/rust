use core::f64;

fn main() {
    println!("Hello, world!");
    let _scores: [f64; 3] = [70.0,58.5,87.3];
    let _frame: (&'static str, u32,f64,char)=("rahul",3244,91.0,'a');
    
    
    
   let _tup: (&'static str, u32, f64, char, &'static str, f64) = frame_func(_frame,_scores);

   let(name,rollno,percentage,grade,pass_failm,_avg)=_tup;

    println!("{} {} {} {} {} Average={}",name,rollno,percentage,grade,pass_failm,_avg );  


    println!("{} {} {} {}",_frame.0,_frame.1,_frame.2,_frame.3);

    

    

    //how to use calculate 
}

fn frame_func(_frame: (&'static str, u32,f64,char),_scores: [f64; 3])->(&'static str, u32,f64,char,&'static str,f64){
 
 let(_str_var,_int_var,_float_var, _char_var)=_frame;    
 let _average:f64 = {(_scores[0]+_scores[1]+_scores[2])/3.0};

 let _float_var:f64=_average;

    
   let _char_var=if _float_var>=60.0 && _float_var<70.0{'c'} else if _float_var>=70.0 && _float_var<80.0{'b'}
    else if _float_var>=80.0 && _float_var<90.0{'a'} else if _float_var>=90.0 && _float_var<100.0{'e'} else {'f'};
   let _pass_fail:&str=if _float_var>=60.0 {"pass"} else {"fail"};

   

    return (_str_var,_int_var,_float_var,_char_var,_pass_fail,_average);
}