use rand::Rng; //to do list: make skills and run function work
use std::io;

fn main(){
let EnemyValues {enemyatt: mut enemyatt, enemy_name: mut enemy_name, enemyhp: mut enemyhp };//enemy statss

println!("{enemy_name} appears!! HP: {enemyhp} ATT: {enemyatt}\n",);
println!("BOB:");//PLACEHOLDER FOR PLAYER'S NAME
let mut hp = 100;//hp will not reset after fights later
let power = rand::thread_rng().gen_range(15..=20);

loop {//battle loop start
let healing = rand::thread_rng().gen_range(20..=40); //skills
let guard = rand::thread_rng().gen_range(5..=20);
   
    let mut ran_away = false;

println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:20\n");
println!("What will you do?");
println!("ATTACK SKILL CHECK RUN");

let mut selection = String::new();

io::stdin()
.read_line(&mut selection)
.expect("You cannot do that");                                             //selections
print!("{}[2J", 27 as char);                    //clears terminal

if selection.to_lowercase().contains("attack"){                                 //attacking
enemyhp = enemyhp - power;

println!("\nYou attack {enemy_name} causing {power} damage!!");
println!("\n{enemy_name} now has {enemyhp} hp left!\n");
}

if selection.to_lowercase().contains("run") {ran_away = running(); }
if ran_away == true {break;}

if selection.to_lowercase().contains("check") {//displays enemy's stats
    println!("\n\n{enemy_name}:");
    println!("{enemyhp}HP {enemyatt}ATT\n\n"); continue;
}


if selection.to_lowercase().contains("skill"){ hp= skills(hp,healing);}              //skills


//breaks loop if run succeeded

 //else { println! ("You can't do that");} //guhh??
if enemyhp < 1 {println!("you win!!"); break;} 
selection.truncate(0);//clears selection

                                                 //enemy attack
hp = hp - enemyatt;
println!("Enemy attacks you for {enemyatt} DMG\n");

if hp < 1 { println!("You died"); break;}

}//end of battle loop

}


fn running() -> bool
{
let running_chance = rand::thread_rng().gen_range(0..=70);//running chance


let mut ran_away = false;
if running_chance < 50 { ran_away = true; println!("\nyou ran away!!\n");}

else if running_chance > 50 { ran_away =false; println!("\nYou failed to run away :(\n");}
return ran_away;
}


fn skills(mut hp:i32,healing:i32) -> i32{           //skills
   
    println!("\nWhich skill would you like to use?");
    println!("HEAL, GUARD");
    let mut skills = String::new();
    
    io::stdin()//skill selection
    .read_line(&mut skills)
    .expect("You cannot do that");                                                      
    print!("{}[2J", 27 as char);
    
    if skills.to_lowercase().contains("heal"){hp = hp + healing;
    println!("You healed for {healing}\n");
    };
    
    if skills.to_lowercase().contains("guard"){println!("gaming")};//not done
    return hp
    }

struct EnemyValues
{
    enemy_name: &'static str,
    enemyhp: i32,
    enemyatt: i32,
}

fn enemystats() -> EnemyValues {

    
let enemy_names = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];//enemy name n stats

let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection
let enemy_name = enemy_names[enemyrng];

let enemyhp = rand::thread_rng().gen_range(100..=200);
let enemyatt = rand::thread_rng().gen_range(15..=25);

let enemy_stats = EnemyValues
{
    enemy_name: enemy_name,
    enemyhp: enemyhp,
    enemyatt: enemyatt,
};
return enemy_stats
;


}