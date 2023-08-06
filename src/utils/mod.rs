use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck<Card> {
    pub cards: Vec<Card>,
}

#[allow(dead_code)]
impl<Card> Deck<Card> {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn place(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_from(&mut self, index: usize) -> Option<Card> {
        match self.len() {
            0 => None,
            _ => Some(self.cards.remove(index)),
        }
    }

    pub fn truncate(&mut self, len: usize) {
        self.cards.truncate(len);
    }
}
