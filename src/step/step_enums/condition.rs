
use super::*;

#[derive(Debug, Clone)]
pub enum Condition {
    StackSize(Location, RuntimeValue) // Check if a stack contains a certain number of cards
}
