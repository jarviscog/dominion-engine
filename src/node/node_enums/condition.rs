use super::*;
use crate::RuntimeValue;

/// Condition used for `Step::RepeatUntil`
#[derive(Debug, Clone)]
pub enum Condition {
    /// Repeat for a fixed number of times, calculated at runtime
    NumberOfTimes(RuntimeValue),
    /// Check if a stack contains a certain number of cards
    StackSize(Location, RuntimeValue),
}
