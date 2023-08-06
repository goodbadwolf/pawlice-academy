use super::models::*;
use crate::utils::Deck;
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Player {
    pub index: usize,
    pub bear: BearCard,
    pub hand: Vec<ResourceCard>,
    pub quest_tiles: Vec<QuestTile>,
    pub completed_quests: Vec<QuestCard>,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Uninitialized,
    PlayerReady { player_index: usize },
    Done { winner_indices: Vec<usize> },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum InitialPlayerSelectionStrategy {
    Random,
    First,
    Last,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum QuestTileSelectionStrategy {
    Random,
    Target { target: usize },
    Greedy,
}

#[derive(Debug)]
pub enum LoggedEvents {
    BuiltQuestDeck { quests: Deck<QuestCard> },
    BuiltResourceDeck { resources: Deck<ResourceCard> },
    SelectedInitialPlayer { player_index: usize },
}

#[derive(Debug)]
pub struct GameTable {
    pub players: Vec<Player>,
    pub quests: Deck<QuestCard>,
    pub quest_tiles: Deck<QuestTile>,
    pub resources: Deck<ResourceCard>,
    pub open_quests: Vec<QuestCard>,
    pub discard_pile: Deck<ResourceCard>,
    pub state: GameState,
    pub initial_player_selection_strategy: InitialPlayerSelectionStrategy,
    pub quest_tile_selection_strategy: QuestTileSelectionStrategy,
    pub events: Vec<LoggedEvents>,
}

impl GameTable {
    pub fn from(
        num_players: usize,
        initial_player_selection_strategy: InitialPlayerSelectionStrategy,
        quest_tile_selection_strategy: QuestTileSelectionStrategy,
    ) -> Option<Self> {
        if num_players < 2 || num_players > 6 {
            return None;
        }
        Some(Self {
            players: Self::create_players(num_players),
            quests: Deck::new(),
            quest_tiles: Deck::new(),
            resources: Deck::new(),
            open_quests: Vec::new(),
            discard_pile: Deck::new(),
            state: GameState::Uninitialized,
            initial_player_selection_strategy,
            quest_tile_selection_strategy,
            events: Vec::new(),
        })
    }

    pub fn play_one_step(&mut self) {
        match self.state {
            GameState::Uninitialized => {
                self.build_quest_deck();
                self.build_resource_deck();
                self.select_initial_player();
                self.distribute_resources();
                self.distribute_quest_tiles();
            }
            GameState::PlayerReady { player_index } => {
                self.state = GameState::Done {
                    winner_indices: vec![player_index],
                };
            }
            GameState::Done { .. } => {}
        }
    }

    fn add_event(&mut self, event: LoggedEvents) {
        self.events.push(event);
    }

    fn create_players(num_players: usize) -> Vec<Player> {
        let mut bears = vec![
            BearCard::Polar,
            BearCard::Panda,
            BearCard::Sloth,
            BearCard::Spectacled,
            BearCard::Grizzly,
            BearCard::Sun,
        ];
        bears.shuffle(&mut rand::thread_rng());
        bears
            .iter()
            .enumerate()
            .take(num_players)
            .map(|(i, b)| Player::new(i, *b))
            .collect()
    }

    fn build_quest_deck(&mut self) {
        let (quests, quest_tiles) = build_all_quests();
        self.quests = Deck::from(quests);
        self.quests.shuffle();
        let num_quests = match self.players.len() {
            1..=3 => 12,
            4..=5 => 15,
            _ => self.quests.len(),
        };
        self.quests.truncate(num_quests);
        self.quest_tiles = Deck::from(quest_tiles);
        self.add_event(LoggedEvents::BuiltQuestDeck {
            quests: self.quests.clone(),
        });
    }

    fn build_resource_deck(&mut self) {
        self.resources = Deck::from(build_all_resources());
        self.resources.shuffle();
        self.add_event(LoggedEvents::BuiltResourceDeck {
            resources: self.resources.clone(),
        });
    }

    fn distribute_resources(&mut self) {
        let initial_resource_count = 5;
        for _ in 0..initial_resource_count {
            for player in &mut self.players {
                match self.resources.draw() {
                    Some(resource) => player.add_to_hand(resource),
                    None => {
                        panic!("Ran out of resources while distributing. This should never happen.")
                    }
                }
            }
        }
    }

    fn distribute_quest_tiles(&mut self) {
        let tiles_per_player = 3;
        match self.quest_tile_selection_strategy {
            QuestTileSelectionStrategy::Random => {
                for _ in 0..tiles_per_player {
                    for player in &mut self.players {
                        let quest_tile_index =
                            rand::thread_rng().gen_range(0..self.quest_tiles.len());
                        let quest_tile = self.quest_tiles.draw_from(quest_tile_index);
                        match quest_tile {
                            Some(quest_tile) => player.add_quest_tile(quest_tile),
                            None => {
                                panic!("Ran out of quest tiles while distributing. This should never happen.")
                            }
                        }
                    }
                }
            }
            QuestTileSelectionStrategy::Greedy => {
                todo!("Greedy quest tile selection not implemented yet")
            }

            QuestTileSelectionStrategy::Target { target: _ } => {
                todo!("Target quest tile selection not implemented yet")
            }
        }
    }

    fn select_initial_player(&mut self) {
        let mut player_index = match self.initial_player_selection_strategy {
            InitialPlayerSelectionStrategy::Random => {
                rand::thread_rng().gen_range(0..self.players.len())
            }
            InitialPlayerSelectionStrategy::First => 0,
            InitialPlayerSelectionStrategy::Last => self.players.len() - 1,
        };
        self.players.rotate_left(player_index);
        player_index = self.players[0].index;
        self.state = GameState::PlayerReady { player_index };
        self.add_event(LoggedEvents::SelectedInitialPlayer { player_index });
    }
}

impl Player {
    pub fn new(index: usize, bear: BearCard) -> Self {
        Self {
            index,
            bear,
            hand: Vec::new(),
            quest_tiles: Vec::new(),
            completed_quests: Vec::new(),
        }
    }

    pub fn add_to_hand(&mut self, resource: ResourceCard) {
        self.hand.push(resource);
    }

    pub fn add_quest_tile(&mut self, tile: QuestTile) {
        self.quest_tiles.push(tile);
    }
}
