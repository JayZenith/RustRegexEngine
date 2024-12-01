use std::io::{self, BufRead, Write};


#[derive(Debug)]
enum State{
    Locked, 
    Unlocked
}

#[derive(Debug)]
enum Event{
    Push, 
    Coin
}
fn next_state(state: State, event: Event) -> State{
    match state{
        State::Locked => match event{
            Event::Push => State::Locked,
            Event::Coin => State::Unlocked,
        }
        State::Unlocked => match event{
            Event::Push => State::Locked,
            Event::Coin => State::Unlocked,
        }
    }
}

fn main() {
    let mut state = State::Locked;
    println!("State: {:?}", state);
    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines(){
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            unknown => {
                eprintln!("ERROR: unknown event {}", unknown);
            }
        }
        println!("State: {:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}