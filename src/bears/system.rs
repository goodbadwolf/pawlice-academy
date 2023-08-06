use super::models::*;
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Player {
    pub index: usize,
    pub hand: Vec<ResourceCard>,
    pub quest_tiles: Vec<QuestCard>,
    pub completed_quests: Vec<QuestCard>,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Uninitialized,
    PlayerReady { player_index: usize },
    Done { winner_indices: Vec<usize> },
}

#[derive(Debug)]
pub enum InitialPlayerSelectionStrategy {
    Random,
    First,
    Last,
}

#[derive(Debug)]
pub enum QuestTileSelectionStrategy {
    Random,
    Target { target: usize },
    Greedy,
}

#[derive(Debug)]
pub enum LoggedEvents {
    BuiltQuestDeck { quests: Vec<QuestCard> },
    BuiltResourceDeck { resources: Vec<ResourceCard> },
    SelectedInitialPlayer { player_index: usize },
}

#[derive(Debug)]
pub struct GameTable {
    pub players: Vec<Player>,
    pub quest_deck: Vec<QuestCard>,
    pub quest_tiles: Vec<QuestCard>,
    pub resource_deck: Vec<ResourceCard>,
    pub open_quests: Vec<QuestCard>,
    pub discard_pile: Vec<ResourceCard>,
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
    ) -> Self {
        Self {
            players: Self::create_players(num_players),
            quest_deck: Vec::new(),
            quest_tiles: Vec::new(),
            resource_deck: Vec::new(),
            open_quests: Vec::new(),
            discard_pile: Vec::new(),
            state: GameState::Uninitialized,
            initial_player_selection_strategy,
            quest_tile_selection_strategy,
            events: Vec::new(),
        }
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
        let mut players = Vec::new();
        for i in 0..num_players {
            players.push(Player::new(i));
        }
        players
    }

    fn build_quest_deck(&mut self) {
        let mut quests = build_all_quests();
        quests.shuffle(&mut rand::thread_rng());
        let num_quests = match self.players.len() {
            1..=3 => 12,
            4..=5 => 15,
            _ => quests.len(),
        };
        quests.truncate(num_quests);
        self.quest_deck = quests;
        self.quest_tiles = self.quest_deck.clone();
        self.add_event(LoggedEvents::BuiltQuestDeck {
            quests: self.quest_deck.clone(),
        });
    }

    fn build_resource_deck(&mut self) {
        let mut resources = build_all_resources();
        resources.shuffle(&mut rand::thread_rng());
        self.resource_deck = resources;
        self.add_event(LoggedEvents::BuiltResourceDeck {
            resources: self.resource_deck.clone(),
        });
    }

    fn distribute_resources(&mut self) {
        let initial_resource_count = 5;
        for _ in 0..initial_resource_count {
            for player in &mut self.players {
                match self.resource_deck.pop() {
                    Some(resource) => player.add_to_hand(resource),
                    None => {
                        panic!("Ran out of resources while distributing. This should never happen.")
                    }
                }
            }
        }
    }

    fn distribute_quest_tiles(&mut self) {
        let quest_tile_count = 3;
        match self.quest_tile_selection_strategy {
            QuestTileSelectionStrategy::Random => {
                for _ in 0..quest_tile_count {
                    for player in &mut self.players {
                        let quest_tile_index =
                            rand::thread_rng().gen_range(0..self.quest_tiles.len());
                        let quest_tile = self.quest_tiles.remove(quest_tile_index);
                        player.add_quest_tile(quest_tile);
                    }
                }
            }
            QuestTileSelectionStrategy::Greedy => {
                panic!("Greedy quest tile selection not implemented yet")
            }

            QuestTileSelectionStrategy::Target { target } => {
                panic!("Target quest tile selection not implemented yet")
            } /*
                  QuestTileSelectionStrategy::Target { target } => {
                      for _ in 0..quest_tile_count {
                          for player in &mut self.players {
                              let current_tile_total = player.quest_tiles.iter().fold(0u32, |acc, q| acc + q.reward);
                              if current_tile_total < target {
                                  let mut best_quest_index = 0;
                                  let mut best_quest_reward = 0;
                                  for (i, quest) in self.quest_tiles.iter().enumerate() {
                                      if quest.reward > best_quest_reward {
                                          best_quest_index = i;
                                          best_quest_reward = quest.reward;
                                      }
                                  }
                                  match self.quest_deck.remove(best_quest_index) {
                                      Some(quest) => player.add_quest_tile(quest),
                                      None => {
                                          panic!("Ran out of quest tiles while distributing. This should never happen.")
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
              let num_quest_tiles = match self.players.len() {
                  1..=3 => 4,
                  4..=5 => 5,
                  _ => 6,
              };
              for _ in 0..num_quest_tiles {
                  match self.quest_deck.pop() {
                      Some(quest) => self.open_quests.push(quest),
                      None => {
                          panic!("Ran out of quests while distributing. This should never happen.")
                      }
                  }
              }
              self.open_quests.sort_by(|a, b| a.reward.cmp(&b.reward)
              */
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
    pub fn new(index: usize) -> Self {
        Self {
            index,
            hand: Vec::new(),
            quest_tiles: Vec::new(),
            completed_quests: Vec::new(),
        }
    }

    pub fn add_to_hand(&mut self, resource: ResourceCard) {
        self.hand.push(resource);
    }

    pub fn add_quest_tile(&mut self, quest: QuestCard) {
        self.quest_tiles.push(quest);
    }
}
