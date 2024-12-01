use std::io::{self, BufRead, Write};


// #[derive(Debug)] 
// enum State{ 
//     Locked, 
//     Unlocked
// }

// #[derive(Debug)]
// enum Event{
//     Push, 
//     Coin
// }

enum State{
    Locked, 
    Unlocked,
}

 
type FSM = HashMap<State, HashMap<Event, State>>

const LOCKED: usize = 0;
const UNLOCKED: usize = 1;
const STATES_COUNT: usize = 2;

const PUSH: usize = 0;
const COIN: usize = 1;
const EVENTS_COUNT: usize = 2;

const FSM: [[usize;EVENTS_COUNT]; STATES_COUNT] = [
    //PUSH COIN
    [LOCKED, UNLOCKED],       //LOCKED
    [LOCKED, UNLOCKED],   //UNLOCKED

];

//fn next_state(state: State, event: Event) -> State{
//accept usizes 
fn next_state(state: usize, event: usize) -> usize{
    FSM[state][event]    // match state{
    //     State::Locked => match event{
    //         Event::Push => State::Locked,
    //         Event::Coin => State::Unlocked,
    //     }
    //     State::Unlocked => match event{
    //         Event::Push => State::Locked,
    //         Event::Coin => State::Unlocked,
    //     }
    // }
}

fn state_to_str(state: usize) -> &'static str{
    match state {
        LOCKED => "Locked",
        UNLOCKED => "Unlocked",
        _ => unreachable!()
    }
}

fn main() {
    // let mut state = State::Locked;
    let mut state = LOCKED;
    // println!("State: {:?}", state);
    println!("State: {}", state_to_str(state));
    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines(){
        match line.unwrap().as_str() {
            // "coin" => state = next_state(state, Event::Coin),
            // "push" => state = next_state(state, Event::Push),
            "coin" => state = next_state(state, COIN),
            "push" => state = next_state(state, PUSH),
            "quit" => return,
            unknown => {
                eprintln!("ERROR: unknown event {}", unknown);
            }
        }
        println!("State: {}", state_to_str(state));
        print!("> ");
        io::stdout().flush().unwrap();
    }
}