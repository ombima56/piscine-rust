use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat = self.weight_in_kg * self.fat_content;
        let protein = self.weight_in_kg * (1.0 - self.fat_content);
        fat * 9.0 + protein * 4.0
    }
}

impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives();
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        writeln!(f, "Weapons: {:?}", self.weapons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_eat_fruit() {
        let mut player = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife")],
        };
        let apple = Fruit { weight_in_kg: 1.0 };
        player.eat(apple);
        assert_eq!(player.strength, 5.0);
    }

    #[test]
    fn test_player_eat_meat() {
        let mut player = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife")],
        };
        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 0.2,
        };
        player.eat(steak);
        assert_eq!(player.strength, 5.4);
    }
}
