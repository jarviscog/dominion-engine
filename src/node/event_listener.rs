use crate::card::RuntimeValue;

use super::{EventListenerDestructCondition, EventListenerFireCondition, Node, NodeType};

/// Any event that does not happen right as a card is played.
#[derive(Debug, Clone)]
pub struct EventListener {
    played_by: RuntimeValue,
    fire_condition: EventListenerFireCondition, // When to fire the EventListener
    destruct_condition: EventListenerDestructCondition, // When to destuct the EventListener
    steps: Vec<StepNodeType>, // Steps to take when the EventListener fires
    destruct_on_fire: bool,
}

impl EventListener {
    pub fn new(
        played_by: RuntimeValue, 
        fire_condition: EventListenerFireCondition,
        destruct_condition: EventListenerDestructCondition,
        steps: Vec<StepNodeType>,
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
