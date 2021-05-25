//! The player module manages players.

use crate::ui;

struct Player {
    player_name: String,
    money: u64,
    population: u64,
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player{player_name, money: 0, population: 3000}
    }
}

pub struct PlayerMan {
    players: Vec<Player>,
}

impl PlayerMan {
    pub fn new() -> PlayerMan {
        PlayerMan{players: Vec::<Player>::new()}
    }

    pub fn new_player(&mut self) {
        println!("Player's name: ");
        let mut player_name : String = ui::input_string(2, 20);
        let mut repeated = true;
        while repeated {
            repeated = false;
            for p in &self.players {
                if p.player_name == player_name {
                    println!("Player name {} is repeated. Please, be more original.", player_name);
                    println!("Player's name: ");
                    player_name = ui::input_string(2, 20);
                    repeated = true;
                }
            }
        }
        self.players.push(Player::new(player_name));
    }

    pub fn print(&self) {
        for p in &self.players {
            println!("player_name: {}, money: {}, population: {}", p.player_name, p.money, p.population);
        }
    }
}
