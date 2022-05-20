use crate::machine::{self, Machine};

pub struct InMemory<T> {
    history: Vec<T>,
}

impl<T> InMemory<T> {
    pub fn new(state: T) -> Self {
        Self {
            history: vec![state],
        }
    }
}

impl<T: machine::Definition + Copy> Machine<T> for InMemory<T> {
    fn history(&self) -> &Vec<T> {
        &self.history
    }

    fn create_transition(&mut self, to_state: T) {
        self.history.push(to_state);
    }
}
