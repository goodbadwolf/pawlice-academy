mod bears;

fn main() {
    let num_players = 3;
    let game_state = bears::system::GameState::from(num_players);
    println!("Game state: {:?}", game_state);
}
