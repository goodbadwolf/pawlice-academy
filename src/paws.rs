use std::process::ExitCode;

mod bears;
mod utils;

fn main() -> ExitCode {
    let num_players = 5;
    let mut game_table = match bears::system::GameTable::from(
        num_players,
        bears::system::InitialPlayerSelectionStrategy::Random,
        bears::system::QuestTileSelectionStrategy::Random,
    ) {
        Some(game_table) => game_table,
        None => {
            println!("Invalid number of players: {}", num_players);
            return ExitCode::FAILURE;
        }
    };
    loop {
        match game_table.state {
            bears::system::GameState::Done { winner_indices } => {
                println!(
                    "Done, Winner: {:?}",
                    game_table
                        .players
                        .iter()
                        .find(|p| winner_indices.contains(&p.index))
                        .unwrap()
                );
                break;
            }
            _ => {
                println!(
                    "State: {:?}, Players: {:?}",
                    game_table.state, game_table.players
                );
                game_table.play_one_step();
            }
        }
    }
    println!("---------Events---------");
    for event in game_table.events.iter() {
        println!("{:?}", event);
    }
    ExitCode::SUCCESS
}
