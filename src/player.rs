//! The player module manages players.

use crate::ui;

struct Player {
    player_name: String,
    money: u64,
    population: u64,
    option: u32,
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player{player_name, money: 0, population: 3, option: 1}
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

    pub fn select_turn(&mut self) {
        for p in &mut self.players {
            if p.option != 0 {
                println!("Player: {}", p.player_name);
                println!("Options: 1) Trade 2) Housing 0) Surrender");
                p.option = ui::input_u32(0, 2);
            }
        }
    }

    pub fn execute_turn(&mut self) {
        for p in &mut self.players {
            if p.option == 1 {
                p.money += p.population;
            } else if p.option == 2 {
                p.population += 1;
            }
        }
    }

    pub fn living_players(&self) -> u8 {
        let mut n_living_players = 0;
        for p in &self.players {
            if p.option != 0 {
                n_living_players += 1;
            }
        }
        n_living_players
    }

    pub fn print_winner(&self) {
        for p in &self.players {
            if p.option != 0 {
                println!("The winner is {}", p.player_name);
            }
        }
    }
}
