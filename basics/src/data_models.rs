#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub health: u16,
    pub mana: u16,
    pub strength: u16,
    pub move_speed: u16,
}

impl Character {
    pub fn new() -> Self {
        Character{
            name: String::default(),
            health: 0,
            mana: 0,
            strength: 0,
            move_speed: 0
        }
    }
}

pub struct CharacterBuilder {
    pub name: Option<String>,
    pub health: Option<u16>,
    pub mana: Option<u16>,
    strength: Option<u16>,
    pub move_speed: Option<u16>,
}

impl CharacterBuilder {
    pub fn new() -> Self {
        CharacterBuilder {
            name: None,
            health: None,
            mana: None,
            strength: None,
            move_speed: None,
        }
    }

    pub fn name(mut self, name_val: &str) -> Self {
        self.name = Some(name_val.to_string());
        self
    }

    pub fn health(mut self, health_val: u16) -> Self {
        self.health = Some(health_val);
        self
    }    

    pub fn mana(mut self, mana_val: u16) -> Self {
        self.mana = Some(mana_val);
        self
    }

    pub fn strength(mut self, str_val: u16) -> Self {
        self.strength = Some(str_val);
        self
    }

    pub fn move_speed(mut self, move_speed_val: u16) -> Self {
        self.move_speed = Some(move_speed_val);
        self
    }

    pub fn build(&self) -> Character {
        Character {
            name: self.name.clone().unwrap_or_default(),
            health: self.health.unwrap_or_default(),
            mana: self.mana.unwrap_or_default(),
            strength: self.strength.unwrap_or_default(),
            move_speed: self.move_speed.unwrap_or_default(),
        }
    }
}

pub struct Enemy{
    pub name: String,
    pub health: u16,
}

pub enum PoisonResult {
    Applied(u16),
    HitNotApplied(u16),
    Missed,
    Dodged,
}