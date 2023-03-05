use std::env::{args, Args};

fn main(){
    let mut arugs:Args = args();
    let first:u32 = args().nth(1).unwrap().parse::<u32>().unwrap();
    let op:char = arugs.nth(2).unwrap().chars().next().unwrap();
    let second:u32 = args().nth(3).unwrap().parse::<u32>().unwrap();

    println!("{} {} {}", first, op, second);
    match op {
        '+' => sum(first, second),
        _ => panic!(" ")
    };
}

fn sum(a:u32, b:u32){
   println!("{}",a+b);
}