use super::*;

pub fn register() -> Vec<(&'static str, fn() -> Card)> {
    vec![
        //("Develop", Card::develop),
        //("Berserker", Card::berserker),
        //("Border Village", Card::border_village),
        //("Cartographer", Card::cartographer),
        //("Cauldron", Card::cauldron),
        //("Crossroads", Card::crossroads),
        //("Farmland", Card::farmland),
        //("Fool's Gold", Card::fools_gold),
        //("Guard Dog", Card::guard_dog),
        //("Haggler", Card::haggler),
        //("Highway", Card::highway),
        //("Inn", Card::inn),
        //("Jack of All Trades", Card::jack_of_all_trades),
        //("Margrave", Card::margrave),
        //("Nomads", Card::nomads),
        ("Oasis", Card::oasis),
        //("Scheme", Card::scheme),
        //("Souk", Card::souk),
        //("Spice Merchant", Card::spice_merchant),
        //("Stables", Card::stables),
        //("Trader", Card::trader),
        //("Trail", Card::trail),
        //("Tunnel", Card::tunnel),
        //("Weaver", Card::weaver),
        //("Wheelwright", Card::wheelwright),
        //("Witch's Hut", Card::witchs_hut),
    ]
}

impl Card {
    pub fn oasis() -> Card {
        Card {
            name: "Oasis".to_owned(),
            expansion: Expansion::Hinterlands,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(1)),
                NodeTemplate::DiscardCard(RuntimeI32::Const(1)),
            ])],
            cost: vec![Cost::Coin(6)],
            on_gain: None,
        }
    }
}
