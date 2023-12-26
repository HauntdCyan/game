use rand::Rng;
use std::io;

fn main(){

let enemy = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];//enemy name n stats
let enemyhp = rand::thread_rng().gen_range(100..=200);
let enemyatt = rand::thread_rng().gen_range(15..=25);



let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection

println!("{} appears!!", enemy[enemyrng]);

let hp = 100;//hp will not reset after fights later
let power = 20;
//battle loop start
println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:{power}");
println!("What will you do?");
println!("ATTACK SKILL RUN");

let mut selection = String::new();

io::stdin()
.read_line(&mut selection)
.expect("You cannot do that");
 
 if let _selection = "attack" {println!("You do a thing");}//fix
 if let _selection = "run" {println!("You ran away");}//fix
 //else { println! ("You can't do that");} //guhh??
println!("test {enemyhp}");
//add choice slection and turns for battles with loops




}
fn running(){//running choice
let runningchance = rand::thread_rng().gen_range(0..=50);


}


