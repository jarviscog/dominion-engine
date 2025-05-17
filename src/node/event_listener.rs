use crate::card::RuntimeValue;

use super::{EventListenerDestructCondition, EventListenerFireCondition, Node, NodeTemplate};

/// Any event that does not happen right as a card is played.
#[derive(Debug, Clone)]
pub struct EventListener {
    pub played_by: RuntimeValue,
    pub fire_condition: EventListenerFireCondition, // When to fire the EventListener
    pub destruct_condition: EventListenerDestructCondition, // When to destuct the EventListener
    pub steps: Vec<NodeTemplate>,                       // Steps to take when the EventListener fires
    pub destruct_on_fire: bool,
}

impl EventListener {
    pub fn new(
        played_by: RuntimeValue,
        fire_condition: EventListenerFireCondition,
        destruct_condition: EventListenerDestructCondition,
        steps: Vec<NodeTemplate>,
        destruct_on_fire: bool,
    ) -> EventListener {
        EventListener {
            played_by,
            fire_condition,
            destruct_condition,
            steps,
            destruct_on_fire,
        }
    }
}
