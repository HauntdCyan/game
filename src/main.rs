use rand::Rng; //
use std::io;
fn main(){

    let roomchances = rand::thread_rng().gen_range(0..=200);


    
let roomtype =["loot", "enemy", "boss", "camp"];
let room = roomtype [rand::thread_rng().gen_range(0..=4)];



println!("{}",room)


}