mod data_models;
use crate::data_models::Character;

fn main() {
    // Simple printing
    println!("Hello, world!");

    /*


    */
    // Basic assignment and addition
    let mut x = 5;

    println!("x is: {}", x);

    x += 1;
    println!("x is: {}", x);

    /*


    */
    // Strings!
    let mut my_string = "hello";
    println!("my_string is {}", my_string);

    let mut growable_string: String = "hello".to_string();
    println!("{}", growable_string);
    growable_string += " everyone!";

    let to_stringed_my_string = my_string.to_string() + "everyone!";
    my_string = &to_stringed_my_string;
    println!("my_string remains {}", my_string);

    /*


    */
    // Structs!
    let player_1 = Character {
        health: 100,
        mana: 100,
        move_speed: 5,
        name: "Hiro Protagonist".into(),
    };

    println!("{:?}", player_1);

    //player_1.health -= 5;

    deal_damage(player_1, 5);

    //println!("{:?}", player_1);
}

fn deal_damage(mut player: Character, damage: u16) {
    player.health -= damage;
}
