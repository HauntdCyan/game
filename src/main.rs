use rand::Rng; //to do list: make skills and run function work
use std::io;

fn main(){

let enemy = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];//enemy name n stats
let enemyhp = rand::thread_rng().gen_range(100..=200);
let enemyatt = rand::thread_rng().gen_range(15..=25);



let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection

let enemy_name = enemy[enemyrng];
let mut enemy_hp = enemyhp;

println!("{enemy_name} appears!! {enemyhp}HP {enemyatt}ATT \n",);
println!("{enemy_name}");
let hp = 100;//hp will not reset after fights later
let power = rand::thread_rng().gen_range(15..=20);

loop {//battle loop start
let ran_away = false;    

println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:{power}\n");
println!("What will you do?");
println!("ATTACK SKILL RUN");

let mut selection = String::new();

io::stdin()
.read_line(&mut selection)
.expect("You cannot do that");                                             //selections

if selection.to_lowercase().contains("attack"){                                 //attacking

let mut new_enemy_hp = enemy_hp - power;

let mut enemy_hp = new_enemy_hp;


println!("\nYou attack {enemy_name} causing {power}damage!!");
println!("\n{enemy_name} now has {enemy_hp} hp left!\n");

}
//add attack doing something
if selection.to_lowercase().contains("run") { let ran_away = running(); }

if selection.to_lowercase().contains("Skill") {println!("no skills yet :)")}


if ran_away == true {break;}//breaks loop if run succeeded

 //else { println! ("You can't do that");} //guhh??
//println!(" ene test {enemyhp}");// enemy hp test
//add choice slection and turns for battles with loops

selection.truncate(0);
}//end of battle loop

}
fn running() -> bool
{//running chance
let running_chance = rand::thread_rng().gen_range(0..=100);


let ran_away = false;
if running_chance < 50 { let ran_away = true; println!("\nyou ran away!!\n");}

else if running_chance > 50 { let ran_away =false; println!("\nYou failed to run away :(\n");}//succeeding on running
return ran_away;
}


