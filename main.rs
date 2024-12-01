use std::io::{self, BufRead, Write};

enum State{
    Locked, 
    Unlocked
}

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
    let mut state = State::Unlocked;
    print!("> ");
    io::stdout().flush();
    for line in io::stdin().lock().lines(){
        match line.unwrap().as_str() {
            "coin" => todo!(),
            "push" => todo!(),
            unkown => {
                eprintln!("ERROR: unknown event {}", unkown);
            }
        }
        print!("> ");
        io::stdout().flush();
    }
}