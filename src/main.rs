
use turncraft::player::PlayerMan;
use turncraft::ui;

fn print_separator() {
    println!("--------------------------------");
}

fn print_title() {
    print_separator();
    println!("-----------TurnCraft------------");
    println!("------------The game------------");
    print_separator();
}

fn print_turn(turn: usize) {
    println!("-----------Turn {}------------", turn);
}

fn main() {
    print_title();
    println!("Number of players (2 to 8): ");
    let num_players : usize = ui::input_u32(2, 8) as usize;
    print_separator();

    let mut playerman : PlayerMan = PlayerMan::new();
    for n in 0..num_players {
        println!("-------Player {}", n);
        playerman.new_player();
    }
    print_separator();

    let mut turn : usize = 0;
    while playerman.living_players() >= 2 {

        print_turn(turn);
        playerman.print();
        playerman.select_turn();
        playerman.execute_turn();
        turn += 1;
    }
    playerman.print_winner();
}
