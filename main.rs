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
    println!("Congratulations! Your Rust program works.");
}