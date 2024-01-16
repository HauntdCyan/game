use std::io;
use rand::Rng;


fn main()                                                            //main
{
    // Greet the user and welcome them to the game.
    println!("\n\nHello welcome to the gaem!");

    // Call the "who are you" function. This function returns our name and puts it into the name variable.
    let playername: String = who_are_you();
    
    let mut hp = 100;//hp will not reset after fights later
let mut weapondmg = 200000000;
let mut healingstrength = 40;

let (hp,winning) = battle(&playername,hp,healingstrength,weapondmg);  if winning == false {std::process::exit(0) };//battles
                 //return hp
let (hp,winning) = battle(&playername,hp,healingstrength,weapondmg);  if winning == false {std::process::exit(0) };//battles
//add drops and a way to adjust enemy hp/attack per encounter AND make hp not change after battle


}

                                                        ///name selection
fn who_are_you() -> String
{
    // Crate the name variable and make it type "String". This holds the players name.
    let mut playername: String = String::new();

    // This variable holds a simple (Y)es or (n)o when the player is making decisions.
    let mut choice: String = String::new();

    loop
    {
        println!("\nWhat is your name?");
        io::stdin().read_line(&mut playername).unwrap();
        playername.truncate(playername.len() - 2);
        
        println!("\nYour name is: {}\nAre you sure you want to use this name? (Y/n)",playername);
        io::stdin().read_line(&mut choice).unwrap();
        choice.truncate(choice.len() - 2);

        

        // If the name variable is equal to "y". The ".to_lowercase()" method just allows us for case insensitive.
        if choice.to_lowercase() == "y"
        {
            println!("\n\nWelcome to the gaem!\n\n");

            // This exits the function and returns the data in the variable "name".
            return playername
        }
        else
        {
            // This empties the string variables.
            playername.truncate(playername.len() - playername.len());
            choice.truncate(choice.len() - choice.len());
        }
    }
}



fn battle(playername: &str,mut hp:i32,healingstrength:i32,weapondmg:i32) -> (i32,bool)
{
    let mut winning = true;
    let mut enemy = enemystats();//enemy statss
    let mut turn = 0;//turn counter
    
    
    loop {//battle loop start
    let enemyatt = rand::thread_rng().gen_range(15..=25);
        if turn == 0{
        println!("\n\n{} appears!! HP: {} ATT: {}\n",enemy.enemy_name, enemy.enemyhp, enemyatt);
    println!("{playername}");//PLACEHOLDER FOR PLAYER'S NAME
    }
    
    let power = rand::thread_rng().gen_range(15..={weapondmg});//player attack
    
        let healing = rand::thread_rng().gen_range(20..={healingstrength}); //skills
    let guard = rand::thread_rng().gen_range(5..=20);
        let mut ran_away = false;
    
    println!("HP:{hp}"); 
    println!("ATTACK POWER:{}\n",weapondmg);
    println!("What will you do?");                                                    //FIX ENEMY ATTACK SO ITS RANDOM
    println!("ATTACK SKILL CHECK RUN");
    
    let mut selection = String::new();
    
    io::stdin()
    .read_line(&mut selection)
    .expect("You cannot do that");                                             //selections
    print!("{}[2J", 27 as char);                    //clears terminal
    
    if selection.to_lowercase().contains("attack"){                                 //attacking
    enemy.enemyhp = enemy.enemyhp - power;
    
    println!("\nYou attack {} causing {power} damage!!",enemy.enemy_name);
    println!("\n{} now has {} hp left!\n",enemy.enemy_name, enemy.enemyhp);
    }
    
    if selection.to_lowercase().contains("run") {ran_away = running(); }
    if ran_away == true {winning = true; return (hp,winning);}
    
    if selection.to_lowercase().contains("check") {//displays enemy's stats
        println!("\n\n{}:",enemy.enemy_name);
        println!("{}HP {}ATT\n\n",enemy.enemyhp, enemyatt); continue;
    }
    
    
    if selection.to_lowercase().contains("skill"){ hp= skills(hp,healing);}              //skills
    
    
    //breaks loop if run succeeded
    
     //else { println! ("You can't do that");} //guhh??
    if enemy.enemyhp < 1 {println!("you win!!");let winning = true; return (hp, winning);}               // return hp
    selection.truncate(0);//clears selection
    
                                                     //enemy attack
    hp = hp - enemyatt;
    println!("{} attacks you for {} DMG\n",enemy.enemy_name,enemyatt);
    
    if hp < 1 { println!("You died");let winning = false; return (hp, winning);}
    turn += 1;//adds 1
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
    }
    
    fn enemystats() -> EnemyValues {
    
        
    let enemy_names = ["Slime", "Cool_guy", "Among_us_impostor", "Jerma_from_twitchtv"];//enemy name n stats
    
    let enemyrng = rand::thread_rng().gen_range(0..=3);//enemy selection
    let enemy_name = enemy_names[enemyrng];
    let enemyhp = rand::thread_rng().gen_range(100..=200);
    
    let enemy_stats = EnemyValues
    {
        enemy_name: enemy_name,
        enemyhp: enemyhp,
    };
    return enemy_stats
    ;
    
    
    }
