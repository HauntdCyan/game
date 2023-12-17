use std::io;
use rand::Rng;


fn main()
{
    // Greet the user and welcome them to the game.
    println!("\n\nHello welcome to the gaem!");

    // Call the "who are you" function. This function returns our name and puts it into the name variable.
    let name: String = who_are_you();







}

// This is the "who are you" function. It takes no inputs but will output a variable of type "String"
fn who_are_you() -> String
{
    // Crate the name variable and make it type "String". This holds the players name.
    let mut name: String = String::new();

    // This variable holds a simple (Y)es or (n)o when the player is making decisions.
    let mut choice: String = String::new();

    loop
    {
        println!("\nWhat is your name?");
        io::stdin().read_line(&mut name).unwrap();
        name.truncate(name.len() - 2);
        
        println!("\nYour name is: {name}\nAre you sure you want to use this name? (Y/n)");
        io::stdin().read_line(&mut choice).unwrap();
        choice.truncate(choice.len() - 2);

        

        // If the name variable is equal to "y". The ".to_lowercase()" method just allows us for case insensitive.
        if choice.to_lowercase() == "y"
        {
            println!("Welcome to the gaem!");

            // This exits the function and returns the data in the variable "name".
            return name
        }
        else
        {
            // This empties the string variables.
            name.truncate(name.len() - name.len());
            choice.truncate(choice.len() - choice.len());
        }
    }
}



fn battle()
{let player_attack = {power};
let player_hp = {hp};

}
