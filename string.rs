use std::ops::Range;


type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 127;

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
            print!("{} => ", symbol);
            for column in self.cs.iter(){
                println!("{}",column.ts[symbol]);
            }
        }
    }
}

fn main(){
    let mut fsm = Fsm::new();

    //FsmColumn 0
    {
        let mut col = FsmColumn::new(); //get struct with 0 init array
        col.fill_range('a'..'b', 1); //ascii char index with state 1 assigned 
        fsm.push(col);
    }

    fsm.dump();
}