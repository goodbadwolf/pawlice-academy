use enum_display::EnumDisplay;

#[derive(Debug, Clone, Copy, EnumDisplay)]
pub enum IngredientKind {
    Fish,
    Berries,
    Wheat,
    Rice,
    Raindrop,
    Any,
}

#[derive(Debug, Clone, Copy, EnumDisplay)]
pub enum MaterialKind {
    Snowball,
    Cotton,
    Bamboo,
    Rope,
    Leaf,
    Silk,
    Pebble,
    Any,
}

#[derive(Debug, Clone, Copy, EnumDisplay)]
pub enum FortuneKind {
    BeeAttack,
    Picnic,
    Avalanche,
    Wildfire,
    Hibernation,
    BearHug,
    Bearglar,
    Famine,
}

#[derive(Debug, Clone, Copy, EnumDisplay)]
pub enum Resource {
    Ingredient(IngredientKind),
    Material(MaterialKind),
    Fortune(FortuneKind),
}

#[derive(Debug, Clone, Copy, EnumDisplay)]
pub enum BearCard {
    PolarBear,
    Panda,
    SlothBear,
    SpectacledBear,
    GrizzlyBear,
    SunBear,
}

#[derive(Debug, Clone)]
pub struct QuestCard {
    pub name: String,
    pub cost: Vec<Resource>,
    pub reward: usize,
}

pub struct QuestTile<'a> {
    pub quest: &'a QuestCard,
}

pub fn get_all_quests() -> Vec<QuestCard> {
    vec![
        QuestCard {
            name: "Tuna Sandwich".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Fish),
                Resource::Ingredient(IngredientKind::Wheat),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Treehouse".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Cotton),
                Resource::Material(MaterialKind::Leaf),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tiny Home".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Silk),
                Resource::Material(MaterialKind::Pebble),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tent".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Cotton),
                Resource::Material(MaterialKind::Leaf),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tea".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Raindrop),
                Resource::Ingredient(IngredientKind::Raindrop),
                Resource::Material(MaterialKind::Leaf),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Sweater".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Cotton),
                Resource::Material(MaterialKind::Silk),
                Resource::Material(MaterialKind::Any),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Sushi".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Fish),
                Resource::Ingredient(IngredientKind::Rice),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Soup".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Raindrop),
                Resource::Ingredient(IngredientKind::Any),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
        QuestCard {
            name: "Snow Cone".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Snowball),
                Resource::Material(MaterialKind::Snowball),
                Resource::Ingredient(IngredientKind::Berries),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Ramen".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Raindrop),
                Resource::Ingredient(IngredientKind::Wheat),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Map".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Rice),
                Resource::Material(MaterialKind::Leaf),
                Resource::Material(MaterialKind::Pebble),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Lean-To".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Rope),
                Resource::Material(MaterialKind::Pebble),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Igloo".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Snowball),
                Resource::Material(MaterialKind::Snowball),
                Resource::Material(MaterialKind::Cotton),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "House Boat".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Silk),
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Leaf),
                Resource::Material(MaterialKind::Any),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 5,
        },
        QuestCard {
            name: "Hair Brush".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Rope),
                Resource::Material(MaterialKind::Bamboo),
                Resource::Material(MaterialKind::Any),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Fruit Salad".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Berries),
                Resource::Ingredient(IngredientKind::Berries),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Fishing Rod".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Rope),
                Resource::Material(MaterialKind::Bamboo),
                Resource::Ingredient(IngredientKind::Any),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Curry".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Rice),
                Resource::Ingredient(IngredientKind::Any),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
        QuestCard {
            name: "Cave Hotel".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Pebble),
                Resource::Material(MaterialKind::Silk),
                Resource::Material(MaterialKind::Leaf),
                Resource::Material(MaterialKind::Any),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 5,
        },
        QuestCard {
            name: "Castle".to_string(),
            cost: vec![
                Resource::Material(MaterialKind::Pebble),
                Resource::Material(MaterialKind::Silk),
                Resource::Material(MaterialKind::Rope),
                Resource::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Beary Pie".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Berries),
                Resource::Ingredient(IngredientKind::Wheat),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Bao".to_string(),
            cost: vec![
                Resource::Ingredient(IngredientKind::Wheat),
                Resource::Ingredient(IngredientKind::Any),
                Resource::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
    ]
}
