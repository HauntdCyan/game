use rand::Rng; //
use std::io;
fn main(){


    
//println!("left:{} Mid:{} Right:{}",leftdoor,middledoor,rightdoor);

}

fn roomgen()
{
let directions = rand::thread_rng().gen_range(0..=200); //left middle right doorways

let roomtype =["loot", "enemy", "boss", "empty", "camp"];
let leftdoor  = roomtype [rand::thread_rng().gen_range(1..=5)];
if directions < 150 { let middledoor = roomtype [rand::thread_rng().gen_range(1..=5)]};
let rightdoor = roomtype [rand::thread_rng().gen_range(1..=5)];

println!("{}",middledoor)


}