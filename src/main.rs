
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

fn print_options() {
    println!("Options: 1) Next turn 0) Exit");
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
    let mut exit_turncraft = false;
    while !exit_turncraft {

        print_options();
        let sel_option : usize = ui::input_u32(0, 1) as usize;
        match sel_option {
            1 => {
                print_turn(turn);
                playerman.print();
                turn += 1;
            }
            0 => exit_turncraft = true,
            _ => println!("ERROR invalid option {}", sel_option),
        }
    }

}
