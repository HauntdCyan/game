use rand::Rng; //
fn main(){

    opendoors();
//let doors = Doorsopen {left, right, middle};
    if doors.left == true {println!("shit it  left")};
    if doors.middle == true {println!("shit it  middle")};
    if doors.right == true {println!("shit it  right")};
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
left:leftdoor,
middle:middledoor,
right:rightdoor};
    
    return doorsopen
}}


}





}

struct Doorsopen{

left:bool,
middle:bool,
right:bool
}

fn roomtype(){





}