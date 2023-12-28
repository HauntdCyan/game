use rand::Rng; //to do list: make skills and run function work
use std::io;

fn main(){

let enemy = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];//enemy name n stats
let enemyhp = rand::thread_rng().gen_range(100..=200);
let enemyatt = rand::thread_rng().gen_range(15..=25);



let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection

println!("{} appears!!\n", enemy[enemyrng]);

let hp = 100;//hp will not reset after fights later
let power = 20;

loop {//battle loop start
    

println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:{power}\n");
println!("What will you do?");
println!("ATTACK SKILL RUN");

let mut selection = String::new();

io::stdin()
.read_line(&mut selection)
.expect("You cannot do that");

if selection.to_lowercase().contains("attack"){println!("\nyou do a thing");}

if selection.to_lowercase().contains("run"){running();}//functions activates no matter what


//running output
let ran_away = running();
if ran_away == true {println!("you ran away"); break; }

 //else { println! ("You can't do that");} //guhh??
println!(" ene test {enemyhp}");// enemy hp test
//add choice slection and turns for battles with loops


}//end of battle loop

}
fn running() -> bool
{//running chance
let running_chance = rand::thread_rng().gen_range(0..=100);


let ran_away = false;

if running_chance < 50 { let ran_away = true;}//fix

//else if running_chance > 50 { let ran_away =false; println!("\nYou failed to run away :(\n");}//succeeding on running
return ran_away;
}


