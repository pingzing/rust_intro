//#![deny(unused_must_use)]

mod data_models;
use data_models::{Character, CharacterBuilder, Enemy, PoisonResult};
use rand::Rng;

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
        strength: 5,
        name: "Hiro Protagonist".to_string(),
    };

    println!("{:?}", player_1);

    //player_1.health -= 5; // <-- mutability

    //deal_damage(&mut player_1, 5);

    //println!("{:?}", player_1);

    // Constructor pattern + mutability
    //let player_2 = Character::new();
    // player_2.health = 50;
    // player_2.mana = 200;
    // player_2.move_speed = 4;
    // player_2.strength = 1;
    // player_2.name = "Lancer Deuteragonist".to_string();

    // Replacement for lack of overlaoding

    let player_3 = CharacterBuilder::new()
        .health(75)
        .mana(75)
        .strength(3)
        .move_speed(8)
        .name("Sneaksneak Stabstab")
        .build();

    /*


    */
    // Error handling
    let mut enemy_1 = Enemy {
        health: 50,
        name: "Jerk".to_string(),
    };

    mug_enemy(&mut enemy_1, &player_3);
    // match mug_result {
    //     Ok(damage) => println!("{} did {} damage!", player_3.name, damage),
    //     Err(err_message) => println!("Mug failed: {}", err_message),
    // }    

    /*
    
    
    
    */
    // Enums
    let poison_result = apply_poison(&mut enemy_1, &player_3);
    match poison_result {
        PoisonResult::Applied(dmg) => println!("Poison applied! Also did {} damage!", dmg),
        PoisonResult::HitNotApplied(dmg) => println!("Failed to apply the poison, but stabbed the enemy for {} damage!", dmg),
        PoisonResult::Missed => println!("You tried to poison the enemy, but missed!"),
        PoisonResult::Dodged => println!("You tried to poison the enemy, but they dodged!")
    }
}

fn hurt_player(player: &mut Character, damage: u16) {
    player.health -= damage;
}

fn mug_enemy(enemy: &mut Enemy, attacker: &Character) -> Result<u16, String> {
    let attack_result = attack_enemy(enemy, attacker)?;
    let steal_roll = rand::random::<u16>();
    if steal_roll % 4 == 0 {
        let damage = attack_result + attacker.strength;
        // Fire a "Stole some item!" event to some other handler
        return Ok(damage);
    } else {
        return Err("THEFT BLOCKED".to_string());
    }
}

fn attack_enemy(enemy: &mut Enemy, attacker: &Character) -> Result<u16, String> {
    let hit_roll = rand::random::<u16>();
    if hit_roll % 2 == 0 {
        let damage = attacker.strength;
        enemy.health -= damage;
        return Ok(damage);
    } else {
        return Err("MISS".to_string());
    }
}

fn apply_poison(enemy: &mut Enemy, attacker: &Character) -> PoisonResult {
    let mut rng = rand::thread_rng();
    return match rng.gen_range(0..4) {
        0 => PoisonResult::Applied(attacker.strength), // <-- if we had a real game engine, we could apply poison to the enemy here, too
        1 => PoisonResult::HitNotApplied(attacker.strength),
        2 => PoisonResult::Missed,
        3 => PoisonResult::Dodged,
        _ => panic!("oh no!")
    }
}