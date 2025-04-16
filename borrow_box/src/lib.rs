#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            (self.p1.0.clone(), self.p1.1)
        } else if self.p2.1 > self.p1.1 {
            (self.p2.0.clone(), self.p2.1)
        } else {
            ("Same score! tied".to_string(), self.p1.1)
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        let max_score = self.nb_games / 2 + 1;

        if self.p1.1 >= max_score || self.p2.1 >= max_score {
            return;
        }

        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }
    
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        assert_eq!(game.id, 0);
        assert_eq!(game.p1.0, "Joao");
        assert_eq!(game.p1.1, 0);
        assert_eq!(game.p2.0, "Susana");
        assert_eq!(game.p2.1, 0);
        assert_eq!(game.nb_games, 5);
    }

    #[test]
    fn test_read_winner() {
        let game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        let winner = game.read_winner();
        assert_eq!(winner.0, "Same score! tied");
        assert_eq!(winner.1, 0);
    }

    #[test]
    fn test_update_score() {
        let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        game.update_score(String::from("Joao"));
        assert_eq!(game.p1.1, 1);
        game.update_score(String::from("Susana"));
        assert_eq!(game.p2.1, 1);    
    }

    #[test]
    fn test_delete() {
        let game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        assert_eq!(game.delete(), "game deleted: id -> 0");
    }
}
