use crate::machine::{self, Machine};

pub struct InMemory<T> {
    state: T,
}

impl<T> InMemory<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T: machine::Definition + Copy> Machine<T> for InMemory<T> {
    fn current_state(&self) -> T {
        self.state
    }

    fn create_transition(&mut self, to_state: T) {
        self.state = to_state;
    }
}
