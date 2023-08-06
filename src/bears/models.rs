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
pub enum ResourceCard {
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
    pub cost: Vec<ResourceCard>,
    pub reward: usize,
}

pub struct QuestTile<'a> {
    pub quest: &'a QuestCard,
}

pub fn build_all_quests() -> Vec<QuestCard> {
    vec![
        QuestCard {
            name: "Tuna Sandwich".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Fish),
                ResourceCard::Ingredient(IngredientKind::Wheat),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Treehouse".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Cotton),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tiny Home".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Silk),
                ResourceCard::Material(MaterialKind::Pebble),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tent".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Cotton),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Tea".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Raindrop),
                ResourceCard::Ingredient(IngredientKind::Raindrop),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Sweater".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Cotton),
                ResourceCard::Material(MaterialKind::Silk),
                ResourceCard::Material(MaterialKind::Any),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Sushi".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Fish),
                ResourceCard::Ingredient(IngredientKind::Rice),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Soup".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Raindrop),
                ResourceCard::Ingredient(IngredientKind::Any),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
        QuestCard {
            name: "Snow Cone".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Snowball),
                ResourceCard::Material(MaterialKind::Snowball),
                ResourceCard::Ingredient(IngredientKind::Berries),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Ramen".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Raindrop),
                ResourceCard::Ingredient(IngredientKind::Wheat),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Map".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Rice),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Material(MaterialKind::Pebble),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Lean-To".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Rope),
                ResourceCard::Material(MaterialKind::Pebble),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Igloo".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Snowball),
                ResourceCard::Material(MaterialKind::Snowball),
                ResourceCard::Material(MaterialKind::Cotton),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "House Boat".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Silk),
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Material(MaterialKind::Any),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 5,
        },
        QuestCard {
            name: "Hair Brush".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Rope),
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Material(MaterialKind::Any),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Fruit Salad".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Berries),
                ResourceCard::Ingredient(IngredientKind::Berries),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Fishing Rod".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Rope),
                ResourceCard::Material(MaterialKind::Bamboo),
                ResourceCard::Ingredient(IngredientKind::Any),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Curry".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Rice),
                ResourceCard::Ingredient(IngredientKind::Any),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
        QuestCard {
            name: "Cave Hotel".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Pebble),
                ResourceCard::Material(MaterialKind::Silk),
                ResourceCard::Material(MaterialKind::Leaf),
                ResourceCard::Material(MaterialKind::Any),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 5,
        },
        QuestCard {
            name: "Castle".to_string(),
            cost: vec![
                ResourceCard::Material(MaterialKind::Pebble),
                ResourceCard::Material(MaterialKind::Silk),
                ResourceCard::Material(MaterialKind::Rope),
                ResourceCard::Material(MaterialKind::Any),
            ],
            reward: 4,
        },
        QuestCard {
            name: "Beary Pie".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Berries),
                ResourceCard::Ingredient(IngredientKind::Wheat),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 3,
        },
        QuestCard {
            name: "Bao".to_string(),
            cost: vec![
                ResourceCard::Ingredient(IngredientKind::Wheat),
                ResourceCard::Ingredient(IngredientKind::Any),
                ResourceCard::Ingredient(IngredientKind::Any),
            ],
            reward: 2,
        },
    ]
}

pub fn build_all_resources() -> Vec<ResourceCard> {
    let mut resources = Vec::new();
    let mut insert_resource_cards = |resource: ResourceCard, count: u8| {
        for _ in 0..count {
            resources.push(resource);
        }
    };
    // Ingredients
    insert_resource_cards(ResourceCard::Ingredient(IngredientKind::Fish), 6);
    insert_resource_cards(ResourceCard::Ingredient(IngredientKind::Berries), 8);
    insert_resource_cards(ResourceCard::Ingredient(IngredientKind::Wheat), 8);
    insert_resource_cards(ResourceCard::Ingredient(IngredientKind::Rice), 6);
    insert_resource_cards(ResourceCard::Ingredient(IngredientKind::Raindrop), 8);
    // Materials
    insert_resource_cards(ResourceCard::Material(MaterialKind::Snowball), 8);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Cotton), 8);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Bamboo), 11);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Rope), 9);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Leaf), 9);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Silk), 8);
    insert_resource_cards(ResourceCard::Material(MaterialKind::Pebble), 11);
    // Fortunes
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::BeeAttack), 4);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Picnic), 4);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Avalanche), 4);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Wildfire), 3);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Hibernation), 3);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::BearHug), 3);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Bearglar), 2);
    insert_resource_cards(ResourceCard::Fortune(FortuneKind::Famine), 2);

    resources
}
