use super::*;

pub fn register() -> Vec<(&'static str, fn() -> Card)> {
    vec![
        //("Astrolabe", Card::astrolabe),
        ("Bazaar", Card::bazaar),
        //("Blockade", Card::blockade),
        //("Caravan", Card::caravan),
        //("Corsair", Card::corsair),
        //("Cutpurse", Card::cutpurse),
        //("Fishing Village", Card::fishing_village),
        //("Haven", Card::haven),
        //("Island", Card::island),
        //("Lighthouse", Card::lighthouse),
        //("Lookout", Card::lookout),
        //("Merchant Ship", Card::merchant_ship),
        //("Monkey", Card::monkey),
        //("Native Village", Card::native_village),
        //("Outpost", Card::outpost),
        //("Pirate", Card::pirate),
        //("Sailor", Card::sailor),
        //("Salvager", Card::salvager),
        //("Sea Chart", Card::sea_chart),
        //("Sea Witch", Card::sea_witch),
        //("Smugglers", Card::smugglers),
        //("Tactician", Card::tactician),
        //("Tide Pools", Card::tide_pools),
        //("Treasure Map", Card::treasure_map),
        //("Treasury", Card::treasury),
        ("Warehouse", Card::warehouse),
        //("Wharf", Card::wharf),
    ]
}


impl Card {

    pub fn bazaar() -> Card {
        Card {
            name: "Bazaar".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                    NodeTemplate::PlusAction(RuntimeI32::Const(2)),
                    NodeTemplate::PlusCoin(RuntimeI32::Const(1)),

                ])
            ],
            on_gain: None,
        }
    }

    pub fn warehouse() -> Card {
        Card {
            name: "Warehouse".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(3)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::DiscardCard(RuntimeI32::Const(3)),
            ])],
            on_gain: None,
        }
    }
}
