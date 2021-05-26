//! The player module manages players.

use crate::ui;

struct Attack {
    attacking_player: usize,
    attacked_player: usize,
    attacking_soldiers: u32,
}

struct Player {
    player_name: String,
    money: u32,
    population: u32,
    soldiers: u32,
    option: u32,
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player{player_name, money: 0, population: 3, soldiers: 0, option: 1}
    }
}

pub struct PlayerMan {
    players: Vec<Player>,
    pending_attacks: Vec<Attack>,
}

impl PlayerMan {
    pub fn new() -> PlayerMan {
        PlayerMan{players: Vec::<Player>::new(), pending_attacks: Vec::<Attack>::new()}
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
            if p.option != 0 {
                println!("player_name: {}", p.player_name);
                println!("money: {}, population: {}, soldiers: {}",
                         p.money, p.population, p.soldiers);
            }
        }
    }

    pub fn select_turn(&mut self) {
        let players_len = self.players.len();
        for i in 0..players_len {
            if self.players[i].option != 0 {
                println!("Player: {}", self.players[i].player_name);
                if self.players[i].money > 0 && self.players[i].soldiers > 0 {
                    println!("Options: 1) Trade 2) Housing 3) Soldiers 4) Attack 0) Surrender");
                    self.players[i].option = ui::input_u32(0, 4);
                } else if self.players[i].money > 0 {
                    println!("Options: 1) Trade 2) Housing 3) Soldiers 0) Surrender");
                    self.players[i].option = ui::input_u32(0, 3);
                } else {
                    println!("Options: 1) Trade 2) Housing 0) Surrender");
                    self.players[i].option = ui::input_u32(0, 2);
                }

                if self.players[i].option == 4 {
                    let mut attacked_player;
                    // If there's only two players, just attack the other player
                    if players_len == 2 {
                        if i == 0 {
                            attacked_player = 1;
                        } else {
                            attacked_player = 0;
                        }
                    } else {
                        println!("Which player do you wish to attack?");
                        for j in 0..players_len {
                            if !self.players[j].player_name.eq(&self.players[i].player_name) {
                                println!("{} {}", j, self.players[j].player_name);
                            }
                        }

                        attacked_player = ui::input_u32(0, self.players.len() as u32) as usize;
                        while attacked_player == i {
                            println!("You can't attack yourself");
                            attacked_player = ui::input_u32(0, self.players.len() as u32) as usize;
                        }
                    }

                    println!("How many soldiers do you want to send for the attack ({} available)?", self.players[i].soldiers);
                    let attacking_soldiers = ui::input_u32(1, self.players[i].soldiers);
                    self.players[i].soldiers -= attacking_soldiers;
                    self.pending_attacks.push(Attack{attacking_player: i, attacked_player, attacking_soldiers});
                }
            }

        }
    }

    pub fn execute_turn(&mut self) {
        for p in &mut self.players {
            if p.option == 1 {
                p.money += p.population;
            } else if p.option == 2 {
                p.population += 1;
            } else if p.option == 3 {
                p.money -= 1;
                p.soldiers += 1;
            } else if p.option == 4 {
                p.money -= 1;
            }
        }

        for attack in &self.pending_attacks {
            if self.players[attack.attacked_player].soldiers == 0 {
                if attack.attacking_soldiers > self.players[attack.attacked_player].population {
                    self.players[attack.attacked_player].population = 0;
                } else {
                    self.players[attack.attacked_player].population -= attack.attacking_soldiers;
                }
                self.players[attack.attacking_player].soldiers += attack.attacking_soldiers;
            } else if attack.attacking_soldiers > self.players[attack.attacked_player].soldiers {
                self.players[attack.attacked_player].soldiers = 0;
                self.players[attack.attacking_player].soldiers += attack.attacking_soldiers - 1;
            } else {
                self.players[attack.attacked_player].soldiers -= 1;
            }
        }
        self.pending_attacks.clear();

        for p in &mut self.players {
            if p.population == 0 {
                p.option = 0;
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
