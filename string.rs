use std::ops::Range;


type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEWLINE: usize = 129;

struct FsmColumn {
    ts: [FsmIndex; FSM_COLUMN_SIZE], //array of 127 usize
} //127 ascii charcters 


impl FsmColumn{
    //new() is a constructor associated with a struct 
    fn new() -> Self {
        Self {
            ts: [0; FSM_COLUMN_SIZE] //0 initialized the array 
        }
    }
    
    fn fill_range(&mut self, range: Range<char>, state: FsmIndex){
        for i in range{  //ascii 
            self.ts[i as usize] = state;
        }
    }
}

struct Fsm{
     cs: Vec<FsmColumn>
}

impl Fsm{
    fn new() -> Self{
        Self{
            cs: Vec::new()
        }
    }

    fn push(&mut self, column: FsmColumn){
        self.cs.push(column);
    }

    fn dump(&self){
        for symbol in 0..FSM_COLUMN_SIZE{
            print!("{:03} => ", symbol);
            for column in self.cs.iter(){
                print!("{} ",column.ts[symbol]);
            }
            println!("");
        }
    }
}

fn match_fsm(fsm: &Fsm, input: &str) -> bool {
    let mut state = 1;
    for c in input.chars(){
        if state == 0 || state >= fsm.cs.len(){ //reached terminal state
            break;
        }

        //we have a state, an event, and 2d table
        //where we lookup the next state
        state = fsm.cs[state].ts[c as usize];
    }

    if state == 0{
        return false;
    }
    if state < fsm.cs.len(){
        state = fsm.cs[state].ts[FSM_NEWLINE];
    }

    return state >= fsm.cs.len();
}

fn main(){
    let mut fsm = Fsm::new();

    let events = vec!['a' as usize, 'b' as usize, 'c' as usize, FSM_NEWLINE];    
    //Failed State
    fsm.push(FsmColumn::new());


    for event in events.iter(){
        let mut col = FsmColumn::new();
        col.ts[*event] = fsm.cs.len() + 1;
        fsm.push(col);
    }

    // //FsmColumn 1
    // {
    //     let mut col = FsmColumn::new(); //get struct with 0 init array
    //     //col.fill_range('a'..'b', 1); //ascii char index with state 1 assigned 
    //     col.ts['a' as usize] = fsm.cs.len() + 1;
    //     fsm.push(col);
    // }

    // //FsmColumn 2
    // {
    //     let mut col = FsmColumn::new(); //get struct with 0 init array
    //     //col.fill_range('a'..'b', 1); //ascii char index with state 1 assigned 
    //     col.ts['b' as usize] = fsm.cs.len() + 1;
    //     fsm.push(col);
    // }

    // //FsmColumn 3
    // {
    //     let mut col = FsmColumn::new(); //get struct with 0 init array
    //     //col.fill_range('a'..'b', 1); //ascii char index with state 1 assigned 
    //     col.ts['c' as usize] = fsm.cs.len() + 1;
    //     fsm.push(col);
    // }

    // //FsmColumn 4
    // {
    //     let mut col = FsmColumn::new(); //get struct with 0 init array
    //     //col.fill_range('a'..'b', 1); //ascii char index with state 1 assigned 
    //     col.ts[FSM_NEWLINE] = fsm.cs.len() + 1;
    //     fsm.push(col);
    // }

    
    fsm.dump();

    let inputs = vec!["Hello, World","abc", "abcd"];
    for input in inputs.iter(){
        println!("{:?} => {:?}", input, match_fsm(&fsm, input));
    }
}