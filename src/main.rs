use rand::Rng;

fn main(){

let enemy = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];

let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection

println!("{} appears!!", enemy[enemyrng]);

let hp = 100;
let power = 20;

println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:{power}");
println!("What do you do?");
println!("ATTACK ITEM RUN");





}



