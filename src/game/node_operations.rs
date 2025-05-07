use super::*;

impl Game {
    /// Allow a player to buy a card
    /// The player must have the funds, and it must be in the bank
    /// `player_index` The index of the player that would like to buy the card
    /// `card_to_buy`
    /// `buy_anyways` If the card is not in the supply, or the player does not have enough funds,
    /// this allows them to gain the card anyways
    pub fn buy_card(&mut self, player_index: usize, card_to_buy: &Card, buy_anyways: bool) -> Result<(), String> {

        // Check the current player has sufficcient funds
        let mut can_afford = true;
        if card_to_buy.get_coin_cost() > self.players.get(self.current_player_index).unwrap().get_coins() {
            can_afford = false;
        }
        if !can_afford && !buy_anyways {
            return Err("Can't afford card".to_owned());
        }

        // If they do, move_card()
        match self.transfer_card(
            player_index, 
            card_to_buy, 
            Location::Supply, 
            Location::Discard, 
            false) {
            Ok(v) => {
                // Subtract the amount
                let mut_player = self.players.get_mut(self.current_player_index).unwrap();
                mut_player.sub_coins(card_to_buy.get_coin_cost());
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    pub fn transfer_card(
        &mut self,
        player_index: usize,
        card_to_move: &Card,
        from: Location,
        to: Location,
        move_anyways: bool,
    ) -> Result<(), String> {
        // TODO this function needs an overhaul
        let mut player = self.players.get_mut(player_index);
        println!("Moving card... from {} to {}", from, to);

        // Get the card from the location
        let r: Option<Card> = match from {
            Location::Hand => { todo!() }
            Location::Discard => { todo!() }
            Location::DeckTop => { todo!() }
            Location::Supply=> {
                self.bank.take_card(card_to_move)
            }
            Location::InPlay=> { todo!() }
            Location::InternalBuffer=> { todo!() }
            Location::Trash=> { todo!() }
            Location::SearchDeck => {return Err("Cannot move card from SearchDeck".to_owned())}


        };

        Ok(())
    }
}
