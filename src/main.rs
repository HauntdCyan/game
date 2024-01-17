use rand::Rng; //
use std::{io, collections::btree_map::Values};
fn main(){

    opendoors();
    if opendoors.middledoor = true {println!("mid door open")};
    
    
    roomtype()
//println!("left:{} Mid:{} Right:{}",leftdoor,middledoor,rightdoor);

}

fn opendoors() -> Doorsopen
{
let mut leftdoor = false;
let mut middledoor = false;
let mut rightdoor = false;



let roomtype =["loot", "enemy", "boss", "empty", "camp"];

let mut randomization_turn = 0;
loop{

let directions = rand::thread_rng().gen_range(1..=5); //left middle right doorways

if directions == 1 || directions == 2 {randomization_turn += 1;}
if directions == 3 {randomization_turn += 1; leftdoor  = true;}
if directions == 4 {randomization_turn += 1; middledoor  = true;}
if directions == 5 {randomization_turn += 1; rightdoor  = true;}

if leftdoor || middledoor || rightdoor {if randomization_turn > 3 {
    
    let doorsopen = Doorsopen{
leftdoor:leftdoor,
middledoor:middledoor,
rightdoor:rightdoor};
    
    return doorsopen
}}


}





}

struct Doorsopen{

leftdoor:bool,
middledoor:bool,
rightdoor:bool
}

fn roomtype(){





}