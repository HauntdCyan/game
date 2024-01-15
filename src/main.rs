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
    let directions = rand::thread_rng().gen_range(1..=5); //left middle right doorways



let roomtype =["loot", "enemy", "boss", "empty", "camp"];

loop{
    if directions == 0 {continue;}
if directions > 1 { leftdoor  = true;break;}
if directions > 2 { middledoor  = true;break;}
if directions > 3 { rightdoor  = true;break;}
}
println!("{directions}");

println!("{} {} {}",leftdoor,middledoor,rightdoor)


}