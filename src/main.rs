use rand::Rng; //
use std::io;
fn main(){


roomgen::roomtype 
    
println!("left:{} Mid:{} Right:{}",leftdoor,middledoor,rightdoor);

}

fn roomgen()
{
let directions  Directions::Left;//left middle right doorways

let roomtype =["loot", "enemy", "boss", "empty", "camp"];
let leftdoor  = roomtype [rand::thread_rng().gen_range(1..=5)];
let middledoor = roomtype [rand::thread_rng().gen_range(1..=5)];
let rightdoor = roomtype [rand::thread_rng().gen_range(1..=5)];




}
enum Directions{
    left,
    right,
    middle,
}