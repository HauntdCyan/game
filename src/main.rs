use rand::Rng;

fn main(){

let enemy = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];
let enemyhp = rand::thread_rng().gen_range(100..=200);
let enemyatt = 20;



let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection

println!("{} appears!!", enemy[enemyrng]);

let hp = 100;//hp will not reset after fights later
let power = 20;

println!("HP:{hp}");//the battle intro
println!("ATTACK POWER:{power}");
println!("What will you do?");
println!("ATTACK ITEMs RUN");



println!("test {enemyhp}")
//add choice slection and turns for battles with loopsS



}



