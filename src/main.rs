use rand::Rng; //
use std::io;
fn main(){

    roomgen();
    
//println!("left:{} Mid:{} Right:{}",leftdoor,middledoor,rightdoor);

}

fn roomgen()
{
let mut leftdoor = false;
let mut middledoor = false;
let mut rightdoor = false;



let roomtype =["loot", "enemy", "boss", "empty", "camp"];

let mut randomization_turn = 0;
loop{

let directions = rand::thread_rng().gen_range(1..=5); //left middle right doorways
if directions == 1 || directions == 2 {randomization_turn += 1;}
if directions == 3 {randomization_turn += 1;println!("left"); leftdoor  = true;}
if directions == 4 {randomization_turn += 1;println!("mid"); middledoor  = true;}
if directions == 5 {randomization_turn += 1;println!("right"); rightdoor  = true;}


if leftdoor || middledoor || rightdoor {if randomization_turn > 3 {break;}}



}
//randomization_turn.turncate(0);

println!("{} {} {}",leftdoor,middledoor,rightdoor)


}