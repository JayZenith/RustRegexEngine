use std::ops::Range;

fn fill_range(range: Range<char>){
    for i in range{
        println!("{}",i as usize);
    }
}

fn main(){
    fill_range('a'..'c');
}