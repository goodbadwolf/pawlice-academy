use super::models::*;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Player {
    pub index: usize,
}

#[derive(Debug)]
pub struct GameState<'a> {
    pub players: Vec<Player>,
    pub all_quests: Vec<QuestCard>,
    pub open_quests: Vec<&'a QuestCard>,
}

impl GameState<'_> {
    fn create_players(num_players: usize) -> Vec<Player> {
        let mut players = Vec::new();
        for i in 0..num_players {
            players.push(Player { index: i });
        }
        players
    }

    fn create_quests(num_players: usize) -> Vec<QuestCard> {
        let mut quests = get_all_quests();
        quests.shuffle(&mut rand::thread_rng());
        let num_quests = match num_players {
            1..=3 => 12,
            4..=5 => 15,
            _ => quests.len(),
        };
        quests.truncate(num_quests);
        quests
    }

    pub fn from(num_players: usize) -> Self {
        Self {
            players: Self::create_players(num_players),
            all_quests: Self::create_quests(num_players),
            open_quests: Vec::new(),
        }
    }
}
