use rand::Rng; //


fn main(){

   let doorsopen = randomization();
//let doors = Doorsopen {left, right, middle};
    if doorsopen.left == true {println!("shit it left")};
    if doorsopen.middle == true {println!("shit it middle")};
    if doorsopen.right == true {println!("shit it right")};
    
//println!("left:{} Mid:{} Right:{}",leftdoor,middledoor,rightdoor);

}

fn randomization() -> Doorsopen
{
let mut leftdoor = false;
let mut middledoor = false;
let mut rightdoor = false;

let roomtype = ["loot", "enemy", "boss", "empty", "camp"];



let mut randomization_turn = 0;
loop{

let mut number_of_directions = 0;

let directions = rand::thread_rng().gen_range(1..=5); //left middle right doorways

if directions == 1 || directions == 2 {randomization_turn += 1;}
if directions == 3 {number_of_directions += 1; randomization_turn += 1; leftdoor  = true;}
if directions == 4 {number_of_directions += 1; randomization_turn += 1; middledoor  = true;}
if directions == 5 {number_of_directions += 1; randomization_turn += 1; rightdoor  = true;}



if leftdoor || middledoor || rightdoor {if randomization_turn > 3 {
    
let room = directions * randomization_turn + rand::thread_rng().gen_range(000..=500);

if (room <= 500) || (room >= 400) {println!("cool!!!")}//WORK ON THIS



println!("{room}");

let doorsopen = Doorsopen{
left:leftdoor,
middle:middledoor,
right:rightdoor

};
    
    return doorsopen
}}


}//reuse functions for thing and do another thing with it!!!!!!!!!!!!!!!!!!!!!!!!!!!11





}

struct Doorsopen{

left:bool,
middle:bool,
right:bool
}

fn roomtype(){





}