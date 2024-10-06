use std::collections::HashMap;

struct Bank {
    // TODO this should contain a list of treasure, victory points, and actions, and have funtions
    // for returning 
    
    action: HashMap<Card, u32>

}

impl Bank {

    pub fn new() -> Self {
        Bank { 
            action: HashMap::new(),
        }
    }

    pub fn base_game() -> Bank {

    }

}
