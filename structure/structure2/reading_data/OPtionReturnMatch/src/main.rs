use std::{io::SeekFrom, sync::Arc};


#[derive(Debug)]
enum  State {
    Start,
    Middle,
    End,
}

struct StateMachine {
    state: State,
}   

impl StateMachine {
    fn new()->Self{
        Self {
            state: State::Start,
        }
    }

    fn transition(&mut self,  new_state: & mut State) {

     match self.state {
        State::Start => {let mut new_state=start_function(&new_state);
        self.state = new_state;
        println!("Current state: {:}", new_state);},
        State::Middle => {let mut new_state=middle_function(&new_state);
        println!("Current state: {:}", new_state);},
        State::End => {let mut new_state=end_function(&new_state);
        println!("Current state: {:}", new_state);},    
    }

        
}

}


fn main() {
    println!("Hello, world!");
  let mut _machine = StateMachine::new();      
        _machine.transition(State::Middle);
        
  

}


fn switch_loop(mut self_state: &State){
     println!("Looping..."); 

    match self_state {
        State::Start=>{},
        State::Middle=>{},
        State::End=>{}
    }
    println!("outer Current state: {:#?}", self_state);
    std::thread::sleep(std::time::Duration::from_secs(1));

    

}

fn start_function(self_state: &State)-> State {
    println!("State is Start");
   let mut self_state = State::Middle; // Change state to Middle
    // Change state to Middle
    return self_state;
}

fn middle_function(self_state: &State) -> State{
    println!("State is Middle");
    State::End
    // Here you can change the state if needed
    // _state = State::End; // Uncomment to change state
}
fn end_function(self_state: &State)-> State{
    println!("State is End");
    State::Start
    // Here you can change the state if needed
    // _state = State::Start; // Uncomment to change state
}